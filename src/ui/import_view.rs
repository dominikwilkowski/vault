use floem::{
	event::{Event, EventListener},
	reactive::{create_signal, WriteSignal},
	style::Position,
	view::View,
	views::virtual_stack,
	views::{
		h_stack, label, scroll, v_stack, Decorators, VirtualDirection,
		VirtualItemSize,
	},
	widgets::checkbox,
	EventPropagation,
};

use crate::{
	// env::Environment,
	db::Db,
	ui::primitives::{
		que::Que,
		styles,
		tooltip::{tooltip_view, TooltipSignals},
	},
};

const TOP_HEIGHT: f32 = 25.0;

fn import_line(
	item: (usize, bool),
	set_import_items: WriteSignal<im::Vector<(usize, bool)>>,
	db: Db,
) -> impl View {
	let entry = db.get_by_id(&item.0);

	h_stack((
		checkbox(move || item.1).on_update(move |state| {
			set_import_items.update(|items| {
				if let Some(index) = items.iter().position(|&x| x.0 == item.0) {
					items[index].1 = state;
				}
			});
		}),
		label(move || entry.title.clone()).style(|s| s),
	))
	.style(|s| s.gap(5, 0).height(30).padding(5))
}

pub fn import_view(db: Db, que: Que) -> impl View {
	let tooltip_signals = TooltipSignals::new(que);

	let import_items = db
		.get_list()
		.into_iter()
		.map(|(_idx, _title, id)| (id, true))
		.collect::<im::Vector<(usize, bool)>>();
	let (import_items, set_import_items) = create_signal(import_items.clone());

	let import_view = v_stack((
		h_stack((
			label(|| "Importing").style(|s| s.font_size(16.0)),
			label(move || import_items.get().iter().filter(|&(_, b)| *b).count())
				.style(|s| s.font_size(9.0)),
		))
		.style(|s| s.height(TOP_HEIGHT).margin(5)),
		scroll(
			virtual_stack(
				VirtualDirection::Vertical,
				VirtualItemSize::Fixed(Box::new(|| 30.0)),
				move || import_items.get(),
				move |item| *item,
				move |item| import_line(item, set_import_items, db.clone()),
			)
			.style(|s| s),
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
