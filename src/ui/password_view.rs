use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal},
	style::Position,
	view::View,
	views::{container, h_stack, label, svg, v_stack, Decorators},
	EventPropagation,
};

use crate::ui::{colors::*, primitives::input_field::input_field};

pub fn password_view(
	password: RwSignal<String>,
	error: RwSignal<String>,
) -> impl View {
	let value = create_rw_signal(String::from(""));
	let show_password = create_rw_signal(false);

	let see_icon = include_str!("./icons/see.svg");
	let hide_icon = include_str!("./icons/hide.svg");

	let input = input_field(value);
	let input_id = input.id();
	let height = 25;

	// TODO: add button for creating new db and deleting the db in-case one lost their password

	v_stack((
		h_stack((
			label(move || {
				if show_password.get() {
					value.get()
				} else {
					let len = value.get().len();
					String::from("•").repeat(len)
				}
			})
			.style(|s| {
				s.position(Position::Absolute)
					.padding_left(5)
					.font_family(String::from("Monospace"))
					.background(floem::peniko::Color::TRANSPARENT)
					.color(C_TEXT_MAIN)
					.hover(|s| s.color(C_TEXT_MAIN))
				// .z_index(5)
			}),
			input
				.style(move |s| {
					s.position(Position::Relative)
						.width(250)
						.height(height)
						.border_right(0)
						.font_family(String::from("Monospace"))
						.color(floem::peniko::Color::TRANSPARENT)
						.background(floem::peniko::Color::TRANSPARENT)
						.hover(|s| s.background(floem::peniko::Color::TRANSPARENT))
					// .z_index(2)
				})
				.placeholder("Enter password")
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
				}),
			container(
				svg(move || {
					if show_password.get() {
						String::from(hide_icon)
					} else {
						String::from(see_icon)
					}
				})
				.style(|s| s.width(16).height(16)),
			)
			.on_click_cont(move |_| {
				show_password.set(!show_password.get());
			})
			.style(move |s| {
				s.height(height)
					.padding(4)
					.border(1)
					.border_color(C_TEXT_TOP)
					.border_left(0)
			}),
		))
		.style(|s| {
			s.flex()
				.items_center()
				.hover(|s| s.background(C_FOCUS.with_alpha_factor(0.05)))
		}),
		label(move || error.get()).style(|s| s.color(C_ERROR)),
	))
	.style(|s| {
		s.position(Position::Absolute)
			.inset(0)
			.z_index(100)
			.flex()
			.items_center()
			.justify_center()
			.width_full()
			.height_full()
			.gap(0, 6)
			.background(C_BG_MAIN.with_alpha_factor(0.8))
	})
}
