use url_escape;
use webbrowser;

use floem::{
	event::{Event, EventListener},
	reactive::{create_rw_signal, provide_context},
	style::CursorStyle,
	views::{scroll, svg, v_stack_from_iter, Decorators},
	IntoView, View,
};

use crate::{
	db::{Db, DbFields, DynFieldKind},
	ui::{
		colors::*,
		details::{
			button_slots::{history_button_slot, HistoryButtonSlot},
			detail_view::{LABEL_WIDTH, SECRET_PLACEHOLDER},
		},
		primitives::{
			que::Que,
			tooltip::{tooltip_view, TooltipSignals},
		},
	},
};

pub fn import_detail_view(id: usize, db: Db, que: Que) -> impl IntoView {
	let tooltip_signals = TooltipSignals::new(que);
	provide_context(tooltip_signals);

	let is_overflowing = create_rw_signal(false);

	let password_icon = include_str!("../icons/password.svg");

	let mut field_list = db.get_fields(&id);
	field_list.sort_by_key(|&(_, is_visible)| !is_visible);

	let entry = db.get_by_id(&id);
	let title = entry.title.clone();

	let import_detail_view = scroll(
		(
			tooltip_view(tooltip_signals),
			(
				svg(move || String::from(password_icon))
					.style(|s| s.width(24).height(24).min_width(24)),
				entry
					.title
					.clone()
					.into_view()
					.on_text_overflow(move |is_overflown| {
						is_overflowing.set(is_overflown);
					})
					.on_event_cont(EventListener::PointerEnter, move |_| {
						if is_overflowing.get() {
							tooltip_signals.show(title.clone());
						}
					})
					.on_event_cont(EventListener::PointerLeave, move |_| {
						tooltip_signals.hide();
					})
					.style(|s| {
						s.text_ellipsis().font_size(24.0).max_width(300 - 24 - 5 - 10)
					}),
			)
				.style(|s| {
					s.items_center()
						.width(300)
						.justify_center()
						.row_gap(5)
						.margin_left(5)
						.margin_top(15)
						.margin_right(20)
						.margin_bottom(20)
				}),
			v_stack_from_iter(field_list.into_iter().map(|(field, is_visible)| {
				let dates = create_rw_signal(db.get_history_dates(&id, &field));

				let field_title = match field {
					DbFields::Fields(_) => db.get_name_of_field(&id, &field),
					other => format!("{}", other),
				};
				let field_title_history = field_title.clone();
				let dyn_field_kind = db.get_field_kind(&id, &field);
				let is_secret = match dyn_field_kind {
					DynFieldKind::TextLine
					| DynFieldKind::MultiLine
					| DynFieldKind::Url => false,
					DynFieldKind::TextLineSecret | DynFieldKind::MultiLineSecret => true,
				};
				let is_url_field = matches!(dyn_field_kind, DynFieldKind::Url);

				let field_value = if is_secret {
					String::from(SECRET_PLACEHOLDER)
				} else {
					db.get_last_by_field(&id, &field)
				};
				let field_value_browser = field_value.clone();

				(
					field_title
						.clone()
						.style(move |s| s.width(LABEL_WIDTH).text_ellipsis()),
					field_value
						.clone()
						.style(move |s| {
							s.flex_grow(1.0).text_ellipsis().hover(|s| {
								s.apply_if(is_url_field, |s| {
									s.color(C_FOCUS).cursor(CursorStyle::Pointer)
								})
							})
						})
						.on_click_cont(move |_| {
							if is_url_field {
								let _ = webbrowser::open(&url_escape::encode_fragment(
									&field_value_browser,
								));
							}
						}),
					history_button_slot(HistoryButtonSlot {
						id,
						field,
						dates,
						is_shown: true,
						field_title: field_title_history,
						db: db.clone().into(),
					}),
				)
					.style(move |s| {
						s.items_center()
							.width_full()
							.padding_left(5)
							.padding_right(5)
							.row_gap(5)
							.apply_if(!is_visible, |s| s.color(C_MAIN_TEXT_INACTIVE))
					})
			}))
			.style(|s| s.margin_bottom(10).column_gap(5).width_full()),
		)
			.style(|s| {
				s.flex_col().padding(8.0).width(400).justify_center().items_center()
			}),
	)
	.style(|s| s.width_full().height_full().background(C_MAIN_BG))
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
			let id = import_detail_view.id();
			import_detail_view.on_event_stop(EventListener::KeyUp, move |e| {
				if let floem::event::Event::KeyUp(e) = e {
					if e.key.logical_key
						== floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11)
					{
						id.inspect();
					}
				}
			})
		},
		Err(_) => import_detail_view,
	}
}
