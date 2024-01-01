use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	kurbo::Size,
	peniko::Color,
	reactive::{create_rw_signal, create_signal},
	style::{CursorStyle, Display, Position},
	view::View,
	views::{
		container, dyn_container, h_stack, label, scroll, svg, v_stack,
		virtual_list, Decorators, VirtualListDirection, VirtualListItemSize,
	},
	EventPropagation,
};

use crate::{
	config::Config,
	ui::{
		colors::*,
		detail_view::detail_view,
		primitives::{
			button::icon_button,
			input_field::input_field,
			styles,
			tooltip::{tooltip_view, TooltipSignals},
		},
		settings_view::settings_view,
		window_management::{make_settings_path, opening_window},
	},
};

const SIDEBAR_WIDTH: f64 = 140.0;
const SEARCHBAR_HEIGHT: f64 = 30.0;

pub fn app_view(config: Config) -> impl View {
	let db = config.db.read().unwrap().get_list();
	let db_backup = config.db.read().unwrap().get_list();
	let config_search = config.clone();

	let sidebar_width = create_rw_signal(SIDEBAR_WIDTH);
	let is_sidebar_dragging = create_rw_signal(false);
	let (list, set_list) = create_signal(db.clone());
	let (active_tab, set_active_tab) = create_signal(db[0].0);
	let search_text = create_rw_signal(String::from(""));
	let sidebar_scrolled = create_rw_signal(false);
	let main_scroll_to = create_rw_signal(0.0);

	let tooltip_signals = TooltipSignals::new();
	let overflow_labels = create_rw_signal(vec![0]);

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
							*list = db
								.iter()
								.cloned()
								.filter(|item| {
									item
										.1
										.to_lowercase()
										.contains(&search_text.get().to_lowercase())
								})
								.collect::<im::Vector<_>>();
						},
					);
					EventPropagation::Continue
				})
				.on_event(EventListener::KeyUp, move |event| {
					let key = match event {
						Event::KeyUp(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};

					if key == PhysicalKey::Code(KeyCode::Enter) {
						{
							config_search.clone().db.write().unwrap().add(search_text.get());
						}

						let new_list = config_search.db.read().unwrap().get_list();
						let list_copy = new_list.clone();
						set_list.update(
							|list: &mut im::Vector<(usize, &'static str, usize)>| {
								*list = new_list;
							},
						);
						search_text.set(String::from(""));
						set_active_tab.update(|v: &mut usize| {
							*v = list_copy[0].0;
						});
					}
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
				*list = db_backup
					.iter()
					.map(|entries| (entries.0, entries.1, entries.2))
					.collect();
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
			opening_window(
				settings_view,
				make_settings_path(),
				String::from("Vault Settings"),
				Size::new(430.0, 400.0),
				|| {},
			);
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
						.on_text_overflow(move |is_overflown| {
							let mut labels = overflow_labels.get();
							if is_overflown {
								labels.push(item.0);
							} else {
								labels.retain(|i| *i != item.0);
							}
							overflow_labels.set(labels);
						})
						.on_event(EventListener::PointerEnter, move |_event| {
							let labels = overflow_labels.get();
							if labels.contains(&item.0) {
								tooltip_signals.show(String::from(item.1));
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
							main_scroll_to.set(0.0);
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
		dyn_container(
			move || active_tab.get(),
			move |id| {
				Box::new(detail_view(
					id,
					main_scroll_to,
					tooltip_signals,
					set_list,
					list,
					config.clone(),
				))
			},
		)
		.style(|s| {
			s.flex_col()
				.items_start()
				.padding_bottom(10.0)
				.min_width(450)
				.width_full()
		}),
	)
	.scroll_to_percent(move || {
		main_scroll_to.track();
		main_scroll_to.get()
	})
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

	let content =
		h_stack((sidebar, shadow_box_top, shadow_box_right, dragger, main_window))
			.style(|s| {
				s.position(Position::Absolute)
					.inset_top(SEARCHBAR_HEIGHT)
					.inset_bottom(0.0)
					.width_full()
			});

	v_stack((tooltip_view(tooltip_signals), search_bar, content))
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
		})
}
