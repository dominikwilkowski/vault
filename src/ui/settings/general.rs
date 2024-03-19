use zeroize::Zeroize;

use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, use_context, RwSignal},
	view::View,
	views::{container, empty, h_stack, label, v_stack, Decorators},
	widgets::toggle_button,
};

use crate::{
	env::Environment,
	ui::{
		app_view::{ToastSignalsSettings, TooltipSignalsSettings},
		colors::*,
		primitives::{button::button, password_field::password_field, styles},
	},
	DEFAULT_DEBUG_PASSWORD,
};

fn change_password(
	old_password: RwSignal<String>,
	new_password: RwSignal<String>,
	new_password_check: RwSignal<String>,
	success: RwSignal<bool>,
) {
	let toast_signals = use_context::<ToastSignalsSettings>()
		.expect("No toast_signals context provider")
		.inner;
	let env: Environment = use_context().expect("No env context provider");

	success.set(false);
	if new_password.get() != new_password_check.get() {
		toast_signals.add(String::from("New passwords do not match"));
	} else if new_password.get().is_empty() {
		toast_signals.add(String::from("Empty passwords are not allowed"));
	} else {
		let result = env.db.change_password(old_password.get(), new_password.get());
		match result {
			Err(e) => {
				toast_signals.add(e.to_string());
			},
			Ok(()) => {
				old_password.update(|pass| pass.zeroize());
				new_password.update(|pass| pass.zeroize());
				new_password_check.update(|pass| pass.zeroize());
				toast_signals.kill_all_toasts();
				success.set(true);
			},
		}
	}
}

pub fn general_view() -> impl View {
	let tooltip_signals = use_context::<TooltipSignalsSettings>()
		.expect("No tooltip_signals context provider")
		.inner;
	let env: Environment = use_context().expect("No env context provider");

	let old_password = create_rw_signal(String::from(""));
	let new_password = create_rw_signal(String::from(""));
	let new_password_check = create_rw_signal(String::from(""));
	let success = create_rw_signal(false);

	let debug_settings_slot = if std::env::var("DEBUG").is_ok() {
		let is_encrypted = create_rw_signal(env.db.config_db.read().encrypted);

		v_stack((
			label(|| "Debug settings")
				.style(|s| s.inset_top(-5).margin_bottom(5).color(C_MAIN_BG_BORDER)),
			h_stack((
				label(move || {
					if is_encrypted.get() {
						"Disable encryption:"
					} else {
						"Enable encryption:"
					}
				}),
				container(
					toggle_button(move || is_encrypted.get())
						.on_toggle(move |_| {
							let new_state = !is_encrypted.get();
							env.db.config_db.write().encrypted = new_state;
							let _ = env.db.save();
							is_encrypted.set(new_state);
							if new_state {
								tooltip_signals
									.show(String::from("Decrypt with password in memory"));
							} else {
								tooltip_signals.show(format!(
									"Encrypt with default debug password \"{}\"",
									DEFAULT_DEBUG_PASSWORD
								));
							}
						})
						.style(styles::toggle_button),
				)
				.on_event_cont(EventListener::PointerEnter, move |_| {
					if is_encrypted.get() {
						tooltip_signals
							.show(String::from("Decrypt with password in memory"));
					} else {
						tooltip_signals.show(format!(
							"Encrypt with default debug password \"{}\"",
							DEFAULT_DEBUG_PASSWORD
						));
					}
				})
				.on_event_cont(EventListener::PointerLeave, move |_| {
					tooltip_signals.hide();
				}),
			))
			.style(styles::settings_line),
		))
		.any()
		.style(move |s| {
			s.border_top(1).border_color(C_MAIN_BG_BORDER).padding_top(5)
		})
		.style(|s| s.margin_top(20).width_full())
	} else {
		empty().any()
	};

	let change_password_slot = v_stack((
		h_stack((
			label(|| "Change Password"),
			v_stack((
				password_field(old_password, "Old Password")
					.on_event_cont(EventListener::KeyDown, move |event| {
						let key = match event {
							Event::KeyDown(k) => k.key.physical_key,
							_ => PhysicalKey::Code(KeyCode::F35),
						};
						if key == PhysicalKey::Code(KeyCode::Enter) {
							change_password(
								old_password,
								new_password,
								new_password_check,
								success,
							);
						}
					})
					.style(|s| s.width(250)),
				password_field(new_password, "New Password")
					.on_event_cont(EventListener::KeyDown, move |event| {
						let key = match event {
							Event::KeyDown(k) => k.key.physical_key,
							_ => PhysicalKey::Code(KeyCode::F35),
						};
						if key == PhysicalKey::Code(KeyCode::Enter) {
							change_password(
								old_password,
								new_password,
								new_password_check,
								success,
							);
						}
					})
					.style(|s| s.width(250)),
				password_field(new_password_check, "New Password Again")
					.on_event_cont(EventListener::KeyDown, move |event| {
						let key = match event {
							Event::KeyDown(k) => k.key.physical_key,
							_ => PhysicalKey::Code(KeyCode::F35),
						};
						if key == PhysicalKey::Code(KeyCode::Enter) {
							change_password(
								old_password,
								new_password,
								new_password_check,
								success,
							);
						}
					})
					.style(|s| s.width(250)),
			))
			.style(|s| s.gap(0, 5)),
		))
		.style(styles::settings_line),
		h_stack((
			label(|| "").style(|s| s.height(0)),
			label(move || "Password updated successfully").style(move |s| {
				s.margin_top(5)
					.color(C_MAIN_BG)
					.apply_if(success.get(), |s| s.color(C_SUCCESS))
			}),
			label(|| ""),
			container(button("Change password").on_click_cont(move |_| {
				change_password(old_password, new_password, new_password_check, success)
			})),
		))
		.style(styles::settings_line),
	))
	.style(|s| s.width_full());

	v_stack((change_password_slot, debug_settings_slot))
		.style(|s| s.margin_bottom(15))
}
