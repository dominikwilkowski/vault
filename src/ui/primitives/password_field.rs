use floem::{
	event::{Event, EventListener},
	id::Id,
	peniko::Color,
	reactive::{create_effect, create_rw_signal, RwSignal},
	style::{CursorStyle, Position},
	view::{View, ViewData, Widget},
	views::{container, h_stack, label, svg, Decorators},
	EventPropagation,
};

use crate::ui::{colors::*, primitives::input_field::input_field};

pub struct Password {
	data: ViewData,
	child: Box<dyn Widget>,
	pub input_id: Id,
}

impl View for Password {
	fn view_data(&self) -> &ViewData {
		&self.data
	}

	fn view_data_mut(&mut self) -> &mut ViewData {
		&mut self.data
	}

	fn build(self) -> Box<dyn Widget> {
		Box::new(self)
	}
}

impl Widget for Password {
	fn view_data(&self) -> &ViewData {
		&self.data
	}

	fn view_data_mut(&mut self) -> &mut ViewData {
		&mut self.data
	}

	fn for_each_child<'a>(
		&'a self,
		for_each: &mut dyn FnMut(&'a dyn Widget) -> bool,
	) {
		for_each(&self.child);
	}

	fn for_each_child_mut<'a>(
		&'a mut self,
		for_each: &mut dyn FnMut(&'a mut dyn Widget) -> bool,
	) {
		for_each(&mut self.child);
	}

	fn for_each_child_rev_mut<'a>(
		&'a mut self,
		for_each: &mut dyn FnMut(&'a mut dyn Widget) -> bool,
	) {
		for_each(&mut self.child);
	}
}

#[allow(dead_code)]
impl Password {
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

	pub fn on_event_cont(
		self,
		listener: EventListener,
		action: impl Fn(&Event) + 'static,
	) -> Self {
		self.on_event(listener, move |e| {
			action(e);
			EventPropagation::Continue
		})
	}

	pub fn on_event_stop(
		self,
		listener: EventListener,
		action: impl Fn(&Event) + 'static,
	) -> Self {
		self.on_event(listener, move |e| {
			action(e);
			EventPropagation::Stop
		})
	}

	pub fn on_click(
		self,
		action: impl Fn(&Event) -> EventPropagation + 'static,
	) -> Self {
		let id = self.input_id;
		id.update_event_listener(EventListener::Click, Box::new(action));
		self
	}

	pub fn on_click_cont(
		self,
		action: impl Fn(&Event) -> EventPropagation + 'static,
	) -> Self {
		self.on_click(move |e| {
			action(e);
			EventPropagation::Continue
		})
	}

	pub fn on_click_stop(
		self,
		action: impl Fn(&Event) -> EventPropagation + 'static,
	) -> Self {
		self.on_click(move |e| {
			action(e);
			EventPropagation::Stop
		})
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
			.static_placeholder(placeholder)
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

	Password {
		data: ViewData::new(Id::next()),
		child: child.build(),
		input_id,
	}
}
