use chrono::{DateTime, Local, Utc};

use floem::{
	event::{Event, EventListener},
	reactive::{create_rw_signal, create_signal},
	view::View,
	views::virtual_stack,
	views::{
		container, dyn_container, h_stack, label, scroll, Decorators,
		VirtualDirection, VirtualItemSize,
	},
	EventPropagation,
};

use crate::{
	config::Config,
	db::{DbFields, DynFieldKind},
	ui::{
		colors::*,
		details::{
			button_slots::{clipboard_button_slot, view_button_slot, ViewButtonSlot},
			detail_view::SECRET_PLACEHOLDER,
		},
		primitives::{
			styles,
			tooltip::{tooltip_view, TooltipSignals},
		},
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
	let view_button_switch = create_rw_signal(false);

	let dyn_field_kind = config.db.read().get_dyn_field_kind(&id, &field);
	let is_secret = match dyn_field_kind {
		DynFieldKind::TextLine | DynFieldKind::Url => false,
		DynFieldKind::SecretLine => true,
	};

	let field_value = if is_secret {
		create_rw_signal(String::from(SECRET_PLACEHOLDER))
	} else {
		create_rw_signal(config.db.read().get_last_by_field(&id, &field))
	};

	let config_viewbtn = config.clone();

	let datetime_utc: DateTime<Utc> =
		DateTime::from_timestamp(date as i64, 0).unwrap();
	let datetime_local: DateTime<Local> = datetime_utc.with_timezone(&Local);

	h_stack((
		label(move || datetime_local.format("%v"))
			.style(|s| s.color(C_TEXT_SIDE_INACTIVE).font_size(9.0))
			.on_event(EventListener::PointerEnter, move |_| {
				tooltip_signals.show(datetime_local.to_rfc2822());
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				tooltip_signals.hide();
				EventPropagation::Continue
			}),
		dyn_container(
			move || view_button_switch.get(),
			move |switch| {
				if switch {
					Box::new(
						container(
							scroll(label(move || field_value.get()))
								.style(|s| s.flex_grow(1.0).width(80)),
						)
						.style(|s| s.flex_grow(1.0).width(80)),
					)
				} else {
					Box::new(
						container(label(move || field_value.get()))
							.style(|s| s.flex_grow(1.0)),
					)
				}
			},
		)
		.style(|s| s.flex_grow(1.0)),
		view_button_slot(
			ViewButtonSlot {
				switch: view_button_switch,
				is_shown: is_secret,
				tooltip_signals,
				field_value,
			},
			move || config_viewbtn.db.read().get_n_by_field(&id, &field, idx),
		),
		clipboard_button_slot(tooltip_signals, move || {
			config.db.read().get_n_by_field(&id, &field, idx)
		}),
	))
	.style(move |s| {
		s.flex()
			.flex_row()
			.width_full()
			.max_width_full()
			.height(HISTORY_LINE_HEIGHT)
			.gap(4.0, 0.0)
			.padding_horiz(10)
			.items_center()
			.class(scroll::Handle, styles::scrollbar_styles)
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
			virtual_stack(
				VirtualDirection::Vertical,
				VirtualItemSize::Fixed(Box::new(|| HISTORY_LINE_HEIGHT)),
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
