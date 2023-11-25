use floem::{
	action::exec_after,
	event::{Event, EventListener},
	kurbo::Size,
	reactive::{create_rw_signal, create_signal},
	style::{AlignContent, AlignItems, CursorStyle, Display, Position},
	view::View,
	views::{
		container, h_stack, label, scroll, svg, tab, v_stack, virtual_list,
		Decorators, VirtualListDirection, VirtualListItemSize,
	},
	window::{new_window, WindowConfig},
	EventPropagation,
};
use std::time::Duration;

use crate::db::db::get_db_list;
use crate::ui::{
	colors::*,
	detail_view::detail_view,
	primitives::{button::icon_button, input_field::input_field, styles},
	settings_view::settings_view,
};

const SIDEBAR_WIDTH: f64 = 140.0;
const SEARCHBAR_HEIGHT: f64 = 30.0;

pub fn app_view() -> impl View {
	let db = get_db_list();

	let sidebar_width = create_rw_signal(SIDEBAR_WIDTH);
	let is_sidebar_dragging = create_rw_signal(false);
	let (list, set_list) = create_signal(db.clone());
	let (active_tab, set_active_tab) = create_signal(db[0].0);
	let search_text = create_rw_signal(String::from(""));
	let sidebar_scrolled = create_rw_signal(false);
	let tooltip_text = create_rw_signal(String::from(""));
	let tooltip_pos = create_rw_signal((0.0, 0.0));
	let tooltip_visible = create_rw_signal(false);
	let mouse_pos = create_rw_signal((0.0, 0.0));
	let window_size = create_rw_signal((0.0, 0.0));

	let clear_icon = include_str!("./icons/clear.svg");
	let settings_icon = include_str!("./icons/settings.svg");

	let search_text_input_view =
		input_field(search_text, |s| s.width_full().padding_right(30).z_index(3));
	let search_text_input_view_id = search_text_input_view.id();

	let search_bar = h_stack((
		label(|| "Search / Create:")
			.on_click_stop(move |_| {
				search_text_input_view_id.request_focus();
			})
			.style(|s| {
				s.font_size(12.0)
					.padding(3.0)
					.padding_top(8.0)
					.padding_left(10.0)
					.color(C_TEXT_TOP)
			}),
		container(
			search_text_input_view
				.placeholder("Press enter to create a new entry")
				.keyboard_navigatable()
				.on_event(EventListener::KeyDown, move |_| {
					set_list.update(
						|list: &mut im::Vector<(usize, &'static str, usize)>| {
							*list = get_db_list()
								.iter()
								.copied()
								.filter(|item| item.1.contains(&search_text.get()))
								.collect::<im::Vector<_>>();
						},
					);
					EventPropagation::Continue
				}),
		)
		.style(|s| s.width_full()),
		container(svg(move || clear_icon.to_string()).style(|s| {
			s.z_index(6)
				.inset_top(7)
				.inset_right(4)
				.height(16.0)
				.width(16.0)
				.cursor(CursorStyle::Pointer)
		}))
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
				.inset_right(29)
				.cursor(CursorStyle::Pointer)
				.hover(|s| s.cursor(CursorStyle::Pointer))
				.apply_if(!search_text.get().is_empty(), |s| s.display(Display::Flex))
		}),
		icon_button(String::from(settings_icon), |_| {
			new_window(
				settings_view,
				Some(
					WindowConfig::default()
						.size(Size::new(430.0, 400.0))
						.title("Vault Settings"),
				),
			);
		}),
	))
	.style(|s| {
		s.z_index(3).width_full().height(SEARCHBAR_HEIGHT).background(C_BG_TOP)
	});

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
						.on_event(EventListener::PointerEnter, move |_event| {
							// TODO: check if the text is actually clipped (different fonts will clip at different character limits)
							if item.1.len() > 20 {
								tooltip_text.set(String::from(item.1));
								exec_after(Duration::from_secs_f64(0.6), move |_| {
									if tooltip_text.get() == item.1 {
										let pos = mouse_pos.get();
										let y = if window_size.get().1 > pos.1 + 33.0 {
											pos.1 + 13.0
										} else {
											pos.1 - 23.0
										};
										tooltip_pos.set((pos.0 + 13.0, y));
										tooltip_text.set(String::from(item.1));
										tooltip_visible.set(true);
									}
								});
							}
							EventPropagation::Continue
						})
						.on_event(EventListener::PointerLeave, move |_| {
							tooltip_text.set(String::from(""));
							tooltip_visible.set(false);
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
								.width(sidebar_width.get())
								.items_start()
								.border_bottom(1.0)
								.border_color(C_BG_SIDE_BORDER)
								.color(C_TEXT_SIDE)
								.focus_visible(|s| s.border(1).border_color(C_FOCUS))
								.background(if let 0 = item.2 % 2 {
									C_BG_SIDE
								} else {
									C_BG_SIDE_SELECTED.with_alpha_factor(0.2)
								})
								.apply_if(item.0 == active_tab.get(), |s| {
									s.background(C_BG_SIDE_SELECTED)
								})
								.hover(|s| {
									s.background(C_BG_SIDE_SELECTED.with_alpha_factor(0.6))
										.apply_if(item.0 == active_tab.get(), |s| {
											s.background(C_BG_SIDE_SELECTED)
										})
										.cursor(CursorStyle::Pointer)
								})
						}),
				)
			},
		)
		.style(move |s| {
			s.flex_col().width(sidebar_width.get() - 1.0).background(C_BG_SIDE)
		})
	})
	.on_scroll(move |x| {
		tooltip_visible.set(false);
		if x.y0 > 0.0 {
			sidebar_scrolled.set(true)
		} else {
			sidebar_scrolled.set(false)
		}
	})
	.style(move |s| {
		s.z_index(1)
			.width(sidebar_width.get())
			.border_right(1.0)
			.border_top(1.0)
			.border_color(C_BG_SIDE_BORDER)
			.background(C_BG_SIDE)
			.class(scroll::Handle, styles::scrollbar_styles)
	});

	let shadow_box_top = label(move || "").style(move |s| {
		s.position(Position::Absolute)
			.z_index(2)
			.inset_top(0)
			.inset_left(0)
			.inset_right(sidebar_width.get() + 10.0)
			.height(1)
			.box_shadow_blur(3)
			.box_shadow_color(C_SHADOW_1)
			.box_shadow_spread(2)
			.display(Display::None)
			.apply_if(sidebar_scrolled.get(), |s| s.display(Display::Flex))
	});

	let shadow_box_right = label(move || "").style(move |s| {
		s.position(Position::Absolute)
			.z_index(2)
			.inset_top(0)
			.inset_left(sidebar_width.get())
			.height_full()
			.width(1)
			.box_shadow_blur(3)
			.box_shadow_color(C_SHADOW_1)
			.box_shadow_spread(2)
	});

	let dragger = label(|| "")
		.style(move |s| {
			s.position(Position::Absolute)
				.z_index(10)
				.inset_top(0)
				.inset_bottom(0)
				.inset_left(sidebar_width.get())
				.width(5)
				.border_left(1)
				.border_color(C_BG_SIDE_BORDER)
				.hover(|s| s.border_color(C_FOCUS).cursor(CursorStyle::ColResize))
		})
		.draggable()
		.dragging_style(|s| s.border_color(C_FOCUS)) // this doesn't seem to work?
		.on_event(EventListener::DragStart, move |_| {
			is_sidebar_dragging.set(true);
			EventPropagation::Continue
		})
		.on_event(EventListener::DragEnd, move |_| {
			is_sidebar_dragging.set(false);
			EventPropagation::Continue
		})
		.on_event(EventListener::DoubleClick, move |_| {
			sidebar_width.set(SIDEBAR_WIDTH);
			EventPropagation::Continue
		});

	let main_window = scroll(
		tab(
			move || {
				list
					.get()
					.iter()
					.position(|item| item.0 == active_tab.get())
					.unwrap_or(0)
			},
			move || list.get(),
			move |it| *it,
			|it| detail_view(it.0),
		)
		.style(|s| {
			s.flex_col()
				.items_start()
				.padding_bottom(10.0)
				.min_width(450)
				.width_full()
		}),
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
			.class(scroll::Handle, styles::scrollbar_styles)
	});

	let tooltip = label(move || tooltip_text.get()).style(move |s| {
		s.position(Position::Absolute)
			.z_index(11)
			.inset_left(tooltip_pos.get().0)
			.inset_top(tooltip_pos.get().1)
			.display(Display::None)
			.apply_if(tooltip_visible.get(), |s| s.display(Display::Flex))
			.background(C_BG_MAIN)
			.color(C_TEXT_MAIN)
			.padding(3.0)
			.padding_bottom(4.0)
			.padding_left(4.0)
			.padding_right(4.0)
			.border_radius(3)
			.box_shadow_blur(8)
			.box_shadow_color(C_SHADOW_2)
			.box_shadow_spread(-3)
			.border_color(C_BG_MAIN_BORDER)
			.border(1)
	});

	let content =
		h_stack((sidebar, shadow_box_top, shadow_box_right, dragger, main_window))
			.style(|s| {
				s.position(Position::Absolute)
					.inset_top(SEARCHBAR_HEIGHT)
					.inset_bottom(0.0)
					.width_full()
			});

	let view = v_stack((tooltip, search_bar, content))
		.style(|s| s.width_full().height_full())
		.on_event(EventListener::PointerMove, move |event| {
			let pos = match event {
				Event::PointerMove(p) => p.pos,
				_ => (0.0, 0.0).into(),
			};
			mouse_pos.set((pos.x, pos.y));
			if is_sidebar_dragging.get() {
				sidebar_width.set(pos.x);
			}
			EventPropagation::Continue
		})
		.on_resize(move |event| {
			window_size.set((event.x1, event.y1));
		});

	match std::env::var("DEBUG") {
		Ok(_) => {
			// for debugging the layout
			let id = view.id();
			view.on_event_stop(EventListener::KeyUp, move |e| {
				if let floem::event::Event::KeyUp(e) = e {
					if e.key.logical_key
						== floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11)
					{
						id.inspect();
					}
				}
			})
		}
		Err(_) => view,
	}
}
