use zeroize::{Zeroize, Zeroizing};

use crate::ui::secret;

use floem::{
	event::listener,
	reactive::{RwSignal, SignalTrack, SignalUpdate, SignalWith},
	style::Position,
	views::Decorators,
	IntoView,
};

use crate::ui::{
	colors::*,
	focus::{focus_later, has_focus},
	keyboard::is_submit,
	primitives::{
		logo::logo,
		password_field::password_field,
		toast::{toast_view, ToastSignals},
	},
	shared_context,
};

fn save(
	password: RwSignal<Zeroizing<String>>,
	new_password_value: RwSignal<Zeroizing<String>>,
	repeat_password_value: RwSignal<Zeroizing<String>>,
	toast_signals: ToastSignals,
) {
	let same = new_password_value.with_untracked(|new| {
		repeat_password_value.with_untracked(|repeat| **new == **repeat)
	});
	if same {
		toast_signals.kill_all_toasts();
		let typed =
			new_password_value.with_untracked(|new| Zeroizing::new(new.to_string()));
		new_password_value.update(|pass| pass.zeroize());
		repeat_password_value.update(|pass| pass.zeroize());
		password.update(|pass| {
			pass.zeroize();
			pass.push_str(&typed);
		});
	} else {
		toast_signals.add(String::from("The passwords are not the same"));
	}
}

pub fn onboard_view(password: RwSignal<Zeroizing<String>>) -> impl IntoView {
	let toast_signals = shared_context::get::<ToastSignals>()
		.expect("No toast_signals context provider");

	let new_password_value = secret::new_buffer();
	let repeat_password_value = secret::new_buffer();

	let password_input = password_field(new_password_value, "Create a password");
	let input_id = password_input.input_id;

	(
		toast_view(toast_signals),
		"Welcome to",
		logo().style(|s| s.margin_bottom(15)),
		password_input
			.request_focus(move || password.track())
			.on_input_event_cont(listener::KeyDown, move |cx, event| {
				if !has_focus(cx) {
					return;
				}

				let key = event.code;

				if is_submit(key) {
					save(
						password,
						new_password_value,
						repeat_password_value,
						toast_signals,
					);
					focus_later(input_id);
				}
			})
			.style(|s| s.width(250)),
		password_field(repeat_password_value, "Repeat password")
			.on_input_event_cont(listener::KeyDown, move |cx, event| {
				if !has_focus(cx) {
					return;
				}

				let key = event.code;

				if is_submit(key) {
					save(
						password,
						new_password_value,
						repeat_password_value,
						toast_signals,
					);
					focus_later(input_id);
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
				.row_gap(6)
				.background(C_MAIN_BG)
		})
}
