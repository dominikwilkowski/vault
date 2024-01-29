use floem::event::{Event, EventListener};
use floem::keyboard::{KeyCode, PhysicalKey};
use floem::reactive::RwSignal;
use floem::widgets::button;
use floem::{
	reactive::create_rw_signal,
	views::{container, h_stack, label, v_stack, Container, Decorators},
	widgets::toggle_button,
	EventPropagation,
};

use crate::ui::primitives::password_field::password_field;
use crate::{
	config::Config,
	ui::{
		colors::*,
		primitives::{styles, tooltip::TooltipSignals},
	},
};

pub fn general_view(
	_tooltip_signals: TooltipSignals,
	config: Config,
) -> Container {
	let old_password = create_rw_signal(String::from(""));
	let new_password = create_rw_signal(String::from(""));
	let new_password_check = create_rw_signal(String::from(""));
	let password_error = create_rw_signal(String::from(""));
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
						return change_password(
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
						return change_password(
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
						return change_password(
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
			container(label(|| "")),
			label(move || password_error.get()).style(|s| s.color(C_ERROR)),
			container(label(|| "")),
			container(button(|| "Update Password").on_click(move |_| {
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
	.style(move |s| s.border_top(1).border_color(C_BG_MAIN_BORDER).padding_top(5))
	.style(|s| s.margin_top(20).width_full());

	container(
		v_stack((
			change_password_slot.style(styles::settings_line),
			debug_settings_slot,
		))
		.style(|s| s.width_full()),
	)
	.style(|s| s.width_full().min_width(210))
}

fn change_password(
	config: Config,
	old_password: RwSignal<String>,
	new_password: RwSignal<String>,
	new_password_check: RwSignal<String>,
	password_error: RwSignal<String>,
) -> EventPropagation {
	if new_password.get() != new_password_check.get() {
		password_error.set(String::from("New passwords do not match"));
		return EventPropagation::Continue;
	}
	let result = config.change_password(old_password.get(), new_password.get());
	match result {
		Ok(()) => password_error.set(String::from("Password updated successfully")),
		Err(e) => password_error.set(e.to_string()),
	}
	EventPropagation::Continue
}
