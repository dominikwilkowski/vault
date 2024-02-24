use floem::{
	event::{Event, EventListener},
	kurbo::Size,
	reactive::{create_rw_signal, create_signal, WriteSignal},
	style::{CursorStyle, Position},
	view::View,
	views::virtual_stack,
	views::{
		container, h_stack, label, scroll, v_stack, Decorators, VirtualDirection,
		VirtualItemSize,
	},
	EventPropagation,
};

use crate::{
	db::Db,
	ui::{
		colors::*,
		import::import_detail_view::import_detail_view,
		primitives::{
			button::{icon_button, IconButton},
			checkbox::checkbox,
			que::Que,
			styles,
			tooltip::{tooltip_view, TooltipSignals},
		},
		window_management::{closing_window, opening_window, WindowSpec},
	},
};

const TOP_HEIGHT: f32 = 50.0;

fn import_line(
	item: (usize, bool),
	set_import_items: WriteSignal<im::Vector<(usize, bool)>>,
	tooltip_signals: TooltipSignals,
	db: Db,
) -> impl View {
	let does_overflow = create_rw_signal(false);
	let show_detail_window = create_rw_signal(false);

	let entry = db.get_by_id(&item.0);
	let full_title = entry.title.clone();

	let detail_icon = include_str!("../icons/detail.svg");
	let no_detail_icon = include_str!("../icons/no_detail.svg");

	let update_checkbox = move |id, state| {
		set_import_items.update(|items| {
			if let Some(index) = items.iter().position(|&x| x.0 == id) {
				items[index].1 = state;
			}
		});
	};

	h_stack((
		checkbox(move || item.1).on_update(move |state| {
			update_checkbox(item.0, state);
		}),
		label(move || entry.title.clone())
			.style(|s| {
				s.position(Position::Absolute)
					.margin_top(1)
					.text_ellipsis()
					.inset_left(16 + 5 + 10)
					.cursor(CursorStyle::Pointer)
					.inset_right(29.5 + 5.0 + 10.0)
			})
			.on_text_overflow(move |is_overflown| {
				does_overflow.set(is_overflown);
			})
			.on_event(EventListener::PointerEnter, move |_| {
				if does_overflow.get() {
					tooltip_signals.show(full_title.clone());
				}
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				tooltip_signals.hide();
				EventPropagation::Continue
			})
			.on_click_cont(move |_| {
				update_checkbox(item.0, !item.1);
			}),
		container(icon_button(
			IconButton {
				icon: String::from(detail_icon),
				icon2: Some(String::from(no_detail_icon)),
				tooltip: String::from("See details"),
				tooltip2: Some(String::from("Close details")),
				switch: Some(show_detail_window),
				tooltip_signals,
				..IconButton::default()
			},
			move |_| {
				let window_id = format!("import-detail-window-{}", item.0);
				let que_import_detail = Que::default();

				if show_detail_window.get() {
					let db_detail = db.clone();

					opening_window(
						move || {
							import_detail_view(item.0, db_detail.clone(), que_import_detail)
						},
						WindowSpec {
							id: window_id,
							title: String::from("Import Detail"),
						},
						Size::new(400.0, 320.0),
						move || {
							que_import_detail.unque_all_tooltips();
							show_detail_window.set(false);
						},
					);
				} else {
					closing_window(window_id, move || {});
				}
			},
		))
		.style(|s| s.position(Position::Absolute).inset_right(10)),
	))
	.style(|s| s.height(30).padding_left(10).width_full().items_center())
}

pub fn import_view(db: Db, que: Que) -> impl View {
	let tooltip_signals = TooltipSignals::new(que);

	let select_all = create_rw_signal(true);

	let import_items = db
		.get_list()
		.into_iter()
		.map(|(id, _title, _idx)| (id, true))
		.collect::<im::Vector<(usize, bool)>>();
	let (import_items, set_import_items) = create_signal(import_items.clone());

	let import_view = v_stack((
		h_stack((
			label(|| "Importing").style(|s| s.font_size(21.0).margin_bottom(3)),
			container(
				label(move || {
					format!(
						" Import {} ",
						import_items.get().iter().filter(|&(_, b)| *b).count()
					)
				})
				.keyboard_navigatable()
				.style(styles::button),
			)
			.style(|s| s.width_full().justify_end()),
		))
		.style(|s| {
			s.height(TOP_HEIGHT)
				.gap(5, 0)
				.padding(5)
				.items_center()
				.justify_center()
				.border_color(C_TOP_BG_BORDER)
				.border_bottom(1)
		}),
		scroll(
			v_stack((
				container(
					label(move || {
						if select_all.get() {
							String::from("Deselect all")
						} else {
							String::from("Select all")
						}
					})
					.on_click_cont(move |_| {
						set_import_items.update(|items| {
							items.iter_mut().for_each(|item| item.1 = !select_all.get());
						});
						select_all.set(!select_all.get());
					})
					.style(styles::button),
				)
				.style(|s| s.margin_left(10).margin_top(10)),
				virtual_stack(
					VirtualDirection::Vertical,
					VirtualItemSize::Fixed(Box::new(|| 30.0)),
					move || import_items.get(),
					move |item| *item,
					move |item| {
						import_line(item, set_import_items, tooltip_signals, db.clone())
					},
				)
				.style(|s| s.width_full().margin_bottom(10)),
			))
			.style(|s| s.width_full().gap(0, 5)),
		)
		.style(|s| {
			s.width_full()
				.position(Position::Absolute)
				.inset_top(TOP_HEIGHT)
				.inset_bottom(0.0)
				.min_width(0)
				.flex_grow(1.0)
				.class(scroll::Handle, styles::scrollbar_styles)
		}),
		tooltip_view(tooltip_signals),
	))
	.style(|s| s.flex().width_full().height_full())
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
			let id = import_view.id();
			import_view.on_event_stop(EventListener::KeyUp, move |e| {
				if let floem::event::Event::KeyUp(e) = e {
					if e.key.logical_key
						== floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11)
					{
						id.inspect();
					}
				}
			})
		},
		Err(_) => import_view,
	}
}
