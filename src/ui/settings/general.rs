use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal},
	view::View,
	views::{container, empty, h_stack, label, v_stack, Decorators},
	widgets::toggle_button,
	EventPropagation,
};

use crate::env::Environment;
use crate::{
	ui::{
		colors::*,
		primitives::{
			button::button,
			password_field::{password_field, Password},
			styles,
			tooltip::TooltipSignals,
		},
	},
	DEFAULT_DEBUG_PASSWORD,
};

#[derive(Debug, Clone)]
struct PasswordStatus {
	pub message: String,
	pub success: bool,
}

pub fn general_view(
	tooltip_signals: TooltipSignals,
	env: Environment,
) -> impl View {
	let old_password = create_rw_signal(String::from(""));
	let new_password = create_rw_signal(String::from(""));
	let new_password_check = create_rw_signal(String::from(""));
	let password_error = create_rw_signal(PasswordStatus {
		message: String::from(""),
		success: true,
	});

	let password_env = env.clone();
	let new_pass_again_env = env.clone();
	let old_pass_env = env.clone();
	let new_pass_env = env.clone();

	let debug_settings_slot = if std::env::var("DEBUG").is_ok() {
		let is_encrypted = create_rw_signal(env.db.config_db.read().encrypted);

		v_stack((
			label(|| "Debug settings")
				.style(|s| s.inset_top(-5).margin_bottom(5).color(C_BG_MAIN_BORDER)),
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
				.on_event(EventListener::PointerEnter, move |_| {
					if is_encrypted.get() {
						tooltip_signals
							.show(String::from("Decrypt with password in memory"));
					} else {
						tooltip_signals.show(format!(
							"Encrypt with default debug password \"{}\"",
							DEFAULT_DEBUG_PASSWORD
						));
					}
					EventPropagation::Continue
				})
				.on_event(EventListener::PointerLeave, move |_| {
					tooltip_signals.hide();
					EventPropagation::Continue
				}),
			))
			.style(styles::settings_line),
		))
		.any()
		.style(move |s| {
			s.border_top(1).border_color(C_BG_MAIN_BORDER).padding_top(5)
		})
		.style(|s| s.margin_top(20).width_full())
	} else {
		empty().any()
	};

	let change_password_slot = v_stack((
		h_stack((
			label(|| "Change Password"),
			v_stack((
				create_password_field(
					old_pass_env,
					"Old Password",
					old_password,
					old_password,
					new_password,
					new_password_check,
					password_error,
				),
				create_password_field(
					new_pass_env,
					"New Password",
					new_password,
					old_password,
					new_password,
					new_password_check,
					password_error,
				),
				create_password_field(
					new_pass_again_env,
					"New Password Again",
					new_password_check,
					old_password,
					new_password,
					new_password_check,
					password_error,
				),
			))
			.style(|s| s.gap(0, 5)),
		))
		.style(styles::settings_line),
		h_stack((
			label(|| "").style(|s| s.height(0)),
			label(move || password_error.get().message).style(move |s| {
				s.margin_top(5)
					.color(C_ERROR)
					.apply_if(password_error.get().success, |s| s.color(C_SUCCESS))
			}),
			label(|| ""),
			container(button("Change password").on_click_cont(move |_| {
				change_password(
					password_env.clone(),
					old_password,
					new_password,
					new_password_check,
					password_error,
				)
			})),
		))
		.style(styles::settings_line),
	))
	.style(|s| s.width_full());

	v_stack((change_password_slot, debug_settings_slot))
		.style(|s| s.margin_bottom(15))
}

fn change_password(
	env: Environment,
	old_password: RwSignal<String>,
	new_password: RwSignal<String>,
	new_password_check: RwSignal<String>,
	password_error: RwSignal<PasswordStatus>,
) {
	if new_password.get() != new_password_check.get() {
		password_error.set(PasswordStatus {
			message: String::from("New passwords do not match"),
			success: false,
		})
	} else if new_password.get().is_empty() {
		password_error.set(PasswordStatus {
			message: String::from("Empty passwords are not allowed"),
			success: false,
		})
	} else {
		let result = env.db.change_password(old_password.get(), new_password.get());
		match result {
			Ok(()) => password_error.set(PasswordStatus {
				message: String::from("Password updated successfully"),
				success: true,
			}),
			Err(e) => password_error.set(PasswordStatus {
				message: e.to_string(),
				success: false,
			}),
		}
	}
}

fn create_password_field(
	env: Environment,
	placeholder: &str,
	password_signal: RwSignal<String>,
	old_password: RwSignal<String>,
	new_password: RwSignal<String>,
	new_password_check: RwSignal<String>,
	password_error: RwSignal<PasswordStatus>,
) -> Password {
	password_field(password_signal, placeholder)
		.on_event(EventListener::KeyDown, move |event| {
			let key = match event {
				Event::KeyDown(k) => k.key.physical_key,
				_ => PhysicalKey::Code(KeyCode::F35),
			};
			if key == PhysicalKey::Code(KeyCode::Enter) {
				change_password(
					env.clone(),
					old_password,
					new_password,
					new_password_check,
					password_error,
				);
			}
			EventPropagation::Continue
		})
		.style(|s| s.width(250))
}
