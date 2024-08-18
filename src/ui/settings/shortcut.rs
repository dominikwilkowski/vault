use floem::{
	event::{Event, EventListener},
	keyboard::PhysicalKey,
	peniko::Brush,
	reactive::{create_rw_signal, use_context, RwSignal},
	style::Display,
	views::{container, empty, label, Decorators},
	IntoView,
};

use crate::{
	config::Shortcuts,
	env::Environment,
	ui::{
		app_view::TooltipSignalsSettings,
		colors::*,
		keyboard::{
			keycode_to_key, modifiersstate_to_keymodifier, Key, KeyModifier,
		},
		primitives::{
			button::{button, icon_button, IconButton},
			styles,
			tooltip::TooltipSignals,
		},
	},
};

fn keyboard_capture(
	shortcut: RwSignal<(Key, KeyModifier)>,
	dirty_state: RwSignal<bool>,
	tooltip_signals: TooltipSignals,
) -> impl IntoView {
	(
		label(move || format!("{:?}", shortcut.get().1)).style(styles::tag).style(|s| s.selectable(false)),
		label(move || format!("{:?}", shortcut.get().0)).style(styles::tag).style(|s| s.selectable(false)),
	)
		.on_event_cont(EventListener::PointerEnter, move |_| {
			tooltip_signals.show(String::from("Capture a new shortcut by selecting\nthis field and pressing the new keys"));
		})
		.on_event_cont(EventListener::PointerLeave, move |_| {
			tooltip_signals.hide();
		})
		.keyboard_navigatable()
		.on_event_cont(EventListener::KeyUp, move |event| {
			let key = match event {
				Event::KeyUp(k) => match k.key.physical_key {
					PhysicalKey::Code(code) => keycode_to_key(code),
					_ => Key::F35,
				},
				_ => Key::F35,
			};

			let modifier = match event {
				Event::KeyUp(k) => modifiersstate_to_keymodifier(k.modifiers),
				_ => KeyModifier::None,
			};

			// letting go of a key combination causes two events and we need only the
			// the one that captures both, so we ignore events that register only
			// modifier keys
			let key_string = format!("{:?}", key);
			if !key_string.starts_with("Shift") &&
				!key_string.starts_with("Control") &&
				!key_string.starts_with("Alt") &&
				!key_string.starts_with("Super") {
				shortcut.set((key, modifier));
				dirty_state.set(true);
			}
		})
		.style(|s| {
			s.min_width(132)
				.row_gap(5)
				.height(30)
				.items_center()
				.padding(4)
				.border(1)
				.border_radius(2)
				.border_color(C_TOP_TEXT)
				.cursor_color(Brush::Solid(C_FOCUS.with_alpha_factor(0.5)))
				.hover(|s| s.background(C_FOCUS.with_alpha_factor(0.05)))
				.focus(|s| s.border_color(C_FOCUS).outline_color(C_FOCUS).background(C_FOCUS.with_alpha_factor(0.05)))
				.focus_visible(|s| s.outline(1))
		})
}

pub fn shortcut_view() -> impl IntoView {
	let env = use_context::<Environment>().expect("No env context provider");
	let tooltip_signals = use_context::<TooltipSignalsSettings>()
		.expect("No tooltip_signals context provider")
		.inner;

	let lock_shortcut =
		create_rw_signal(env.config.general.read().shortcuts.lock.clone());
	let search_shortcut =
		create_rw_signal(env.config.general.read().shortcuts.search.clone());
	let settings_shortcut =
		create_rw_signal(env.config.general.read().shortcuts.settings.clone());
	let dirty_state = create_rw_signal(false);

	let revert_icon = include_str!("../icons/revert.svg");

	let env_reset = env.clone();

	container(
		(
			"Lock the app",
			keyboard_capture(lock_shortcut, dirty_state, tooltip_signals),
			"Start search",
			keyboard_capture(search_shortcut, dirty_state, tooltip_signals),
			"Open settings",
			keyboard_capture(settings_shortcut, dirty_state, tooltip_signals),
			empty(),
			(
				icon_button(
					IconButton {
						icon: String::from(revert_icon),
						tooltip: String::from("Reset shortcuts"),
						tooltip_signals,
						..IconButton::default()
					},
					move |_| {
						tooltip_signals.hide();
						lock_shortcut
							.set(env_reset.config.general.read().shortcuts.lock.clone());
						search_shortcut
							.set(env_reset.config.general.read().shortcuts.search.clone());
						settings_shortcut
							.set(env_reset.config.general.read().shortcuts.settings.clone());
						dirty_state.set(false);
					},
				),
				button("Save").on_click_cont(move |_| {
					env.config.edit_shortcuts(Shortcuts {
						lock: lock_shortcut.get(),
						search: search_shortcut.get(),
						settings: settings_shortcut.get(),
					});
					dirty_state.set(false);
				}),
			)
				.style(move |s| {
					s.row_gap(5)
						.display(Display::None)
						.apply_if(dirty_state.get(), |s| s.display(Display::Flex))
				}),
		)
			.style(|s| s.margin_bottom(120))
			.style(styles::settings_line),
	)
}
