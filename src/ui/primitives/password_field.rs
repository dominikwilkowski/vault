use floem::{
	event::{Event, EventListener, EventPropagation},
	peniko::Color,
	reactive::{create_effect, create_rw_signal, RwSignal},
	style::{CursorStyle, Position},
	views::{container, h_stack, label, svg, Decorators},
	IntoView, View, ViewId,
};

use crate::ui::{colors::*, primitives::input_field::input_field};

pub struct Password {
	id: ViewId,
	pub input_id: ViewId,
}

impl View for Password {
	fn id(&self) -> ViewId {
		self.id
	}

	fn debug_name(&self) -> std::borrow::Cow<'static, str> {
		"Password Field".into()
	}
}

pub fn password_field(value: RwSignal<String>, placeholder: &str) -> Password {
	let show_password = create_rw_signal(false);
	let is_focused = create_rw_signal(false);

	let see_icon = include_str!("../icons/see.svg");
	let hide_icon = include_str!("../icons/hide.svg");

	let input = input_field(value);
	let input_id = input.id();
	let height = 25;

	let child = h_stack((
		input
			.placeholder(placeholder)
			.on_event_cont(EventListener::FocusGained, move |_| {
				is_focused.set(true);
			})
			.on_event_cont(EventListener::FocusLost, move |_| {
				is_focused.set(false);
			})
			.style(move |s| {
				s.position(Position::Relative)
					.width_full()
					.height(height)
					.border(0)
					.font_family(String::from("Monospace"))
					.color(Color::TRANSPARENT)
					.border_color(Color::TRANSPARENT)
					.outline_color(Color::TRANSPARENT)
					.background(Color::TRANSPARENT)
					.focus_visible(|s| s.outline_color(Color::TRANSPARENT))
					.hover(|s| s.background(Color::TRANSPARENT))
					.focus(|s| s.hover(|s| s.background(Color::TRANSPARENT)))
			}),
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
				.background(Color::TRANSPARENT)
				.color(C_MAIN_TEXT)
				.hover(|s| s.color(C_MAIN_TEXT))
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
			input_id.request_focus();
		})
		.style(move |s| s.height(height).padding(4).cursor(CursorStyle::Pointer)),
	))
	.style(move |s| {
		s.flex()
			.items_center()
			.width_full()
			.border(1)
			.border_radius(2)
			.border_color(C_TOP_TEXT)
			.apply_if(is_focused.get(), |s| s.border_color(C_FOCUS))
			.hover(|s| s.background(C_FOCUS.with_alpha_factor(0.05)))
	});

	let id = ViewId::new();
	id.set_children(vec![child.into_view()]);
	Password { id, input_id }
}
