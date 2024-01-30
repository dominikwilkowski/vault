use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal},
	style::Position,
	view::View,
	views::{label, v_stack, Decorators},
	EventPropagation,
};

use crate::ui::{colors::*, primitives::password_field::password_field};

pub fn password_view(
	password: RwSignal<String>,
	error: RwSignal<String>,
) -> impl View {
	let value = create_rw_signal(String::from(""));

	let input = password_field(value, "Enter password");
	let input_id = input.input_id;

	// TODO: add button for creating new db and deleting the db in-case one lost their password

	v_stack((
		input
			.request_focus(move || password.track())
			.on_event(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if key == PhysicalKey::Code(KeyCode::Enter) {
					password.set(value.get());
				}

				input_id.request_focus();
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
