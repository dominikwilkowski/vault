use floem::{
	event::{Event, EventListener},
	reactive::{create_rw_signal, create_signal},
	style::{AlignContent, AlignItems},
	view::View,
	views::virtual_stack,
	views::{
		h_stack, label, svg, v_stack, Decorators, VirtualDirection, VirtualItemSize,
	},
	EventPropagation,
};

use crate::{
	db::{Db, DbFields},
	env::Environment,
	ui::{
		details::list_item::{list_item, ListItem},
		primitives::{que::Que, tooltip::TooltipSignals},
	},
};

pub fn import_detail_view(id: usize, db: Db, que: Que) -> impl View {
	let tooltip_signals = TooltipSignals::new(que);

	let is_overflowing = create_rw_signal(false);

	let password_icon = include_str!("../icons/password.svg");

	let field_list: im::Vector<DbFields> = db.get_dyn_fields(&id).into();
	let (dyn_field_list, set_dyn_field_list) = create_signal(field_list);

	let entry = db.get_by_id(&id);
	let title = entry.title.clone();

	let hidden_field_len = create_rw_signal(0);
	let (_, set_hidden_field_list) = create_signal(im::Vector::new());

	let (_, set_list) = create_signal(im::Vector::new());
	let env = Environment::default();
	let env_fields = env.clone();

	let import_detail_view = v_stack((
		h_stack((
			svg(move || String::from(password_icon))
				.style(|s| s.width(24).height(24).min_width(24)),
			label(move || entry.title.clone())
				.on_text_overflow(move |is_overflown| {
					is_overflowing.set(is_overflown);
				})
				.on_event(EventListener::PointerEnter, move |_| {
					if is_overflowing.get() {
						tooltip_signals.show(title.clone());
					}
					EventPropagation::Continue
				})
				.on_event(EventListener::PointerLeave, move |_| {
					tooltip_signals.hide();
					EventPropagation::Continue
				})
				.style(|s| s.text_ellipsis().font_size(24.0).max_width_full()),
		))
		.style(|s| {
			s.flex()
				.flex_row()
				.align_items(AlignItems::Center)
				.max_width_pct(90.0)
				.gap(5, 0)
				.margin(5)
				.margin_right(20)
				.margin_top(15)
				.margin_bottom(20)
		}),
		v_stack((
			list_item(ListItem {
				id,
				field: DbFields::Title,
				set_hidden_field_list,
				set_dyn_field_list,
				hidden_field_len,
				is_hidden: false,
				tooltip_signals,
				set_list,
				env: env.clone(),
			}),
			virtual_stack(
				VirtualDirection::Vertical,
				VirtualItemSize::Fixed(Box::new(|| 30.0)),
				move || dyn_field_list.get(),
				move |item| *item,
				move |field| {
					list_item(ListItem {
						id,
						field,
						set_hidden_field_list,
						set_dyn_field_list,
						hidden_field_len,
						is_hidden: false,
						tooltip_signals,
						set_list,
						env: env_fields.clone(),
					})
					.style(|s| s.padding_bottom(5))
				},
			)
			.style(|s| s.margin_bottom(10)),
		))
		.style(|s| s.gap(0, 5)),
	))
	.style(|s| {
		s.padding(8.0)
			.width_full()
			.max_width_full()
			.justify_content(AlignContent::Center)
			.align_items(AlignItems::Center)
	})
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
