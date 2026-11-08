use floem::{
	event::{
		listener::{self, EventListenerTrait},
		EventCx,
	},
	peniko::Color,
	reactive::{Effect, RwSignal, SignalGet, SignalUpdate},
	style::{CursorStyle, Position},
	views::{svg, Container, Decorators, Label, Stack, TextInput},
	IntoView, View, ViewId,
};

use crate::ui::{
	colors::*, focus::focus_later, primitives::input_field::input_field,
};

/// A password input with a reveal toggle.
///
/// The view is assembled when it is converted with [`IntoView`], so event
/// listeners can still be attached to the inner text input beforehand.
pub struct Password {
	input: TextInput,
	pub input_id: ViewId,
	value: RwSignal<String>,
	show_password: RwSignal<bool>,
	is_focused: RwSignal<bool>,
}

impl Password {
	/// Focus the inner input whenever `when` runs (it is reactive).
	///
	/// The request is deferred by a tick so it also works while the view is
	/// still being built, before it is part of the window.
	pub fn request_focus(self, when: impl Fn() + 'static) -> Self {
		let input_id = self.input_id;
		Effect::new(move |_| {
			when();
			focus_later(input_id);
		});
		self
	}

	/// Attach an event listener to the inner text input.
	pub fn on_input_event_cont<L: EventListenerTrait>(
		mut self,
		listener: L,
		action: impl FnMut(&mut EventCx, &L::EventData) + 'static,
	) -> Self
	where
		L::EventData: Clone,
	{
		self.input = self.input.on_event_cont(listener, action);
		self
	}
}

impl IntoView for Password {
	type V = Stack;
	type Intermediate = Stack;

	fn into_intermediate(self) -> Stack {
		let Password {
			input,
			input_id,
			value,
			show_password,
			is_focused,
		} = self;

		let see_icon = include_str!("../icons/see.svg");
		let hide_icon = include_str!("../icons/hide.svg");
		let height = 25;

		(
			input.style(move |s| {
				s.position(Position::Relative)
					.width_full()
					.height(height)
					.border(0)
					.font_family(String::from(FONT_MONOSPACE))
					.color(Color::TRANSPARENT)
					.border_color(Color::TRANSPARENT)
					.outline_color(Color::TRANSPARENT)
					.background(Color::TRANSPARENT)
					.focus_visible(|s| s.outline_color(Color::TRANSPARENT))
					.hover(|s| s.background(Color::TRANSPARENT))
					.focus(|s| s.hover(|s| s.background(Color::TRANSPARENT)))
			}),
			Label::derived(move || {
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
					.font_family(String::from(FONT_MONOSPACE))
					.background(Color::TRANSPARENT)
					.color(C_MAIN_TEXT)
					.hover(|s| s.color(C_MAIN_TEXT))
			}),
			Container::new(
				svg(move || {
					if show_password.get() {
						String::from(hide_icon)
					} else {
						String::from(see_icon)
					}
				})
				.style(|s| s.width(16).height(16)),
			)
			.on_event_cont(listener::Click, move |_, _| {
				show_password.set(!show_password.get());
				focus_later(input_id);
			})
			.style(move |s| s.height(height).padding(4).cursor(CursorStyle::Pointer)),
		)
			.style(move |s| {
				s.flex()
					.items_center()
					.width_full()
					.border(1)
					.border_radius(2)
					.border_color(C_TOP_TEXT)
					.apply_if(is_focused.get(), |s| s.border_color(C_FOCUS))
					.hover(|s| s.background(C_FOCUS.multiply_alpha(0.05)))
			})
			.into_view()
	}
}

pub fn password_field(value: RwSignal<String>, placeholder: &str) -> Password {
	let show_password = RwSignal::new(false);
	let is_focused = RwSignal::new(false);

	let input = input_field(value)
		.placeholder(placeholder)
		.on_event_cont(listener::FocusGained, move |_, _| {
			is_focused.set(true);
		})
		.on_event_cont(listener::FocusLost, move |_, _| {
			is_focused.set(false);
		});
	let input_id = input.id();

	Password {
		input,
		input_id,
		value,
		show_password,
		is_focused,
	}
}
