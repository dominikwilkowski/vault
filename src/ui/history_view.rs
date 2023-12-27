use chrono::{DateTime, Local, Utc};
use floem::{
	event::{Event, EventListener},
	reactive::{create_rw_signal, create_signal},
	view::View,
	views::virtual_list,
	views::{
		h_stack, label, scroll, Decorators, VirtualListDirection,
		VirtualListItemSize,
	},
	EventPropagation,
};

use crate::config::Config;
use crate::db::DbFields;
use crate::ui::{
	colors::*,
	detail_view::{clipboard_button_slot, view_button_slot, SECRET_PLACEHOLDER},
	primitives::{
		styles,
		tooltip::{tooltip_view, TooltipSignals},
	},
};

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
		label(move || datetime_local.format("%v"))
			.style(|s| s.color(C_TEXT_SIDE_INACTIVE).font_size(9.0))
			.on_event(EventListener::PointerEnter, move |_event| {
				tooltip_signals.show(datetime_local.to_rfc2822());
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				tooltip_signals.hide();
				EventPropagation::Continue
			}),
		label(move || value.get()).style(|s| s.flex_grow(1.0)),
		view_button_slot(true, tooltip_signals, value, move || {
			config_viewbtn.db.read().unwrap().get_n_by_field(&id, &field, idx)
		}),
		clipboard_button_slot(tooltip_signals, move || {
			config.db.read().unwrap().get_n_by_field(&id, &field, idx)
		}),
	))
	.style(move |s| {
		s.flex()
			.flex_row()
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
	id: usize,
	field: DbFields,
	dates: Vec<(usize, u64)>,
	config: Config,
) -> impl View {
	let long_list: im::Vector<(usize, u64)> = dates.into();
	let (long_list, _set_long_list) = create_signal(long_list);

	let tooltip_signals = TooltipSignals::new();

	let history_view = h_stack((
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
			.style(|s| s.flex_col().flex_grow(1.0)),
		)
		.style(|s| {
			s.width_full()
				.height_full()
				.class(scroll::Handle, styles::scrollbar_styles)
		}),
		tooltip_view(tooltip_signals),
	))
	.style(|s| s.width_full().height_full())
	.on_event(EventListener::PointerMove, move |event| {
		let pos = match event {
			Event::PointerMove(p) => p.pos,
			_ => (0.0, 0.0).into(),
		};
		tooltip_signals.mouse_pos.set((pos.x, pos.y));
		EventPropagation::Continue
	})
	.on_resize(move |event| {
		tooltip_signals.window_size.set((event.x1, event.y1));
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
