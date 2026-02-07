use floem::{
	event::{Event, EventListener},
	kurbo::Size,
	peniko::Color,
	reactive::{
		Context, Effect, RwSignal,
		SignalGet, SignalTrack, SignalUpdate, Trigger,
	},
	style::{CursorStyle, Display, Position},
	ui_events::{keyboard::KeyState, pointer::PointerEvent},
	views::{
		Scroll,
		Container, dyn_container, Empty, Label, virtual_stack,
		Decorators,
	},
	IntoView,
};

use crate::{
	config::{PresetFields, WindowSettings},
	env::Environment,
	lock_app,
	ui::{
		colors::*,
		details::detail_view::{detail_view, DETAILS_MIN_WIDTH},
		keyboard::is_submit,
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
};

const SEARCHBAR_HEIGHT: f64 = 30.0;

pub type SidebarList = RwSignal<imbl::Vector<(usize, String, usize)>>;
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

pub fn app_view(search_trigger: Trigger) -> impl IntoView {
	let env = Context::get::<Environment>().expect("No env context provider");
	let tooltip_signals = Context::get::<TooltipSignals>()
		.expect("No tooltip_signals context provider");
	let toast_signals =
		Context::get::<ToastSignals>().expect("No toast_signals context provider");

	let list_sidebar_signal: SidebarList =
		RwSignal::new(env.db.get_sidebar_list());

	Context::provide(list_sidebar_signal);
	let field_presets: PresetFieldSignal =
		RwSignal::new(env.config.get_field_presets());
	Context::provide(field_presets);

	let env_search_reset = env.clone();
	let config_sidebar_drag = env.config.clone();
	let config_sidebar_double_click = env.config.clone();

	let sidebar_width =
		RwSignal::new(env.config.general.read().window_settings.sidebar_width);
	let is_sidebar_dragging = RwSignal::new(false);
	let active_tab = RwSignal::new(
		list_sidebar_signal.get().get(0).unwrap_or(&(0, String::from(""), 0)).0,
	);
	let search_text = RwSignal::new(String::from(""));
	let sidebar_scrolled = RwSignal::new(false);
	let main_scroll_to = RwSignal::new(0.0);

	let que_settings = QueSettings {
		inner: Que::default(),
	};
	let tooltip_signals_settings = TooltipSignalsSettings {
		inner: TooltipSignals::new(que_settings.inner),
	};

	Context::provide(que_settings);
	Context::provide(tooltip_signals_settings);

	let overflow_labels = RwSignal::new(vec![0]);

	let delete_icon = include_str!("./icons/delete.svg");
	let icon = RwSignal::new(String::from(""));
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
				|list: &mut imbl::Vector<(usize, String, usize)>| {
					*list = env_search_reset
						.db
						.get_sidebar_list()
						.iter()
						.map(|entries| (entries.0, entries.1.clone(), entries.2))
						.collect();
				},
			);
		},
	);
	let search_text_input_view_id = search_text_input_view.input_id;

	Effect::new(move |_| {
		search_trigger.track();
		search_text_input_view_id.request_focus();
	});

	let search_bar = (
		"Search / Create:"
			.on_click_stop(move |_| {
				search_text_input_view_id.request_focus();
			})
			.style(|s| {
				s.font_size(12.0)
					.padding(3.0)
					.padding_left(10.0)
					.color(C_TOP_TEXT)
					.selectable(false)
			}),
		search_text_input_view
			.on_event_cont(EventListener::KeyDown, move |event| {
				if search_text.get().is_empty() {
					icon.set(String::from(""));
				} else {
					icon.set(String::from(delete_icon));
				}

				let code = match event {
					Event::Key(k) if k.state == KeyState::Down => k.code,
					_ => floem::ui_events::keyboard::Code::F35,
				};

				if is_submit(code) && !search_text.get().is_empty() {
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
						*list = env.db.search(&search_text.get());
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
				lock_app();
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
					false,
					move || {
						que_settings.inner.unque_all_tooltips();
					},
				);
			},
		),
	)
		.style(|s| {
			s.z_index(3)
				.items_center()
				.width_full()
				.height(SEARCHBAR_HEIGHT)
				.background(C_TOP_BG)
				.row_gap(3)
				.padding_right(3)
		});

	let sidebar = Scroll::new({
			VirtualDirection::Vertical,
			VirtualItemSize::Fixed(Box::new(|| 21.0)),
		virtual_stack(
			move || list_sidebar_signal.get(),
			move |item| item.clone(),
			move |item| {
				let title = item.1.clone();
				Container::new(
					Label::derived(move || item.1.clone())
						.style(|s| s.font_size(12.0).color(C_SIDE_TEXT))
						.style(|s| s.focusable(true))
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
								tooltip_signals.show(title.clone());
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
									C_SIDE_BG_SELECTED.multiply_alpha(0.2)
								})
								.apply_if(item.0 == active_tab.get(), |s| {
									s.background(C_SIDE_BG_SELECTED)
								})
								.hover(|s| {
									s.background(C_SIDE_BG_SELECTED.multiply_alpha(0.6))
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
	});

	let shadow_box_top = Empty::new().style(move |s| {
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

	let shadow_box_right = Empty::new().style(move |s| {
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

	let dragger = Empty::new()
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
		.style(|s| s.draggable(true))
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

	let main_window = Scroll::new(
		dyn_container(
			move || active_tab.get(),
			move |active_tab| detail_view(active_tab, main_scroll_to).into_any(),
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
	});

	let content =
		(sidebar, shadow_box_top, shadow_box_right, dragger, main_window).style(
			|s| {
				s.position(Position::Absolute)
					.inset_top(SEARCHBAR_HEIGHT)
					.inset_bottom(0.0)
					.width_full()
			},
		);

	(
		tooltip_view(tooltip_signals),
		toast_view(toast_signals),
		search_bar,
		content,
	)
		.style(|s| s.flex_col().width_full().height_full())
		.style(styles::default_window_styles)
		.on_event_cont(EventListener::PointerMove, move |event| {
			let pos = match event {
				Event::Pointer(PointerEvent::Move(pu)) => pu.current.logical_point(),
				_ => (0.0, 0.0).into(),
			};
			tooltip_signals.mouse_pos.set((pos.x, pos.y));
			if is_sidebar_dragging.get() {
				sidebar_width.set(pos.x);
			}
		})
}
