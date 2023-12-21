use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal},
	style::Position,
	view::View,
	views::{container, Decorators},
	EventPropagation,
};

use crate::ui::colors::*;
use crate::ui::primitives::input_field::input_field;

pub fn password_view(password: RwSignal<String>) -> impl View {
	let value = create_rw_signal(String::from(""));

	let input = input_field(value, |s| s.width(250));
	let input_id = input.id();

	container(
		input
			.placeholder("Enter password")
			.on_event(EventListener::WindowGotFocus, move |_| {
				input_id.request_focus();
				EventPropagation::Continue
			})
			.on_event(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if key == PhysicalKey::Code(KeyCode::Enter) {
					password.set(value.get());
				}
				EventPropagation::Continue
			}),
	)
	.style(|s| {
		s.position(Position::Absolute)
			.inset(0)
			.z_index(100)
			.flex()
			.items_center()
			.justify_center()
			.width_full()
			.height_full()
			.background(C_BG_MAIN.with_alpha_factor(0.8))
	})
	.on_event(EventListener::Click, move |_| EventPropagation::Stop)
}
