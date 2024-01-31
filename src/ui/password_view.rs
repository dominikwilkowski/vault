use floem::cosmic_text::FamilyOwned;
use floem::{
	cosmic_text::{Attrs, AttrsList, Style, TextLayout},
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	peniko::Color,
	reactive::{create_rw_signal, RwSignal},
	style::Position,
	view::View,
	views::rich_text,
	views::{label, v_stack, Decorators},
	EventPropagation,
};
use std::ops::Range;

use crate::ui::{colors::*, primitives::password_field::password_field};

const BANNER: &str = "
 __   __    _     _   _   _      _____
 \\ \\ / /   /_\\   | | | | | |    |_   _|
  \\ V /   / _ \\  | |_| | | |__    | |
   \\_/   /_/ \\_\\  \\___/  |____|   |_|
   ";

pub fn password_view(
	password: RwSignal<String>,
	error: RwSignal<String>,
) -> impl View {
	let value = create_rw_signal(String::from(""));

	let input = password_field(value, "Enter password");
	let input_id = input.input_id;

	// TODO: add button for creating new db and deleting the db in-case one lost their password

	v_stack((
		rich_text(move || {
			let attrs = Attrs::new()
				.color(Color::BLACK)
				.style(Style::Normal)
				.family(&[FamilyOwned::Monospace]);

			let mut attrs_list = AttrsList::new(attrs);

			attrs_list.add_span(
				Range { start: 56, end: 73 },
				Attrs::new()
					.color(Color::RED)
					.style(Style::Normal)
					.family(&[FamilyOwned::Monospace]),
			);

			attrs_list.add_span(
				Range { start: 78, end: 79 },
				Attrs::new()
					.color(Color::RED)
					.style(Style::Normal)
					.family(&[FamilyOwned::Monospace]),
			);

			attrs_list.add_span(
				Range {
					start: 96,
					end: 100,
				},
				Attrs::new()
					.color(Color::RED)
					.style(Style::Normal)
					.family(&[FamilyOwned::Monospace]),
			);

			attrs_list.add_span(
				Range {
					start: 101,
					end: 108,
				},
				Attrs::new()
					.color(Color::RED)
					.style(Style::Normal)
					.family(&[FamilyOwned::Monospace]),
			);

			attrs_list.add_span(
				Range {
					start: 110,
					end: 118,
				},
				Attrs::new()
					.color(Color::RED)
					.style(Style::Normal)
					.family(&[FamilyOwned::Monospace]),
			);

			attrs_list.add_span(
				Range {
					start: 143,
					end: 144,
				},
				Attrs::new()
					.color(Color::RED)
					.style(Style::Normal)
					.family(&[FamilyOwned::Monospace]),
			);

			attrs_list.add_span(
				Range {
					start: 148,
					end: 149,
				},
				Attrs::new()
					.color(Color::RED)
					.style(Style::Normal)
					.family(&[FamilyOwned::Monospace]),
			);

			attrs_list.add_span(
				Range {
					start: 152,
					end: 153,
				},
				Attrs::new()
					.color(Color::RED)
					.style(Style::Normal)
					.family(&[FamilyOwned::Monospace]),
			);

			attrs_list.add_span(
				Range {
					start: 154,
					end: 155,
				},
				Attrs::new()
					.color(Color::RED)
					.style(Style::Normal)
					.family(&[FamilyOwned::Monospace]),
			);

			let mut tl = TextLayout::new();
			tl.set_text(BANNER, attrs_list);
			tl
		}),
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
