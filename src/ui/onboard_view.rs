use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal},
	style::Position,
	view::View,
	views::{label, v_stack, Decorators},
	EventPropagation,
};

use crate::ui::{
	colors::*,
	primitives::{logo::logo, password_field::password_field},
};

fn save(
	password: RwSignal<String>,
	new_password_value: RwSignal<String>,
	repeat_password_value: RwSignal<String>,
	error: RwSignal<String>,
) {
	if new_password_value.get() == repeat_password_value.get() {
		password.set(new_password_value.get());
		error.set(String::from(""));
	} else {
		error.set(String::from("The passwords are not the same"));
	}
}

pub fn onboard_view(password: RwSignal<String>) -> impl View {
	let new_password_value = create_rw_signal(String::from(""));
	let repeat_password_value = create_rw_signal(String::from(""));
	let error = create_rw_signal(String::from(""));

	let password_input = password_field(new_password_value, "Create a password");
	let input_id = password_input.input_id;

	v_stack((
		label(|| "Welcome to"),
		logo().style(|s| s.margin_bottom(15)),
		password_input
			.request_focus(move || password.track())
			.on_event(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if key == PhysicalKey::Code(KeyCode::Enter) {
					save(password, new_password_value, repeat_password_value, error);
					input_id.request_focus();
				}

				EventPropagation::Continue
			})
			.style(|s| s.width(250)),
		password_field(repeat_password_value, "Repeat password")
			.on_event(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if key == PhysicalKey::Code(KeyCode::Enter) {
					save(password, new_password_value, repeat_password_value, error);
					input_id.request_focus();
				}

				EventPropagation::Continue
			})
			.style(|s| s.width(250)),
		label(move || error.get()).style(|s| s.color(C_ERROR)),
	))
	.style(|s| {
		s.position(Position::Absolute)
			.inset(0)
			.z_index(1000)
			.flex()
			.items_center()
			.justify_center()
			.width_full()
			.height_full()
			.gap(0, 6)
			.background(C_BG_MAIN.with_alpha_factor(0.8))
	})
}
