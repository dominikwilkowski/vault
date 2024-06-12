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

pub fn password_view(password: RwSignal<String>) -> impl IntoView {
	let toast_signals =
		use_context::<ToastSignals>().expect("No toast_signals context provider");

	let value = create_rw_signal(String::from(""));

	let input = password_field(value, "Enter password");
	let input_id = input.input_id;

	// TODO: add button for creating new db and deleting the db in-case one lost their password

	(
		toast_view(toast_signals),
		logo().style(|s| s.margin_bottom(25)),
		input
			.request_focus(move || password.track())
			.on_event_cont(EventListener::FocusLost, move |_| {
				input_id.request_focus();
			})
			.on_event_cont(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if is_submit(key) {
					let password_entered = value.get_untracked();
					value.update(|pass| pass.zeroize());
					password.set(password_entered);
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
