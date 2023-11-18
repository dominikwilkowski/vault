use floem::{
	event::EventListener,
	reactive::{create_rw_signal, create_signal},
	style::{CursorStyle, Position},
	view::View,
	views::{
		container_box, h_stack, label, scroll, tab, v_stack, virtual_list, Decorators, VirtualListDirection,
		VirtualListItemSize,
	},
	widgets::text_input,
	EventPropagation,
};

use crate::db::db::{get_db_by_id, get_db_list};
use crate::ui::colors::*;

const SIDEBAR_WIDTH: f64 = 140.0;
const SEARCHBAR_HEIGHT: f64 = 30.0;

pub fn app_view() -> impl View {
	let db = get_db_list();
	let (list, set_list) = create_signal(db.clone());
	let (active_tab, set_active_tab) = create_signal(db[0].0);
	let search_text = create_rw_signal("".to_string());

	let search_bar = h_stack((
		label(|| String::from("Search / Create:"))
			.style(|s| s.font_size(12.0).padding(3.0).padding_top(8.0).padding_left(10.0).color(C_TEXT_TOP)),
		container_box(
			text_input(search_text)
				.keyboard_navigatable()
				.on_event(EventListener::KeyDown, move |_| {
					set_list.update(|list: &mut im::Vector<(usize, &'static str)>| {
						*list = db.iter().copied().filter(|item| item.1.contains(&search_text.get())).collect::<im::Vector<_>>();
					});
					EventPropagation::Continue
				})
				.style(|s| {
					s.padding(5.0)
						.width_full()
						.margin(3.0)
						.border_radius(2)
						.border_color(C_TEXT_TOP)
						.cursor_color(C_FOCUS)
						.focus(|s| s.border_color(C_FOCUS).outline_color(C_FOCUS))
				}),
		)
		.style(|s| s.width_full()),
	))
	.style(|s| s.width_full().height(30.0).background(C_BG_TOP));

	let sidebar = scroll({
		virtual_list(
			VirtualListDirection::Vertical,
			VirtualListItemSize::Fixed(Box::new(|| 36.0)),
			move || list.get(),
			move |item| *item,
			move |item| {
				v_stack((label(move || format!("#{} - {}", item.0, item.1)).style(|s| s.font_size(12.0)),))
					.keyboard_navigatable()
					.on_click_stop(move |_| {
						set_active_tab.update(|v: &mut usize| {
							*v = item.0;
						});
					})
					.style(move |s| {
						s.padding(10.0)
							.width(SIDEBAR_WIDTH)
							.items_start()
							.border_bottom(1.0)
							.border_color(C_BG_SIDE_BORDER)
							.color(C_TEXT_SIDE)
							.apply_if(item.0 == active_tab.get(), |s| s.background(C_BG_SIDE_SELECTED))
							.focus_visible(|s| s.border(2.).border_color(C_FOCUS))
							.hover(|s| {
								s.background(C_BG_SIDE_SELECTED.with_alpha_factor(0.6))
									.apply_if(item.0 == active_tab.get(), |s| s.background(C_BG_SIDE_SELECTED))
									.cursor(CursorStyle::Pointer)
							})
					})
			},
		)
		.style(|s| s.flex_col().width(SIDEBAR_WIDTH - 1.0))
	})
	.style(|s| {
		s.width(SIDEBAR_WIDTH).border_right(1.0).border_top(1.0).border_color(C_BG_SIDE_BORDER).background(C_BG_SIDE)
	});

	let main_window = scroll(
		tab(
			move || list.get().iter().position(|item| item.0 == active_tab.get()).unwrap_or(0),
			move || list.get(),
			move |it| *it,
			|it| {
				let data = get_db_by_id(it.0);
				container_box(
					label(move || format!("id:{} title:{} body:{}", data.0, data.1, data.2)).style(|s| s.padding(8.0)),
				)
			},
		)
		.style(|s| s.flex_col().items_start().padding_bottom(10.0)),
	)
	.style(|s| {
		s.flex_col()
			.flex_basis(0)
			.min_width(0)
			.flex_grow(1.0)
			.background(C_BG_MAIN)
			.border_top(1.0)
			.border_color(C_BG_TOP_BORDER)
	});

	let content = h_stack((sidebar, main_window))
		.style(|s| s.position(Position::Absolute).inset_top(SEARCHBAR_HEIGHT).inset_bottom(0.0).width_full());

	let view = v_stack((search_bar, content)).style(|s| s.width_full().height_full());

	match std::env::var("DEBUG") {
		Ok(_) => {
			// for debugging the layout
			let id = view.id();
			view.on_event_stop(EventListener::KeyUp, move |e| {
				if let floem::event::Event::KeyUp(e) = e {
					if e.key.logical_key == floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11) {
						id.inspect();
					}
				}
			})
		}
		Err(_) => view,
	}
}
