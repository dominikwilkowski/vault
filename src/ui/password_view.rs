use zeroize::Zeroize;

use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, use_context, RwSignal},
	style::Position,
	view::View,
	views::{v_stack, Decorators},
};

use crate::ui::{
	colors::*,
	primitives::{
		logo::logo,
		password_field::password_field,
		toast::{toast_view, ToastSignals},
	},
};

pub fn password_view(password: RwSignal<String>) -> impl View {
	let toast_signals =
		use_context::<ToastSignals>().expect("No toast_signals context provider");

	let value = create_rw_signal(String::from(""));

	let input = password_field(value, "Enter password");
	let input_id = input.input_id;

	// TODO: add button for creating new db and deleting the db in-case one lost their password

	v_stack((
		toast_view(toast_signals),
		logo().style(|s| s.margin_bottom(25)),
		input
			.request_focus(move || password.track())
			.on_event_cont(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if key == PhysicalKey::Code(KeyCode::Enter) {
					password.set(value.get());
					value.update(|pass| pass.zeroize());
					input_id.request_focus();
				}
			})
			.style(|s| s.width(250)),
	))
	.style(|s| {
		s.position(Position::Absolute)
			.inset(0)
			.flex()
			.items_center()
			.justify_center()
			.width_full()
			.height_full()
			.gap(0, 6)
			.background(C_MAIN_BG)
	})
}
