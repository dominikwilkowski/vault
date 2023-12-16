use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, ModifiersState, PhysicalKey},
	reactive::create_signal,
	view::View,
	views::virtual_list,
	views::{
		container, label, scroll, Decorators, VirtualListDirection,
		VirtualListItemSize,
	},
	window::{close_window, WindowId},
	EventPropagation,
};

use crate::config::Config;
use crate::db::DbFields;
use crate::ui::colors::*;
use crate::ui::detail_view::HISTORY_WINDOW_OPEN;
use crate::ui::primitives::styles;

pub fn history_view(
	window_id: WindowId,
	id: usize,
	field: DbFields,
	config: Config,
) -> impl View {
	let long_list = config.db.read().unwrap().get_history(&id, &field).unwrap();
	let (long_list, _set_long_list) = create_signal(long_list);

	container(
		scroll(
			virtual_list(
				VirtualListDirection::Vertical,
				VirtualListItemSize::Fixed(Box::new(|| 30.0)),
				move || long_list.get(),
				move |item| item.clone(),
				move |(idx, item)| {
					label(move || item.to_string()).style(move |s| {
						s.height(30.0).padding(5).width_full().background(
							if let 0 = idx % 2 {
								C_BG_SIDE
							} else {
								C_BG_SIDE_SELECTED.with_alpha_factor(0.2)
							},
						)
					})
				},
			)
			.style(|s| s.flex_col().width_full()),
		)
		.style(|s| {
			s.width_full()
				.height_full()
				.class(scroll::Handle, styles::scrollbar_styles)
		}),
	)
	.style(|s| s.width_full().height_full())
	.on_event(EventListener::KeyDown, move |event| {
		let key = match event {
			Event::KeyDown(k) => (k.key.physical_key, k.modifiers),
			_ => (PhysicalKey::Code(KeyCode::F35), ModifiersState::default()),
		};

		if key.0 == PhysicalKey::Code(KeyCode::KeyW)
			&& key.1 == ModifiersState::SUPER
		{
			close_window(window_id);
		}

		EventPropagation::Continue
	})
	.on_event(EventListener::WindowClosed, move |_| {
		let mut history_window = HISTORY_WINDOW_OPEN.get();
		match field {
			DbFields::Username => history_window.username = false,
			DbFields::Password => history_window.password = false,
			DbFields::Notes => history_window.notes = false,
			_ => {}
		}
		HISTORY_WINDOW_OPEN.set(history_window);
		EventPropagation::Continue
	})
}
