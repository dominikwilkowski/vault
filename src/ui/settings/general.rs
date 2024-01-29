use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal},
	views::{container, h_stack, label, v_stack, Container, Decorators},
	widgets::toggle_button,
	EventPropagation,
};

use crate::{
	config::Config,
	ui::{
		colors::*,
		primitives::{
			button::{ButtonVariant, NormalButton, normal_button}, password_field::password_field, styles,
			tooltip::TooltipSignals,
		},
	},
};

#[derive(Debug, Clone)]
struct PasswordStatus {
	pub message: String,
	pub success: bool,
}

pub fn general_view(
	_tooltip_signals: TooltipSignals,
	config: Config,
) -> Container {
	let old_password = create_rw_signal(String::from(""));
	let new_password = create_rw_signal(String::from(""));
	let new_password_check = create_rw_signal(String::from(""));
	let password_error = create_rw_signal(PasswordStatus {
		message: String::from(""),
		success: true,
	});
	let password_config = config.clone();
	let new_pass_again_config = config.clone();
	let old_pass_config = config.clone();
	let new_pass_config = config.clone();

	let debug_settings_slot = if std::env::var("DEBUG").is_ok() {
		let is_encrypted = create_rw_signal(config.config_db.read().encrypted);

		container(v_stack((
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
							config.config_db.write().encrypted = new_state;
							let _ = config.save();
							is_encrypted.set(new_state);
						})
						.style(styles::toggle_button),
				),
			))
			.style(styles::settings_line),
		)))
		.style(move |s| {
			s.border_top(1).border_color(C_BG_MAIN_BORDER).padding_top(5)
		})
		.style(|s| s.margin_top(20).width_full())
	} else {
		container(label(|| ""))
	};
	let change_password_slot = container(v_stack((
		label(|| "Change password:"),
		h_stack((
			label(|| "Old Password"),
			container(password_field(old_password, "").on_event(
				EventListener::KeyDown,
				move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};
					if key == PhysicalKey::Code(KeyCode::Enter) {
						change_password(
							old_pass_config.clone(),
							old_password,
							new_password,
							new_password_check,
							password_error,
						);
					}
					EventPropagation::Continue
				},
			)),
			label(|| "New Password"),
			container(password_field(new_password, "").on_event(
				EventListener::KeyDown,
				move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};
					if key == PhysicalKey::Code(KeyCode::Enter) {
						change_password(
							new_pass_config.clone(),
							old_password,
							new_password,
							new_password_check,
							password_error,
						);
					}
					EventPropagation::Continue
				},
			)),
			label(|| "New Password Again"),
			container(password_field(new_password_check, "").on_event(
				EventListener::KeyDown,
				move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};
					if key == PhysicalKey::Code(KeyCode::Enter) {
						change_password(
							new_pass_again_config.clone(),
							old_password,
							new_password,
							new_password_check,
							password_error,
						);
					}
					EventPropagation::Continue
				},
			)),
			container(label(|| "").style(|s| s.height(0))),
			label(move || password_error.get().message).style(move |s| {
				if password_error.get().message.is_empty() {
					return s.height(0);
				}
				if password_error.get().success {
					s.justify_end().color(C_SUCCESS)
				} else {
					s.justify_end().color(C_ERROR)
				}
			}),
			container(label(|| "")),
			container(normal_button(NormalButton {
				label: "Change password".to_string(),
				variant: ButtonVariant::Default,
				switch: None,
				tooltip: "Change password button".to_string(),
				tooltip2: None,
				tooltip_signals: TooltipSignals::new(),
			}, move |_| {
				change_password(
					password_config.clone(),
					old_password,
					new_password,
					new_password_check,
					password_error,
				)
			}))
			.style(|s| s.justify_end()),
		))
		.style(styles::settings_line),
	)))
	.style(|s| s.width_full());

	container(
		v_stack((change_password_slot, debug_settings_slot))
			.style(|s| s.width_full()),
	)
	.style(|s| s.width_full().min_width(210))
}

fn change_password(
	config: Config,
	old_password: RwSignal<String>,
	new_password: RwSignal<String>,
	new_password_check: RwSignal<String>,
	password_error: RwSignal<PasswordStatus>,
) {
	if new_password.get() != new_password_check.get() {
		password_error.set(PasswordStatus {
			message: String::from("New passwords do not match"),
			success: false,
		});
		return
	}
	if new_password.get().is_empty() {
		password_error.set(PasswordStatus {
			message: String::from("Empty passwords are not allowed"),
			success: false,
		});
		return
	}
	let result = config.change_password(old_password.get(), new_password.get());
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
