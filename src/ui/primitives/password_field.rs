use floem::{
	action::set_ime_allowed,
	event::{
		listener::{self, EventListenerTrait},
		EventCx, EventPropagation,
	},
	peniko::Color,
	reactive::{Effect, RwSignal, SignalGet, SignalUpdate, SignalWith},
	style::{CursorStyle, Display},
	ui_events::keyboard::{Key, KeyboardEvent, Modifiers, NamedKey},
	views::{svg, Container, Decorators, Empty, Label, Stack},
	Clipboard, IntoView, View, ViewId,
};
use zeroize::{Zeroize, Zeroizing};

use crate::ui::{
	colors::*,
	focus::{focus_later, has_focus},
};

/// A masked input for the master password with a reveal toggle.
///
/// The typed characters only live in `value`, a buffer that wipes itself when
/// it is dropped. Floem's text input keeps unwiped copies of its buffer and of
/// the laid out text (GHSA-vjvh-m6q3-jrgc), so this view handles the key
/// events itself and only ever shows floem one dot per character, or the
/// plain text while the user holds it revealed.
///
/// The view is assembled when it is converted with [`IntoView`], so event
/// listeners can still be attached to the input beforehand.
pub struct Password {
	input: Container,
	pub input_id: ViewId,
	show_password: RwSignal<bool>,
	is_focused: RwSignal<bool>,
}

impl Password {
	/// Focus the input whenever `when` runs (it is reactive).
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

	/// Attach an event listener to the input.
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
			show_password,
			is_focused,
		} = self;

		let see_icon = include_str!("../icons/see.svg");
		let hide_icon = include_str!("../icons/hide.svg");
		let height = 25;

		(
			input.style(move |s| s.flex_grow(1.0_f32).width_full().height(height)),
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

pub fn password_field(
	value: RwSignal<Zeroizing<String>>,
	placeholder: &str,
) -> Password {
	let show_password = RwSignal::new(false);
	let is_focused = RwSignal::new(false);
	let placeholder = String::from(placeholder);

	let text = Label::derived(move || {
		value.with(|typed| {
			if typed.is_empty() {
				placeholder.clone()
			} else if show_password.get() {
				// revealed on request; floem keeps its own copy of what it draws
				typed.to_string()
			} else {
				"•".repeat(typed.chars().count())
			}
		})
	})
	.style(move |s| {
		s.font_family(String::from(FONT_MONOSPACE))
			.color(C_MAIN_TEXT)
			.apply_if(value.with(|typed| typed.is_empty()), |s| {
				s.color(C_MAIN_TEXT.multiply_alpha(0.5))
			})
	});

	let caret = Empty::new().style(move |s| {
		s.width(1)
			.height(15)
			.margin_left(1)
			.background(C_FOCUS.multiply_alpha(0.5))
			.apply_if(!is_focused.get(), |s| s.display(Display::None))
	});

	let input = Container::new(
		(text, caret).style(|s| s.flex().items_center().height_full()),
	);
	let input_id = input.id();
	let input = input
		.style(|s| {
			s.keyboard_navigable()
				.padding_left(5)
				.cursor(CursorStyle::Text)
				.focus_visible(|s| s.outline_color(Color::TRANSPARENT))
		})
		.on_event_cont(listener::Click, move |_, _| focus_later(input_id))
		.on_event_cont(listener::FocusGained, move |_, _| {
			is_focused.set(true);
			set_ime_allowed(true);
		})
		.on_event_cont(listener::FocusLost, move |_, _| {
			is_focused.set(false);
			set_ime_allowed(false);
		})
		.on_event(listener::KeyDown, move |cx, event| handle_key(cx, event, value))
		.on_event_cont(listener::ImeCommit, move |_, text| {
			value.update(|typed| typed.push_str(text));
		});

	Password {
		input,
		input_id,
		show_password,
		is_focused,
	}
}

fn handle_key(
	cx: &mut EventCx,
	event: &KeyboardEvent,
	value: RwSignal<Zeroizing<String>>,
) -> EventPropagation {
	if !has_focus(cx) {
		return EventPropagation::Continue;
	}

	let command =
		event.modifiers.intersects(Modifiers::CONTROL | Modifiers::META);

	match &event.key {
		Key::Character(text) if command && text.eq_ignore_ascii_case("v") => {
			paste(value);
			EventPropagation::Stop
		},
		// other shortcuts belong to the app, like floem's text input does it
		Key::Character(_)
			if command || event.modifiers.intersects(Modifiers::ALT) =>
		{
			EventPropagation::Continue
		},
		Key::Character(text) => {
			value.update(|typed| typed.push_str(text));
			EventPropagation::Stop
		},
		Key::Named(NamedKey::Backspace) => {
			value.update(remove_last_char);
			EventPropagation::Stop
		},
		// Enter, Escape and Tab are for the callers and the focus navigation
		_ => EventPropagation::Continue,
	}
}

/// Remove the last character and wipe its bytes from the buffer's capacity.
fn remove_last_char(typed: &mut Zeroizing<String>) {
	if typed.pop().is_some() {
		let kept = Zeroizing::new(typed.to_string());
		typed.zeroize();
		typed.push_str(&kept);
	}
}

fn paste(value: RwSignal<Zeroizing<String>>) {
	if let Ok(content) = Clipboard::get_contents() {
		let mut content = Zeroizing::new(content);
		content.retain(|c| c != '\r' && c != '\n');
		value.update(|typed| typed.push_str(&content));
	}
}
