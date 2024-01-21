use floem::{
	event::{Event, EventListener},
	id::Id,
	peniko::Color,
	reactive::{create_effect, create_rw_signal, RwSignal},
	style::{CursorStyle, Position},
	view::{View, ViewData},
	views::{container, h_stack, label, svg, Decorators},
	EventPropagation,
};

use crate::ui::{colors::*, primitives::input_field::input_field};

pub struct Password {
	view_data: ViewData,
	child: Box<dyn View>,
	placeholder_text: Option<String>,
	pub input_id: Id,
}

impl View for Password {
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

impl Password {
	pub fn placeholder(mut self, text: impl Into<String>) -> Self {
		self.placeholder_text = Some(text.into());
		self
	}

	pub fn request_focus(self, when: impl Fn() + 'static) -> Self {
		create_effect(move |_| {
			when();
			self.input_id.request_focus();
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
}

pub fn password_field(value: RwSignal<String>) -> Password {
	let show_password = create_rw_signal(false);
	let is_focused = create_rw_signal(false);

	let see_icon = include_str!("../icons/see.svg");
	let hide_icon = include_str!("../icons/hide.svg");

	let input = input_field(value);
	let input_id = input.id();
	let height = 25;

	let child = h_stack((
		input
			.keyboard_navigatable()
			.on_event(EventListener::FocusGained, move |_| {
				is_focused.set(true);
				EventPropagation::Continue
			})
			.on_event(EventListener::FocusLost, move |_| {
				is_focused.set(false);
				EventPropagation::Continue
			})
			.style(move |s| {
				s.position(Position::Relative)
					.width(250)
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
				.color(C_TEXT_MAIN)
				.hover(|s| s.color(C_TEXT_MAIN))
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
			.border(1)
			.border_radius(2)
			.border_color(C_TEXT_TOP)
			.apply_if(is_focused.get(), |s| s.border_color(C_FOCUS))
			.hover(|s| s.background(C_FOCUS.with_alpha_factor(0.05)))
	});

	Password {
		view_data: ViewData::new(Id::next()),
		child: Box::new(child),
		placeholder_text: None,
		input_id,
	}
}
