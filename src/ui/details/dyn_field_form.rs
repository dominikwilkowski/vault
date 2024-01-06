use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::RwSignal,
	style::{AlignContent, Display},
	view::View,
	views::{h_stack, label, Decorators},
	EventPropagation,
};

use crate::ui::{
	details::detail_view::LABEL_WIDTH, primitives::input_field::input_field,
};

pub struct DynFieldForm {
	pub title_value: RwSignal<String>,
	pub save_btn_visible: RwSignal<bool>,
	pub edit_btn_visible: RwSignal<bool>,
	pub field_value: RwSignal<String>,
	pub reset_text: RwSignal<String>,
	pub is_dyn_field: bool,
}

pub fn dyn_field_form(
	params: DynFieldForm,
	on_save: impl Fn() + 'static,
) -> impl View {
	let DynFieldForm {
		title_value,
		save_btn_visible,
		edit_btn_visible,
		field_value,
		reset_text,
		is_dyn_field,
	} = params;

	h_stack((
		label(move || title_value.get()).style(move |s| {
			s.flex().apply_if(save_btn_visible.get() && is_dyn_field, |s| {
				s.display(Display::None)
			})
		}),
		input_field(title_value)
			.style(move |s| {
				s.width(LABEL_WIDTH)
					.display(Display::None)
					.apply_if(save_btn_visible.get() && is_dyn_field, |s| s.flex())
			})
			.on_event(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if key == PhysicalKey::Code(KeyCode::Escape) {
					field_value.set(reset_text.get());
					edit_btn_visible.set(true);
					save_btn_visible.set(false);
				}

				if key == PhysicalKey::Code(KeyCode::Enter) {
					on_save();
				}
				EventPropagation::Continue
			}),
	))
	.style(move |s| {
		s.flex()
			.width(LABEL_WIDTH)
			.justify_content(AlignContent::End)
			.items_center()
	})
}
