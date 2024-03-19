use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	kurbo::Size,
	peniko::Color,
	reactive::{create_rw_signal, provide_context, use_context, RwSignal},
	style::{CursorStyle, Display, Position},
	view::View,
	views::{
		container, dyn_container, h_stack, label, scroll, v_stack, virtual_stack,
		Decorators, VirtualDirection, VirtualItemSize,
	},
};

use crate::{
	config::{PresetFields, WindowSettings},
	env::Environment,
	ui::{
		colors::*,
		details::detail_view::{detail_view, DETAILS_MIN_WIDTH},
		primitives::{
			button::{icon_button, IconButton},
			input_button_field::{input_button_field, InputButtonField},
			que::Que,
			styles,
			toast::{toast_view, ToastSignals},
			tooltip::{tooltip_view, TooltipSignals},
		},
		settings::settings_view::settings_view,
		window_management::{opening_window, WindowSpec},
	},
	AppState,
};

const SEARCHBAR_HEIGHT: f64 = 30.0;

pub type SidebarList = RwSignal<im::Vector<(usize, &'static str, usize)>>;
pub type PresetFieldSignal = RwSignal<PresetFields>;

#[derive(Debug, Copy, Clone)]
pub struct QueSettings {
	pub inner: Que,
}

#[derive(Debug, Copy, Clone)]
pub struct TooltipSignalsSettings {
	pub inner: TooltipSignals,
}

#[derive(Debug, Copy, Clone)]
pub struct ToastSignalsSettings {
	pub inner: ToastSignals,
}

pub fn app_view() -> impl View {
	let env = use_context::<Environment>().expect("No env context provider");
	let que = use_context::<Que>().expect("No que context provider");
	let tooltip_signals = use_context::<TooltipSignals>()
		.expect("No tooltip_signals context provider");
	let toast_signals =
		use_context::<ToastSignals>().expect("No toast_signals context provider");

	let list_sidebar_signal: SidebarList =
		create_rw_signal(env.db.get_sidebar_list());

	provide_context(list_sidebar_signal);
	let field_presets: PresetFieldSignal =
		create_rw_signal(env.config.get_field_presets());
	provide_context(field_presets);

	let env_search_reset = env.clone();
	let config_sidebar_drag = env.config.clone();
	let config_sidebar_double_click = env.config.clone();
	let db_lock_button = env.db.clone();

	let sidebar_width =
		create_rw_signal(env.config.general.read().window_settings.sidebar_width);
	let is_sidebar_dragging = create_rw_signal(false);
	let active_tab =
		create_rw_signal(list_sidebar_signal.get().get(0).unwrap_or(&(0, "", 0)).0);
	let search_text = create_rw_signal(String::from(""));
	let sidebar_scrolled = create_rw_signal(false);
	let main_scroll_to = create_rw_signal(0.0);

	let que_settings = QueSettings {
		inner: Que::default(),
	};
	let tooltip_signals_settings = TooltipSignalsSettings {
		inner: TooltipSignals::new(que_settings.inner),
	};

	provide_context(que_settings);
	provide_context(tooltip_signals_settings);

	let overflow_labels = create_rw_signal(vec![0]);

	let delete_icon = include_str!("./icons/delete.svg");
	let icon = create_rw_signal(String::from(""));
	let settings_icon = include_str!("./icons/settings.svg");
	let lock_icon = include_str!("./icons/lock.svg");

	let search_text_input_view = input_button_field(
		InputButtonField {
			value: search_text,
			icon,
			placeholder: "Press enter to create a new entry",
			tooltip: String::from("Empty search"),
			tooltip_signals,
		},
		move || {
			icon.set(String::from(""));
			search_text.set(String::from(""));
			list_sidebar_signal.update(
				|list: &mut im::Vector<(usize, &'static str, usize)>| {
					*list = env_search_reset
						.db
						.get_sidebar_list()
						.iter()
						.map(|entries| (entries.0, entries.1, entries.2))
						.collect();
				},
			);
		},
	);
	let search_text_input_view_id = search_text_input_view.input_id;

	let search_bar = h_stack((
		label(|| "Search / Create:")
			.on_click_stop(move |_| {
				search_text_input_view_id.request_focus();
			})
			.style(|s| {
				s.font_size(12.0).padding(3.0).padding_left(10.0).color(C_TOP_TEXT)
			}),
		search_text_input_view
			.on_event_cont(EventListener::KeyUp, move |event| {
				if search_text.get().is_empty() {
					icon.set(String::from(""));
				} else {
					icon.set(String::from(delete_icon));
				}

				let key = match event {
					Event::KeyUp(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if key == PhysicalKey::Code(KeyCode::Enter) {
					{
						env.db.add(search_text.get());
						let _ = env.db.save();
					}

					let search_list = env.db.get_sidebar_list();
					active_tab.set(search_list[0].0);
					list_sidebar_signal.set(search_list);
					search_text.set(String::from(""));
					icon.set(String::from(""));
				} else {
					list_sidebar_signal.update(|list| {
						*list = env
							.db
							.get_sidebar_list()
							.iter()
							.cloned()
							.filter(|item| {
								item
									.1
									.to_lowercase()
									.contains(&search_text.get().to_lowercase())
							})
							.collect::<im::Vector<_>>();
					});
				}
			})
			.style(|s| s.flex_grow(1.0)),
		icon_button(
			IconButton {
				icon: String::from(lock_icon),
				tooltip: String::from("Lock Vault"),
				tooltip_signals,
				..IconButton::default()
			},
			move |_| {
				let app_state = use_context::<RwSignal<AppState>>()
					.expect("No app_state context provider");

				que.unque_all_tooltips();
				db_lock_button.clear_hash();
				*db_lock_button.vault_unlocked.write() = false;
				app_state.set(AppState::PassPrompting);
			},
		),
		icon_button(
			IconButton {
				icon: String::from(settings_icon),
				tooltip: String::from("Vault Settings"),
				tooltip_signals,
				..IconButton::default()
			},
			move |_| {
				opening_window(
					settings_view,
					WindowSpec {
						id: String::from("settings-window"),
						title: String::from("Vault Settings"),
					},
					Size::new(500.0, 400.0),
					move || {
						que_settings.inner.unque_all_tooltips();
					},
				);
			},
		),
	))
	.style(|s| {
		s.z_index(3)
			.items_center()
			.width_full()
			.height(SEARCHBAR_HEIGHT)
			.background(C_TOP_BG)
			.gap(3.0, 0.0)
			.padding_right(3)
	});

	let sidebar = scroll({
		virtual_stack(
			VirtualDirection::Vertical,
			VirtualItemSize::Fixed(Box::new(|| 21.0)),
			move || list_sidebar_signal.get(),
			move |item| *item,
			move |item| {
				container(
					label(move || item.1)
						.style(|s| s.font_size(12.0).color(C_SIDE_TEXT))
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
						.on_event_cont(EventListener::PointerEnter, move |_| {
							let labels = overflow_labels.get();
							if labels.contains(&item.0) {
								tooltip_signals.show(String::from(item.1));
							}
						})
						.on_event_cont(EventListener::PointerLeave, move |_| {
							tooltip_signals.hide();
						})
						.on_click_stop(move |_| {
							active_tab.set(item.0);
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
								.border_color(C_SIDE_BG_BORDER)
								.color(C_SIDE_TEXT)
								.focus_visible(|s| s.border(1).border_color(C_FOCUS))
								.background(if let 0 = item.2 % 2 {
									C_SIDE_BG
								} else {
									C_SIDE_BG_SELECTED.with_alpha_factor(0.2)
								})
								.apply_if(item.0 == active_tab.get(), |s| {
									s.background(C_SIDE_BG_SELECTED)
								})
								.hover(|s| {
									s.background(C_SIDE_BG_SELECTED.with_alpha_factor(0.6))
										.apply_if(item.0 == active_tab.get(), |s| {
											s.background(C_SIDE_BG_SELECTED)
										})
										.cursor(CursorStyle::Pointer)
								})
						}),
				)
			},
		)
		.style(move |s| {
			s.flex_col().width(sidebar_width.get() - 1.0).background(C_SIDE_BG)
		})
	})
	.on_scroll(move |x| {
		tooltip_signals.hide();
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
			.border_color(C_SIDE_BG_BORDER)
			.background(C_SIDE_BG)
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
				.border_color(C_SIDE_BG_BORDER)
				.hover(|s| s.border_color(C_FOCUS).cursor(CursorStyle::ColResize))
				.apply_if(is_sidebar_dragging.get(), |s| s.border_color(C_FOCUS))
		})
		.draggable()
		.dragging_style(|s| s.border_color(Color::TRANSPARENT))
		.on_event_cont(EventListener::DragStart, move |_| {
			is_sidebar_dragging.set(true);
		})
		.on_event_cont(EventListener::DragEnd, move |_| {
			is_sidebar_dragging.set(false);
			config_sidebar_drag.set_sidebar_width(sidebar_width.get());
		})
		.on_event_cont(EventListener::DoubleClick, move |_| {
			let default_window_size = WindowSettings::default();
			sidebar_width.set(default_window_size.sidebar_width);
			config_sidebar_double_click
				.set_sidebar_width(default_window_size.sidebar_width);
		});

	let main_window = scroll(
		dyn_container(
			move || active_tab.get(),
			move |id| detail_view(id, main_scroll_to).any(),
		)
		.style(|s| {
			s.flex_col()
				.items_start()
				.padding_bottom(10.0)
				.min_width(DETAILS_MIN_WIDTH)
				.width_full()
		}),
	)
	.on_scroll(move |_| {
		tooltip_signals.hide();
	})
	.scroll_to_percent(move || {
		main_scroll_to.track();
		main_scroll_to.get()
	})
	.style(|s| {
		s.flex_col()
			.flex_basis(0)
			.min_width(0)
			.flex_grow(1.0)
			.background(C_MAIN_BG)
			.border_top(1.0)
			.border_color(C_TOP_BG_BORDER)
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

	v_stack((
		tooltip_view(tooltip_signals),
		toast_view(toast_signals),
		search_bar,
		content,
	))
	.style(|s| s.width_full().height_full())
	.on_event_cont(EventListener::PointerMove, move |event| {
		let pos = match event {
			Event::PointerMove(p) => p.pos,
			_ => (0.0, 0.0).into(),
		};
		tooltip_signals.mouse_pos.set((pos.x, pos.y));
		if is_sidebar_dragging.get() {
			sidebar_width.set(pos.x);
		}
	})
}
