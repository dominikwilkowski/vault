use floem::{
	event::{Event, EventListener},
	kurbo::Size,
	peniko::Color,
	reactive::{create_rw_signal, create_signal},
	style::{CursorStyle, Display, Position},
	view::View,
	views::{
		container, h_stack, label, scroll, svg, tab, v_stack, virtual_list,
		Decorators, VirtualListDirection, VirtualListItemSize,
	},
	window::{new_window, WindowConfig},
	EventPropagation,
};

use core::cell::Cell;

use crate::db::get_db_list;
use crate::ui::{
	colors::*,
	detail_view::detail_view,
	primitives::{
		button::icon_button,
		input_field::input_field,
		styles,
		tooltip::{tooltip_view, TooltipSignals},
	},
	settings_view::settings_view,
};

const SIDEBAR_WIDTH: f64 = 140.0;
const SEARCHBAR_HEIGHT: f64 = 30.0;

thread_local! {
	pub(crate) static SETTINGS_WINDOW_OPEN: Cell<bool> = Cell::new(false);
}

pub fn app_view() -> impl View {
	let db = get_db_list();

	let sidebar_width = create_rw_signal(SIDEBAR_WIDTH);
	let is_sidebar_dragging = create_rw_signal(false);
	let (list, set_list) = create_signal(db.clone());
	let (active_tab, set_active_tab) = create_signal(db[0].0);
	let search_text = create_rw_signal(String::from(""));
	let sidebar_scrolled = create_rw_signal(false);

	let tooltip_signals = TooltipSignals::new();

	let clear_icon = include_str!("./icons/clear.svg");
	let settings_icon = include_str!("./icons/settings.svg");

	let search_text_input_view = input_field(search_text, |s| {
		s.width_full().padding_right(30).margin_top(3).margin_bottom(3)
	});
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
		container(
			svg(move || clear_icon.to_string()).style(|s| s.height(16.0).width(16.0)),
		)
		.on_click_stop(move |_| {
			search_text.set(String::from(""));
			set_list.update(|list: &mut im::Vector<(usize, &'static str, usize)>| {
				*list = get_db_list();
			});
		})
		.keyboard_navigatable()
		.style(move |s| {
			s.position(Position::Absolute)
				.items_center()
				.justify_center()
				.height(30.0)
				.width(30.0)
				.display(Display::None)
				.z_index(5)
				.inset_top(0)
				.inset_right(29)
				.cursor(CursorStyle::Pointer)
				.hover(|s| s.cursor(CursorStyle::Pointer))
				.apply_if(!search_text.get().is_empty(), |s| s.display(Display::Flex))
		}),
		icon_button(String::from(settings_icon), create_rw_signal(true), |_| {
			if !SETTINGS_WINDOW_OPEN.get() {
				SETTINGS_WINDOW_OPEN.set(true);
				new_window(
					settings_view,
					Some(
						WindowConfig::default()
							.size(Size::new(430.0, 400.0))
							.title("Vault Settings"),
					),
				);
			}
		}),
	))
	.style(|s| {
		s.z_index(3)
			.width_full()
			.height(SEARCHBAR_HEIGHT)
			.background(C_BG_TOP)
			.gap(3.0, 0.0)
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
								tooltip_signals.show(item.1);
							}
							EventPropagation::Continue
						})
						.on_event(EventListener::PointerLeave, move |_| {
							tooltip_signals.hide();
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
		tooltip_signals.tooltip_visible.set(false);
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
				.width(10)
				.border_left(1)
				.border_color(C_BG_SIDE_BORDER)
				.hover(|s| s.border_color(C_FOCUS).cursor(CursorStyle::ColResize))
				.apply_if(is_sidebar_dragging.get(), |s| s.border_color(C_FOCUS))
		})
		.draggable()
		.dragging_style(|s| s.border_color(Color::TRANSPARENT))
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
			move |it| detail_view(it.0, tooltip_signals),
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

	let tooltip = tooltip_view(tooltip_signals);

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
			tooltip_signals.mouse_pos.set((pos.x, pos.y));
			if is_sidebar_dragging.get() {
				sidebar_width.set(pos.x);
			}
			EventPropagation::Continue
		})
		.on_resize(move |event| {
			tooltip_signals.window_size.set((event.x1, event.y1));
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
