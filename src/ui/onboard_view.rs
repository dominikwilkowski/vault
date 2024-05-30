use zeroize::Zeroize;

use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, use_context, RwSignal},
	style::Position,
	views::Decorators,
	IntoView,
};

use crate::ui::{
	colors::*,
	keyboard::is_submit,
	primitives::{
		logo::logo,
		password_field::password_field,
		toast::{toast_view, ToastSignals},
	},
};

fn save(
	password: RwSignal<String>,
	new_password_value: RwSignal<String>,
	repeat_password_value: RwSignal<String>,
	toast_signals: ToastSignals,
) {
	if new_password_value.get() == repeat_password_value.get() {
		toast_signals.kill_all_toasts();
		password.set(new_password_value.get());
		new_password_value.update(|pass| pass.zeroize());
		repeat_password_value.update(|pass| pass.zeroize());
	} else {
		toast_signals.add(String::from("The passwords are not the same"));
	}
}

pub fn onboard_view(password: RwSignal<String>) -> impl IntoView {
	let toast_signals =
		use_context::<ToastSignals>().expect("No toast_signals context provider");

	let new_password_value = create_rw_signal(String::from(""));
	let repeat_password_value = create_rw_signal(String::from(""));

	let password_input = password_field(new_password_value, "Create a password");
	let input_id = password_input.input_id;

	(
		toast_view(toast_signals),
		"Welcome to",
		logo().style(|s| s.margin_bottom(15)),
		password_input
			.request_focus(move || password.track())
			.on_event_cont(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if is_submit(key) {
					save(
						password,
						new_password_value,
						repeat_password_value,
						toast_signals,
					);
					input_id.request_focus();
				}
			})
			.style(|s| s.width(250)),
		password_field(repeat_password_value, "Repeat password")
			.on_event_cont(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if is_submit(key) {
					save(
						password,
						new_password_value,
						repeat_password_value,
						toast_signals,
					);
					input_id.request_focus();
				}
			})
			.style(|s| s.width(250)),
	)
		.style(|s| {
			s.flex_col()
				.position(Position::Absolute)
				.inset(0)
				.flex()
				.items_center()
				.justify_center()
				.width_full()
				.height_full()
				.column_gap(6)
				.background(C_MAIN_BG)
		})
}
