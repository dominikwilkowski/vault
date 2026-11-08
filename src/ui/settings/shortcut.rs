use floem::{
	event::listener,
	peniko::Brush,
	reactive::{RwSignal, SignalGet, SignalUpdate},
	style::Display,
	views::{Container, Decorators, Empty, Label},
	IntoView,
};

use crate::{
	config::Shortcuts,
	env::Environment,
	ui::shared_context,
	ui::{
		app_view::TooltipSignalsSettings,
		colors::*,
		focus::has_focus,
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
		Label::derived(move || format!("{:?}", shortcut.get().1)).style(styles::tag).style(|s| s.selectable(false)),
		Label::derived(move || format!("{:?}", shortcut.get().0)).style(styles::tag).style(|s| s.selectable(false)),
	)
		.on_event_cont(listener::PointerEnter, move |_, _| {
			tooltip_signals.show(String::from("Capture a new shortcut by selecting\nthis field and pressing the new keys"));
		})
		.on_event_cont(listener::PointerLeave, move |_, _| {
			tooltip_signals.hide();
		})
		.style(|s| s.keyboard_navigable())
		.on_event_cont(listener::KeyUp, move |cx, event| {
			if !has_focus(cx) {
				return;
			}

			let key = keycode_to_key(event.code);
			let modifier = modifiersstate_to_keymodifier(event.modifiers);

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
				.col_gap(5)
				.height(30)
				.items_center()
				.padding(4)
				.border(1)
				.border_radius(2)
				.border_color(C_TOP_TEXT)
				.cursor_color(Brush::Solid(C_FOCUS.multiply_alpha(0.5)))
				.hover(|s| s.background(C_FOCUS.multiply_alpha(0.05)))
				.focus(|s| s.border_color(C_FOCUS).outline_color(C_FOCUS).background(C_FOCUS.multiply_alpha(0.05)))
				.focus_visible(|s| s.outline(1))
		})
}

pub fn shortcut_view() -> impl IntoView {
	let env =
		shared_context::get::<Environment>().expect("No env context provider");
	let tooltip_signals = shared_context::get::<TooltipSignalsSettings>()
		.expect("No tooltip_signals context provider")
		.inner;

	let lock_shortcut =
		RwSignal::new(env.config.general.read().shortcuts.lock.clone());
	let search_shortcut =
		RwSignal::new(env.config.general.read().shortcuts.search.clone());
	let settings_shortcut =
		RwSignal::new(env.config.general.read().shortcuts.settings.clone());
	let dirty_state = RwSignal::new(false);

	let revert_icon = include_str!("../icons/revert.svg");

	let env_reset = env.clone();

	Container::new(
		(
			"Lock the app",
			keyboard_capture(lock_shortcut, dirty_state, tooltip_signals),
			"Start search",
			keyboard_capture(search_shortcut, dirty_state, tooltip_signals),
			"Open settings",
			keyboard_capture(settings_shortcut, dirty_state, tooltip_signals),
			Empty::new(),
			(
				icon_button(
					IconButton {
						icon: String::from(revert_icon),
						tooltip: String::from("Reset shortcuts"),
						tooltip_signals,
						..IconButton::default()
					},
					move || {
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
				button("Save").on_event_cont(listener::Click, move |_, _| {
					env.config.edit_shortcuts(Shortcuts {
						lock: lock_shortcut.get(),
						search: search_shortcut.get(),
						settings: settings_shortcut.get(),
					});
					dirty_state.set(false);
				}),
			)
				.style(move |s| {
					s.col_gap(5)
						.display(Display::None)
						.apply_if(dirty_state.get(), |s| s.display(Display::Flex))
				}),
		)
			.style(|s| s.margin_bottom(120))
			.style(styles::settings_line),
	)
}
