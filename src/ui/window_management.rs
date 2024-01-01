use core::cell::RefCell;
use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, ModifiersState, PhysicalKey},
	kurbo::Size,
	view::View,
	views::Decorators,
	window::{close_window, new_window, WindowConfig, WindowId},
	EventPropagation,
};

use crate::db::DbFields;

thread_local! {
	pub(crate) static OPEN_WINDOWS: RefCell<Vec<(String, WindowId)>> = RefCell::new(Vec::new());
}

pub fn make_field_path(id: usize, field: &DbFields) -> String {
	format!("{}-{}", id, field)
}

pub fn closing_window(id: String, callback: impl Fn()) {
	OPEN_WINDOWS.with(|history_window| {
		let mut open_windows = history_window.borrow_mut();

		if let Some((pos, (_, window_id))) =
			open_windows.clone().iter().enumerate().find(|(_, item)| item.0 == id)
		{
			open_windows.remove(pos);

			close_window(*window_id);
			callback();
		}
	});
}

#[allow(clippy::redundant_closure)]
pub fn opening_window<V: View + 'static>(
	view: impl Fn() -> V + 'static,
	id: String,
	title: String,
	size: Size,
	on_close: impl Fn() + 'static,
) {
	OPEN_WINDOWS.with(|history_window| {
		if !history_window.borrow().iter().any(|item| item.0 == id) {
			new_window(
				move |window_id| {
					OPEN_WINDOWS.with(|open_windows| {
						open_windows.borrow_mut().push((id.clone(), window_id));
					});
					view()
						.on_event(EventListener::WindowClosed, move |_| {
							closing_window(id.clone(), || on_close());
							EventPropagation::Continue
						})
						.on_event(EventListener::KeyDown, move |event| {
							let key = match event {
								Event::KeyDown(k) => (k.key.physical_key, k.modifiers),
								_ => {
									(PhysicalKey::Code(KeyCode::F35), ModifiersState::default())
								}
							};

							if key.0 == PhysicalKey::Code(KeyCode::KeyW)
								&& key.1 == ModifiersState::SUPER
							{
								close_window(window_id);
							}

							EventPropagation::Continue
						})
				},
				Some(WindowConfig::default().size(size).title(title)),
			);
		}
	});
}
