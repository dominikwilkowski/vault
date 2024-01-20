use floem::{
	event::{Event, EventListener},
	id::Id,
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_effect, create_rw_signal, RwSignal},
	style::Position,
	view::{View, ViewData},
	views::{container, h_stack, label, svg, v_stack, Decorators},
	EventPropagation,
};

use crate::ui::{colors::*, primitives::input_field::input_field};

pub struct Password {
	view_data: ViewData,
	child: Box<dyn View>,
	placeholder_text: Option<String>,
	input_id: Id,
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
}

pub fn password_view(
	password: RwSignal<String>,
	error: RwSignal<String>,
) -> Password {
	let value = create_rw_signal(String::from(""));
	let show_password = create_rw_signal(false);

	let see_icon = include_str!("./icons/see.svg");
	let hide_icon = include_str!("./icons/hide.svg");

	let input = input_field(value);
	let input_id = input.id();
	let height = 25;

	// TODO: add button for creating new db and deleting the db in-case one lost their password

	let child = v_stack((
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
						.focus(|s| {
							s.hover(|s| s.background(floem::peniko::Color::TRANSPARENT))
						})

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
		s.flex()
			.items_center()
			.justify_center()
			.width_full()
			.height_full()
			.gap(0, 6)
			.background(C_BG_MAIN.with_alpha_factor(0.8))
	});

	Password {
		view_data: ViewData::new(Id::next()),
		child: Box::new(child),
		placeholder_text: None,
		input_id,
	}
}
