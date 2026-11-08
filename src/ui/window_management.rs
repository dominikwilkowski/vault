use core::cell::RefCell;

use floem::{
	event::{listener, EventPropagation},
	kurbo::Size,
	reactive::Context,
	ui_events::keyboard::{Code, Modifiers},
	views::Decorators,
	window::{close_window, new_window, WindowConfig, WindowId},
	IntoView,
};

use crate::{
	db::DbFields,
	ui::{primitives::styles, shared_context},
};

thread_local! {
	/// Windows opened through [`opening_window`] by their id. The window id is
	/// `None` while floem is still creating the window.
	pub(crate) static OPEN_WINDOWS: RefCell<Vec<(String, Option<WindowId>)>> = const { RefCell::new(Vec::new()) };
}

/// Provide a value both as floem context for the current view tree and in the
/// app-wide [`shared_context`] registry, so it can also be fetched from event
/// handlers, timers and other windows.
pub fn provide_shared_context<T: Clone + 'static>(value: T) {
	shared_context::provide(value.clone());
	Context::provide(value);
}

pub fn make_field_path(id: usize, field: &DbFields) -> String {
	format!("{}-{}", id, field)
}

pub fn closing_window(id: String, callback: impl Fn()) {
	OPEN_WINDOWS.with(|all_windows| {
		let mut open_windows = all_windows.borrow_mut();

		if let Some(pos) = open_windows.iter().position(|item| item.0 == id) {
			let (_, window_id) = open_windows.remove(pos);

			if let Some(window_id) = window_id {
				close_window(window_id);
			}
			callback();
		}
	});
}

pub fn close_all_windows() {
	OPEN_WINDOWS.with(|all_windows| {
		let mut open_windows = all_windows.borrow_mut();
		while let Some((_, window_id)) = open_windows.pop() {
			if let Some(window_id) = window_id {
				close_window(window_id);
			}
		}
	});
}

pub struct WindowSpec {
	pub id: String,
	pub title: String,
}

#[allow(clippy::redundant_closure)]
pub fn opening_window<V: IntoView + 'static>(
	view: impl Fn() -> V + 'static,
	spec: WindowSpec,
	size: Size,
	_movable_by_window_background: bool, // movable_by_window_background is too buggy to enable just yet
	on_close: impl Fn() + 'static,
) {
	OPEN_WINDOWS.with(|all_windows| {
		if !all_windows.borrow().iter().any(|item| item.0 == spec.id) {
			// register right away so a repeated request while floem is still
			// creating the window does not open it twice
			all_windows.borrow_mut().push((spec.id.clone(), None));
			let window_key = spec.id.clone();
			new_window(
				move |window_id| {
					OPEN_WINDOWS.with(|open_windows| {
						for item in open_windows.borrow_mut().iter_mut() {
							if item.0 == window_key {
								item.1 = Some(window_id);
							}
						}
					});
					view()
						.style(styles::default_window_styles)
						.on_event_cont(listener::WindowClosed, move |_, _| {
							closing_window(spec.id.clone(), || on_close());
						})
						.on_event(listener::KeyDown, move |_, event| {
							if event.code == Code::KeyW && event.modifiers == Modifiers::META
							{
								close_window(window_id);
								EventPropagation::Stop
							} else {
								EventPropagation::Continue
							}
						})
				},
				Some(
					WindowConfig::default()
						.size(size)
						.title(spec.title.clone())
						.with_mac_os_config(|settings| {
							settings
								.movable_by_window_background(false)
								.tabbing_identifier(spec.title.clone())
						}),
				),
			);
		}
	});
}
