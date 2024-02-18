use floem::{
	event::{Event, EventListener},
	reactive::{create_signal, RwSignal},
	style::Position,
	view::View,
	views::{container, h_stack, scroll, tab, v_stack, Decorators},
	EventPropagation,
};

use crate::{
	config::PresetFields,
	env::Environment,
	ui::{
		colors::*,
		primitives::{
			button::tab_button,
			que::Que,
			styles,
			toast::{toast_view, ToastSignals},
			tooltip::{tooltip_view, TooltipSignals},
		},
		settings::{
			database::database_view, editing::editing_view, general::general_view,
			shortcut::shortcut_view,
		},
	},
	AppState,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Tabs {
	General,
	Editing,
	Database,
	Shortcuts,
}

impl std::fmt::Display for Tabs {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Tabs::General => write!(f, "General"),
			Tabs::Editing => write!(f, "Editing"),
			Tabs::Database => write!(f, "Database"),
			Tabs::Shortcuts => write!(f, "Shortcuts"),
		}
	}
}

pub const TABBAR_HEIGHT: f64 = 63.0;

pub fn settings_view(
	field_presets: RwSignal<PresetFields>,
	timeout_que_id: RwSignal<u8>,
	app_state: RwSignal<AppState>,
	que: Que,
	tooltip_signals: TooltipSignals,
	env: Environment,
) -> impl View {
	let tabs = vec![
		Tabs::General,
		Tabs::Editing,
		Tabs::Database,
		Tabs::Shortcuts,
	]
	.into_iter()
	.collect::<im::Vector<Tabs>>();
	let (tabs, _set_tabs) = create_signal(tabs);
	let (active_tab, set_active_tab) = create_signal(0);

	let settings_icon = include_str!("../icons/settings.svg");
	let editing_icon = include_str!("../icons/editing.svg");
	let database_icon = include_str!("../icons/database.svg");
	let shortcut_icon = include_str!("../icons/shortcut.svg");

	let toast_signals = ToastSignals::new(que);

	let tabs_bar = h_stack((
		tab_button(
			String::from(settings_icon),
			Tabs::General,
			tabs,
			set_active_tab,
			active_tab,
		),
		tab_button(
			String::from(editing_icon),
			Tabs::Editing,
			tabs,
			set_active_tab,
			active_tab,
		),
		tab_button(
			String::from(database_icon),
			Tabs::Database,
			tabs,
			set_active_tab,
			active_tab,
		),
		tab_button(
			String::from(shortcut_icon),
			Tabs::Shortcuts,
			tabs,
			set_active_tab,
			active_tab,
		),
	))
	.style(|s| {
		s.flex_row()
			.width_full()
			.height(TABBAR_HEIGHT)
			.gap(5, 0)
			.padding(5)
			.border_bottom(1)
			.border_color(C_BG_TOP_BORDER)
			.background(C_BG_TOP)
	});

	let main_content = container(
		scroll(
			tab(
				move || active_tab.get(),
				move || tabs.get(),
				|it| *it,
				move |it| {
					let env_settings = env.clone();
					match it {
						Tabs::General => {
							general_view(tooltip_signals, toast_signals, env_settings)
								.any()
								.style(|s| s.padding(8.0).padding_bottom(10.0))
						},
						Tabs::Editing => {
							editing_view(field_presets, tooltip_signals, env_settings)
								.any()
								.style(|s| s.padding(8.0).padding_bottom(10.0))
						},
						Tabs::Database => database_view(
							timeout_que_id,
							app_state,
							que,
							tooltip_signals,
							env_settings,
						)
						.any()
						.style(|s| s.padding(8.0).padding_bottom(10.0)),
						Tabs::Shortcuts => shortcut_view(tooltip_signals, env_settings)
							.any()
							.style(|s| s.padding(8.0).padding_bottom(10.0)),
					}
				},
			)
			.style(|s| s.flex_col().items_start().margin_top(10)),
		)
		.on_scroll(move |_| {
			tooltip_signals.hide();
		})
		.style(|s| {
			s.flex_col()
				.flex_basis(0)
				.min_width(0)
				.flex_grow(1.0)
				.background(C_BG_MAIN)
				.class(scroll::Handle, styles::scrollbar_styles)
		}),
	)
	.style(|s| {
		s.position(Position::Absolute)
			.inset_top(TABBAR_HEIGHT)
			.inset_bottom(0.0)
			.width_full()
	});

	let settings_view = v_stack((
		tooltip_view(tooltip_signals),
		toast_view(toast_signals),
		tabs_bar,
		main_content,
	))
	.style(|s| s.width_full().height_full().gap(0, 5))
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
			let id = settings_view.id();
			settings_view.on_event_stop(EventListener::KeyUp, move |e| {
				if let floem::event::Event::KeyUp(e) = e {
					if e.key.logical_key
						== floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11)
					{
						id.inspect();
					}
				}
			})
		},
		Err(_) => settings_view,
	}
}
