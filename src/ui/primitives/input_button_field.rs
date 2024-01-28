use floem::{
	event::{Event, EventListener},
	id::Id,
	peniko::Color,
	reactive::{create_effect, create_rw_signal, RwSignal},
	style::CursorStyle,
	view::{View, ViewData},
	views::{container, h_stack, svg, Decorators},
	EventPropagation,
};

use crate::ui::{
	colors::*,
	primitives::{input_field::input_field, tooltip::TooltipSignals},
};

pub struct InputButton {
	view_data: ViewData,
	child: Box<dyn View>,
	pub input_id: Id,
}

impl View for InputButton {
	fn view_data(&self) -> &ViewData {
		&self.view_data
	}

	fn view_data_mut(&mut self) -> &mut ViewData {
		&mut self.view_data
	}

	fn for_each_child<'a>(
		&'a self,
		for_each: &mut dyn FnMut(&'a dyn View) -> bool,
	) {
		for_each(&self.child);
	}

	fn for_each_child_mut<'a>(
		&'a mut self,
		for_each: &mut dyn FnMut(&'a mut dyn View) -> bool,
	) {
		for_each(&mut self.child);
	}

	fn for_each_child_rev_mut<'a>(
		&'a mut self,
		for_each: &mut dyn FnMut(&'a mut dyn View) -> bool,
	) {
		for_each(&mut self.child);
	}
}

#[allow(dead_code)]
impl InputButton {
	pub fn request_focus(self, when: impl Fn() + 'static) -> Self {
		create_effect(move |_| {
			when();
			self.input_id.request_focus();
		});
		self
	}

	pub fn disabled(self, disabled_fn: impl Fn() -> bool + 'static) -> Self {
		let id = self.input_id;

		create_effect(move |_| {
			let is_disabled = disabled_fn();
			id.update_disabled(is_disabled);
		});

		self
	}

	pub fn on_event(
		self,
		listener: EventListener,
		action: impl Fn(&Event) -> EventPropagation + 'static,
	) -> Self {
		let id = self.input_id;
		id.update_event_listener(listener, Box::new(action));
		self
	}

	pub fn on_click(
		self,
		action: impl Fn(&Event) -> EventPropagation + 'static,
	) -> Self {
		let id = self.input_id;
		id.update_event_listener(EventListener::Click, Box::new(action));
		self
	}

	pub fn on_double_click(
		self,
		action: impl Fn(&Event) -> EventPropagation + 'static,
	) -> Self {
		let id = self.input_id;
		id.update_event_listener(EventListener::DoubleClick, Box::new(action));
		self
	}

	pub fn on_secondary_click(
		self,
		action: impl Fn(&Event) -> EventPropagation + 'static,
	) -> Self {
		let id = self.input_id;
		id.update_event_listener(EventListener::SecondaryClick, Box::new(action));
		self
	}

	pub fn on_resize(
		self,
		action: impl Fn(floem::kurbo::Rect) + 'static,
	) -> Self {
		let id = self.input_id;
		id.update_resize_listener(Box::new(action));
		self
	}

	pub fn on_move(self, action: impl Fn(floem::kurbo::Point) + 'static) -> Self {
		let id = self.input_id;
		id.update_move_listener(Box::new(action));
		self
	}

	pub fn on_cleanup(self, action: impl Fn() + 'static) -> Self {
		let id = self.input_id;
		id.update_cleanup_listener(Box::new(action));
		self
	}

	pub fn animation(self, anim: floem::animate::Animation) -> Self {
		let id = self.input_id;
		create_effect(move |_| {
			id.update_animation(anim.clone());
		});
		self
	}

	pub fn clear_focus(self, when: impl Fn() + 'static) -> Self {
		let id = self.input_id;
		create_effect(move |_| {
			when();
			id.clear_focus();
		});
		self
	}

	pub fn context_menu(
		self,
		menu: impl Fn() -> floem::menu::Menu + 'static,
	) -> Self {
		let id = self.input_id;
		id.update_context_menu(Box::new(menu));
		self
	}

	/// Adds a primary-click context menu, which opens below the view.
	pub fn popout_menu(
		self,
		menu: impl Fn() -> floem::menu::Menu + 'static,
	) -> Self {
		let id = self.input_id;
		id.update_popout_menu(Box::new(menu));
		self
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
	let is_focused = create_rw_signal(false);

	let input = input_field(value);
	let input_id = input.id();
	let height = 24;

	let child = h_stack((
		input
			.placeholder(placeholder)
			.on_event(EventListener::FocusGained, move |_| {
				is_focused.set(true);
				EventPropagation::Continue
			})
			.on_event(EventListener::FocusLost, move |_| {
				is_focused.set(false);
				EventPropagation::Continue
			})
			.style(move |s| {
				s.flex_grow(1.0)
					.width_full()
					.height(height)
					.border(0)
					.font_family(String::from("Monospace"))
					.border_color(Color::TRANSPARENT)
					.outline_color(Color::TRANSPARENT)
					.background(Color::TRANSPARENT)
					.focus_visible(|s| s.outline_color(Color::TRANSPARENT))
					.hover(|s| s.background(Color::TRANSPARENT))
					.focus(|s| s.hover(|s| s.background(Color::TRANSPARENT)))
			}),
		container(svg(move || icon.get()).style(|s| s.width(16).height(16)))
			.on_click_cont(move |_| {
				on_click();
				input_id.request_focus();
			})
			.on_event(EventListener::PointerEnter, move |_| {
				tooltip_signals.show(tooltip.clone());
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				tooltip_signals.hide();
				EventPropagation::Continue
			})
			.style(move |s| {
				s.flex()
					.items_center()
					.justify_center()
					.height(height)
					.padding(4)
					.cursor(CursorStyle::Pointer)
			}),
	))
	.style(move |s| {
		s.flex()
			.flex_grow(1.0)
			.height(height)
			.items_center()
			.border(1)
			.border_radius(2)
			.border_color(C_TEXT_TOP)
			.apply_if(is_focused.get(), |s| s.border_color(C_FOCUS))
			.hover(|s| s.background(C_FOCUS.with_alpha_factor(0.05)))
	});

	InputButton {
		view_data: ViewData::new(Id::next()),
		child: Box::new(child),
		input_id,
	}
}
