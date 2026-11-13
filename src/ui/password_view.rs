use zeroize::{Zeroize, Zeroizing};

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
	secret, shared_context,
};

pub fn password_view(password: RwSignal<Zeroizing<String>>) -> impl IntoView {
	let toast_signals = shared_context::get::<ToastSignals>()
		.expect("No toast_signals context provider");

	let value = secret::new_buffer();

	let input = password_field(value, "Enter password");
	let input_id = input.input_id;

	// TODO: add button for creating new db and deleting the db in-case one lost their password

	(
		toast_view(toast_signals),
		logo().style(|s| s.margin_bottom(25)),
		input
			.request_focus(move || password.track())
			.on_input_event_cont(listener::FocusLost, move |_, _| {
				focus_later(input_id);
			})
			.on_input_event_cont(listener::KeyDown, move |cx, event| {
				if !has_focus(cx) {
					return;
				}

				let key = event.code;

				if is_submit(key) {
					// hand the typed password over and wipe the input buffer
					let typed =
						value.with_untracked(|typed| Zeroizing::new(typed.to_string()));
					value.update(|typed| typed.zeroize());
					password.update(|pass| {
						pass.zeroize();
						pass.push_str(&typed);
					});
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
