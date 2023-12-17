use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, ModifiersState, PhysicalKey},
	reactive::{create_rw_signal, create_signal},
	view::View,
	views::virtual_list,
	views::{
		container, h_stack, label, scroll, Decorators, VirtualListDirection,
		VirtualListItemSize,
	},
	window::{close_window, WindowId},
	EventPropagation,
};

use crate::config::Config;
use crate::db::DbFields;
use crate::ui::colors::*;
use crate::ui::detail_view::{
	clipboard_button_slot, view_button_slot, HISTORY_WINDOW_OPEN,
	SECRET_PLACEHOLDER,
};
use crate::ui::primitives::{styles, tooltip::TooltipSignals};

use chrono::{DateTime, Local, Utc};

const HISTORY_LINE_HEIGHT: f64 = 31.0;

fn history_line(
	idx: usize,
	id: usize,
	field: DbFields,
	date: u64,
	tooltip_signals: TooltipSignals,
	config: Config,
) -> impl View {
	let value = create_rw_signal(String::from(SECRET_PLACEHOLDER));

	let config_viewbtn = config.clone();

	let datetime_utc: DateTime<Utc> =
		DateTime::from_timestamp(date as i64, 0).unwrap();
	let datetime_local: DateTime<Local> = datetime_utc.with_timezone(&Local);

	h_stack((
		label(move || datetime_local.format("%a %b %e"))
			.style(|s| s.color(C_TEXT_SIDE_INACTIVE).font_size(9.0)),
		label(move || value.get()).style(|s| s.width_full()),
		view_button_slot(true, tooltip_signals, value, move || {
			config_viewbtn.db.read().unwrap().get_n_by_field(&id, &field, idx)
		}),
		clipboard_button_slot(tooltip_signals, move || {
			config.db.read().unwrap().get_n_by_field(&id, &field, idx)
		}),
	))
	.style(move |s| {
		s.width_full()
			.height(HISTORY_LINE_HEIGHT)
			.gap(4.0, 0.0)
			.padding_horiz(10)
			.items_center()
			.background(if let 0 = idx % 2 {
				C_BG_SIDE
			} else {
				C_BG_SIDE_SELECTED.with_alpha_factor(0.2)
			})
	})
}

pub fn history_view(
	window_id: WindowId,
	id: usize,
	field: DbFields,
	dates: Vec<(usize, u64)>,
	tooltip_signals: TooltipSignals,
	config: Config,
) -> impl View {
	let long_list: im::Vector<(usize, u64)> = dates.into();
	let (long_list, _set_long_list) = create_signal(long_list);

	let history_view = container(
		scroll(
			virtual_list(
				VirtualListDirection::Vertical,
				VirtualListItemSize::Fixed(Box::new(|| HISTORY_LINE_HEIGHT)),
				move || long_list.get(),
				move |item| *item,
				move |(idx, date)| {
					history_line(idx, id, field, date, tooltip_signals, config.clone())
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
	});

	match std::env::var("DEBUG") {
		Ok(_) => {
			// for debugging the layout
			let id = history_view.id();
			history_view.on_event_stop(EventListener::KeyUp, move |e| {
				if let floem::event::Event::KeyUp(e) = e {
					if e.key.logical_key
						== floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11)
					{
						id.inspect();
					}
				}
			})
		}
		Err(_) => history_view,
	}
}
