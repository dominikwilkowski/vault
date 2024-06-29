use chrono::{DateTime, Local, Utc};
use std::sync::Arc;

use floem::{
	event::{Event, EventListener},
	reactive::{create_rw_signal, provide_context},
	views::{
		container, label, scroll, virtual_stack, Decorators, VirtualDirection,
		VirtualItemSize,
	},
	IntoView, View,
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
		primitives::tooltip::{tooltip_view, TooltipSignals},
	},
};

const HISTORY_LINE_HEIGHT: f64 = 31.0;
const PADDING: f64 = 10.0;

fn history_line(
	idx: usize,
	id: usize,
	field: DbFields,
	date: u64,
	tooltip_signals: TooltipSignals,
	db: Arc<Db>,
) -> impl IntoView {
	let view_button_switch = create_rw_signal(false);

	let dyn_field_kind = db.get_field_kind(&id, &field);
	let is_secret = match dyn_field_kind {
		DynFieldKind::TextLine
		| DynFieldKind::MultiLine
		| DynFieldKind::Url
		| DynFieldKind::Heading => false,
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

	(
		datetime_local
			.format("%v")
			.to_string()
			.style(|s| s.color(C_SIDE_TEXT_INACTIVE).font_size(9.0).min_width(60))
			.on_event_cont(EventListener::PointerEnter, move |_| {
				tooltip_signals.show(datetime_local.to_rfc2822());
			})
			.on_event_cont(EventListener::PointerLeave, move |_| {
				tooltip_signals.hide();
			}),
		container(
			scroll(
				label(move || replace_consecutive_newlines(field_value.get().clone()))
					.style(|s| s.font_family(String::from("Monospace")))
					.style(move |s| {
						s.apply_if(is_multiline, |s| {
							s.padding_top(PADDING).padding_bottom(PADDING)
						})
					}),
			)
			.style(move |s| {
				s.flex_grow(1.0)
					.width(80)
					.apply_if(is_multiline, |s| s.height(MULTILINE_HEIGHT + PADDING))
			}),
		)
		.into_any()
		.style(|s| s.flex_grow(1.0).width(80)),
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
	)
		.style(move |s| {
			s.flex()
				.flex_row()
				.width_full()
				.max_width_full()
				.height(HISTORY_LINE_HEIGHT)
				.apply_if(is_multiline, |s| s.height(MULTILINE_HEIGHT + PADDING))
				.row_gap(4)
				.padding_horiz(PADDING)
				.items_center()
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
) -> impl IntoView {
	provide_context(tooltip_signals);

	let dates_list: im::Vector<(usize, u64)> = dates.into();
	let dates_list = create_rw_signal(dates_list);

	let db_height = db.clone();

	let history_view = (
		scroll(
			virtual_stack(
				VirtualDirection::Vertical,
				VirtualItemSize::Fn(Box::new(move |_| {
					if matches!(
						db_height.get_field_kind(&id, &field),
						DynFieldKind::MultiLine | DynFieldKind::MultiLineSecret
					) {
						MULTILINE_HEIGHT + PADDING
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
		.style(|s| s.width_full().height_full()),
		tooltip_view(tooltip_signals),
	)
		.style(|s| s.width_full().height_full())
		.on_event_cont(EventListener::PointerMove, move |event| {
			let pos = match event {
				Event::PointerMove(p) => p.pos,
				_ => (0.0, 0.0).into(),
			};
			tooltip_signals.mouse_pos.set((pos.x, pos.y));
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
