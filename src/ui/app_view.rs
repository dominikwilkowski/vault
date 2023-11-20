use floem::{
	action::exec_after,
	event::EventListener,
	reactive::{create_rw_signal, create_signal},
	style::{AlignContent, AlignItems, CursorStyle, Display, Position},
	view::View,
	views::{
		container, h_stack, label, scroll, svg, tab, v_stack, virtual_list, Decorators, VirtualListDirection,
		VirtualListItemSize,
	},
	widgets::{text_input, PlaceholderTextClass},
	EventPropagation,
};
use std::time::Duration;

use crate::db::db::{get_db_by_id, get_db_list};
use crate::ui::colors::*;

const SIDEBAR_WIDTH: f64 = 140.0;
const SEARCHBAR_HEIGHT: f64 = 30.0;

pub fn app_view() -> impl View {
	let db = get_db_list();
	let (list, set_list) = create_signal(db.clone());
	let (active_tab, set_active_tab) = create_signal(db[0].0);
	let search_text = create_rw_signal(String::from(""));
	let sidebar_scrolled = create_rw_signal(false);
	let tooltip_text = create_rw_signal(String::from(""));

	let search_icon = r##"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.7" stroke="#424242">
		<path stroke-linecap="round" stroke-linejoin="round" d="M9.75 9.75l4.5 4.5m0-4.5l-4.5 4.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
	</svg>"##;

	let search_text_input_view = text_input(search_text);
	let search_text_input_view_id = search_text_input_view.id();

	let search_bar = h_stack((
		label(|| "Search / Create:")
			.on_click_stop(move |_| {
				search_text_input_view_id.request_focus();
			})
			.style(|s| s.font_size(12.0).padding(3.0).padding_top(8.0).padding_left(10.0).color(C_TEXT_TOP)),
		container(
			search_text_input_view
				.placeholder("Press enter to create a new entry")
				.keyboard_navigatable()
				.on_event(EventListener::KeyDown, move |_| {
					set_list.update(|list: &mut im::Vector<(usize, &'static str, usize)>| {
						*list = get_db_list()
							.iter()
							.copied()
							.filter(|item| item.1.contains(&search_text.get()))
							.collect::<im::Vector<_>>();
					});
					EventPropagation::Continue
				})
				.style(|s| {
					s.padding(5.0)
						.width_full()
						.margin(3.0)
						.border_radius(2)
						.z_index(3)
						.border_color(C_TEXT_TOP)
						.cursor_color(C_FOCUS.with_alpha_factor(0.5))
						.hover(|s| s.background(C_FOCUS.with_alpha_factor(0.05)))
						.focus(|s| s.border_color(C_FOCUS).outline_color(C_FOCUS))
						.class(PlaceholderTextClass, |s| s.color(C_TEXT_MAIN.with_alpha_factor(0.5)))
				}),
		)
		.style(|s| s.width_full()),
		container(
			svg(move || search_icon.to_string())
				.style(|s| s.z_index(6).inset_top(7).inset_right(4).height(16.0).width(16.0).cursor(CursorStyle::Pointer)),
		)
		.on_click_stop(move |_| {
			search_text.set(String::from(""));
			set_list.update(|list: &mut im::Vector<(usize, &'static str, usize)>| {
				*list = get_db_list();
			});
		})
		.style(move |s| {
			s.position(Position::Absolute)
				.height(30.0)
				.width(30.0)
				.display(Display::None)
				.align_items(AlignItems::Baseline)
				.align_content(AlignContent::Center)
				.padding_left(10.0)
				.z_index(5)
				.inset_top(0)
				.inset_right(3)
				.cursor(CursorStyle::Pointer)
				.hover(|s| s.cursor(CursorStyle::Pointer))
				.apply_if(!search_text.get().is_empty(), |s| s.display(Display::Flex))
		}),
	))
	.style(|s| s.z_index(3).width_full().height(SEARCHBAR_HEIGHT).background(C_BG_TOP));

	let sidebar = scroll({
		virtual_list(
			VirtualListDirection::Vertical,
			VirtualListItemSize::Fixed(Box::new(|| 22.0)),
			move || list.get(),
			move |item| *item,
			move |item| {
				container(
					label(move || item.1)
						.style(|s| s.font_size(12.0).color(C_TEXT_SIDE))
						.keyboard_navigatable()
						.on_event(EventListener::PointerEnter, move |_| {
							tooltip_text.set(String::from(item.1));
							exec_after(Duration::from_secs_f64(0.6), move |_token| {
								if tooltip_text.get() == item.1 {
									// set our global tooltips content and to show it now
									println!("{}", item.1);
								}
							});
							EventPropagation::Continue
						})
						.on_event(EventListener::PointerLeave, move |_| {
							tooltip_text.set(String::from(""));
							// set our global tooltips to empty and hide
							EventPropagation::Continue
						})
						.on_click_stop(move |_| {
							set_active_tab.update(|v: &mut usize| {
								*v = item.0;
							});
						})
						.style(move |s| {
							s.text_ellipsis()
								.padding(10.0)
								.padding_top(3.0)
								.padding_bottom(3.0)
								.width(SIDEBAR_WIDTH)
								.items_start()
								.border_bottom(1.0)
								.border_color(C_BG_SIDE_BORDER)
								.color(C_TEXT_SIDE)
								.focus_visible(|s| s.border(2.).border_color(C_FOCUS))
								.background(if let 0 = item.2 % 2 {
									C_BG_SIDE
								} else {
									C_BG_SIDE_SELECTED.with_alpha_factor(0.2)
								})
								.apply_if(item.0 == active_tab.get(), |s| s.background(C_BG_SIDE_SELECTED))
								.hover(|s| {
									s.background(C_BG_SIDE_SELECTED.with_alpha_factor(0.6))
										.apply_if(item.0 == active_tab.get(), |s| s.background(C_BG_SIDE_SELECTED))
										.cursor(CursorStyle::Pointer)
								})
						}),
				)
			},
		)
		.style(|s| s.flex_col().width(SIDEBAR_WIDTH - 1.0).background(C_BG_SIDE))
	})
	.on_scroll(move |x| {
		if x.y0 > 0.0 {
			sidebar_scrolled.set(true)
		} else {
			sidebar_scrolled.set(false)
		}
	})
	.style(|s| {
		s.z_index(1)
			.width(SIDEBAR_WIDTH)
			.border_right(1.0)
			.border_top(1.0)
			.border_color(C_BG_SIDE_BORDER)
			.background(C_BG_SIDE)
			.class(scroll::Handle, |s| s.set(scroll::Thickness, 5.0))
	});

	let shadow_box_top = label(move || "").style(move |s| {
		s.position(Position::Absolute)
			.z_index(2)
			.inset_top(0)
			.inset_left(0)
			.inset_right(SIDEBAR_WIDTH + 10.0)
			.height(1)
			.box_shadow_blur(3)
			.box_shadow_color(C_SHADOW)
			.box_shadow_spread(2)
			.display(Display::None)
			.apply_if(sidebar_scrolled.get(), |s| s.display(Display::Flex))
	});

	let shadow_box_right = label(move || "").style(|s| {
		s.position(Position::Absolute)
			.z_index(2)
			.inset_top(0)
			.inset_left(SIDEBAR_WIDTH)
			.height_full()
			.width(1)
			.box_shadow_blur(3)
			.box_shadow_color(C_SHADOW)
			.box_shadow_spread(2)
	});

	let main_window = scroll(
		tab(
			move || list.get().iter().position(|item| item.0 == active_tab.get()).unwrap_or(0),
			move || list.get(),
			move |it| *it,
			|it| {
				let data = get_db_by_id(it.0);
				container(label(move || format!("id:{} title:{} body:{}", data.0, data.1, data.2)).style(|s| s.padding(8.0)))
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
			.z_index(3)
			.class(scroll::Handle, |s| s.set(scroll::Thickness, 5.0))
	});

	let content = h_stack((sidebar, shadow_box_top, shadow_box_right, main_window))
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
