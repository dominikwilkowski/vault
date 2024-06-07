use core::cell::RefCell;

use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, Modifiers, PhysicalKey},
	kurbo::Size,
	views::Decorators,
	window::{close_window, new_window, WindowConfig, WindowId},
	IntoView,
};

use crate::{db::DbFields, ui::primitives::styles};

thread_local! {
	pub(crate) static OPEN_WINDOWS: RefCell<Vec<(String, WindowId)>> = const { RefCell::new(Vec::new()) };
}

pub fn make_field_path(id: usize, field: &DbFields) -> String {
	format!("{}-{}", id, field)
}

pub fn closing_window(id: String, callback: impl Fn()) {
	OPEN_WINDOWS.with(|all_windows| {
		let mut open_windows = all_windows.borrow_mut();

		if let Some((pos, (_, window_id))) =
			open_windows.clone().iter().enumerate().find(|(_, item)| item.0 == id)
		{
			open_windows.remove(pos);

			close_window(*window_id);
			callback();
		}
	});
}

pub fn close_all_windows() {
	OPEN_WINDOWS.with(|all_windows| {
		let mut open_windows = all_windows.borrow_mut();
		while open_windows.len() > 0 {
			let window_id = open_windows.pop().unwrap().1;
			close_window(window_id);
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
			new_window(
				move |window_id| {
					OPEN_WINDOWS.with(|open_windows| {
						open_windows.borrow_mut().push((spec.id.clone(), window_id));
					});
					view()
						.style(styles::default_window_styles)
						.on_event_cont(EventListener::WindowClosed, move |_| {
							closing_window(spec.id.clone(), || on_close());
						})
						.on_event_cont(EventListener::KeyDown, move |event| {
							let key = match event {
								Event::KeyDown(k) => (k.key.physical_key, k.modifiers),
								_ => (PhysicalKey::Code(KeyCode::F35), Modifiers::default()),
							};

							if key.0 == PhysicalKey::Code(KeyCode::KeyW)
								&& key.1 == Modifiers::META
							{
								close_window(window_id);
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
