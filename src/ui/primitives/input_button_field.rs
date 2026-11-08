use floem::{
	event::{
		listener::{self, EventListenerTrait},
		EventCx,
	},
	peniko::Color,
	reactive::{RwSignal, SignalGet, SignalUpdate},
	style::CursorStyle,
	views::{svg, Container, Decorators, Stack, TextInput},
	IntoView, View, ViewId,
};

use crate::ui::{
	colors::*,
	focus::focus_later,
	primitives::{input_field::input_field, tooltip::TooltipSignals},
};

/// A text input with an icon button on its right.
///
/// The view is assembled when it is converted with [`IntoView`], so event
/// listeners can still be attached to the inner text input beforehand.
pub struct InputButton {
	input: TextInput,
	pub input_id: ViewId,
	icon: RwSignal<String>,
	tooltip: String,
	tooltip_signals: TooltipSignals,
	is_focused: RwSignal<bool>,
	on_click: Box<dyn Fn()>,
}

impl InputButton {
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

impl IntoView for InputButton {
	type V = Stack;
	type Intermediate = Stack;

	fn into_intermediate(self) -> Stack {
		let InputButton {
			input,
			input_id,
			icon,
			tooltip,
			tooltip_signals,
			is_focused,
			on_click,
		} = self;
		let height = 24;

		(
			input.style(move |s| {
				s.flex_grow(1.0_f32)
					.width_full()
					.min_width(0)
					.height(height)
					.border(0)
					.font_family(String::from(FONT_MONOSPACE))
					.border_color(Color::TRANSPARENT)
					.outline_color(Color::TRANSPARENT)
					.background(Color::TRANSPARENT)
					.focus_visible(|s| s.outline_color(Color::TRANSPARENT))
					.hover(|s| s.background(Color::TRANSPARENT))
					.focus(|s| s.hover(|s| s.background(Color::TRANSPARENT)))
			}),
			Container::new(svg(move || icon.get()).style(|s| s.width(16).height(16)))
				.on_event_cont(listener::Click, move |_, _| {
					on_click();
					focus_later(input_id);
				})
				.on_event_cont(listener::PointerEnter, move |_, _| {
					tooltip_signals.show(tooltip.clone());
				})
				.on_event_cont(listener::PointerLeave, move |_, _| {
					tooltip_signals.hide();
				})
				.style(move |s| {
					s.flex()
						.items_center()
						.justify_center()
						.height(height)
						.padding(4)
						.cursor(CursorStyle::Pointer)
				}),
		)
			.style(move |s| {
				s.flex()
					.flex_grow(1.0_f32)
					.min_width(0)
					.height(height)
					.items_center()
					.border(1)
					.border_radius(2)
					.border_color(C_TOP_TEXT)
					.apply_if(is_focused.get(), |s| s.border_color(C_FOCUS))
					.hover(|s| s.background(C_FOCUS.multiply_alpha(0.05)))
			})
			.into_view()
	}
}

pub struct InputButtonField<'a> {
	pub value: RwSignal<String>,
	pub icon: RwSignal<String>,
	pub placeholder: &'a str,
	pub tooltip: String,
	pub tooltip_signals: TooltipSignals,
}

pub fn input_button_field(
	param: InputButtonField,
	on_click: impl Fn() + 'static,
) -> InputButton {
	let InputButtonField {
		value,
		icon,
		placeholder,
		tooltip,
		tooltip_signals,
	} = param;
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

	InputButton {
		input,
		input_id,
		icon,
		tooltip,
		tooltip_signals,
		is_focused,
		on_click: Box::new(on_click),
	}
}
