use chrono::{DateTime, Local, Utc};
use std::sync::Arc;

use floem::{
	event::{Event, EventListener},
	reactive::create_rw_signal,
	view::View,
	views::{
		container, dyn_container, h_stack, label, scroll, virtual_stack,
		Decorators, VirtualDirection, VirtualItemSize,
	},
	EventPropagation,
};

use crate::{
	db::{Db, DbFields, DynFieldKind},
	ui::{
		colors::*,
		details::{
			button_slots::{clipboard_button_slot, view_button_slot, ViewButtonSlot},
			detail_view::{
				MULTILINE_HEIGHT, SECRET_MULTILINE_PLACEHOLDER, SECRET_PLACEHOLDER,
			},
			list_item::replace_consecutive_newlines,
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
	db: Arc<Db>,
) -> impl View {
	let view_button_switch = create_rw_signal(false);

	let dyn_field_kind = db.get_field_kind(&id, &field);
	let is_secret = match dyn_field_kind {
		DynFieldKind::TextLine | DynFieldKind::MultiLine | DynFieldKind::Url => {
			false
		},
		DynFieldKind::TextLineSecret | DynFieldKind::MultiLineSecret => true,
	};

	let is_multiline = matches!(
		dyn_field_kind,
		DynFieldKind::MultiLine | DynFieldKind::MultiLineSecret
	);

	let field_value = if is_secret {
		create_rw_signal(if is_multiline {
			String::from(SECRET_MULTILINE_PLACEHOLDER)
		} else {
			String::from(SECRET_PLACEHOLDER)
		})
	} else {
		create_rw_signal(db.get_last_by_field(&id, &field))
	};

	let db_view_button = db.clone();

	let datetime_utc: DateTime<Utc> =
		DateTime::from_timestamp(date as i64, 0).unwrap();
	let datetime_local: DateTime<Local> = datetime_utc.with_timezone(&Local);

	h_stack((
		label(move || datetime_local.format("%v"))
			.style(|s| s.color(C_SIDE_TEXT_INACTIVE).font_size(9.0).min_width(60))
			.on_event(EventListener::PointerEnter, move |_| {
				tooltip_signals.show(datetime_local.to_rfc2822());
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				tooltip_signals.hide();
				EventPropagation::Continue
			}),
		dyn_container(
			move || (view_button_switch.get(), field_value.get()),
			move |(switch, value)| {
				let value_with_lines = replace_consecutive_newlines(value.clone());

				match switch {
					// Show secret data
					true => container(
						scroll(label(move || value_with_lines.clone()).style(move |s| {
							s.apply_if(is_multiline, |s| s.padding_top(10).padding_bottom(10))
						}))
						.style(move |s| {
							s.flex_grow(1.0)
								.width(80)
								.apply_if(is_multiline, |s| s.height(MULTILINE_HEIGHT))
						}),
					)
					.any()
					.style(|s| s.flex_grow(1.0).width(80)),
					// Show placeholder
					false => label(move || value_with_lines.clone())
						.style(move |s| {
							s.flex_grow(1.0).apply_if(is_multiline, |s| {
								s.height(MULTILINE_HEIGHT).padding_top(10)
							})
						})
						.any(),
				}
			},
		)
		.style(|s| s.flex_grow(1.0)),
		view_button_slot(
			ViewButtonSlot {
				switch: view_button_switch,
				is_shown: is_secret,
				is_multiline,
				field_value,
			},
			move || db_view_button.get_n_by_field(&id, &field, idx),
		),
		clipboard_button_slot(move || db.get_n_by_field(&id, &field, idx)),
	))
	.style(move |s| {
		s.flex()
			.flex_row()
			.width_full()
			.max_width_full()
			.height(HISTORY_LINE_HEIGHT)
			.apply_if(is_multiline, |s| s.height(MULTILINE_HEIGHT))
			.gap(4.0, 0.0)
			.padding_horiz(10)
			.items_center()
			.class(scroll::Handle, styles::scrollbar_styles)
			.background(if let 0 = idx % 2 {
				C_SIDE_BG
			} else {
				C_SIDE_BG_SELECTED.with_alpha_factor(0.2)
			})
	})
}

pub fn history_view(
	id: usize,
	field: DbFields,
	dates: Vec<(usize, u64)>,
	tooltip_signals: TooltipSignals,
	db: Arc<Db>,
) -> impl View {
	let dates_list: im::Vector<(usize, u64)> = dates.into();
	let dates_list = create_rw_signal(dates_list);

	let db_height = db.clone();

	let history_view = h_stack((
		scroll(
			virtual_stack(
				VirtualDirection::Vertical,
				VirtualItemSize::Fn(Box::new(move |_| {
					if matches!(
						db_height.get_field_kind(&id, &field),
						DynFieldKind::MultiLine | DynFieldKind::MultiLineSecret
					) {
						MULTILINE_HEIGHT
					} else {
						HISTORY_LINE_HEIGHT
					}
				})),
				move || dates_list.get(),
				move |item| *item,
				move |(idx, date)| {
					history_line(idx, id, field, date, tooltip_signals, db.clone())
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
		},
		Err(_) => history_view,
	}
}
