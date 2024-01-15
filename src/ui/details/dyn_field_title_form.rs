use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::RwSignal,
	style::{AlignContent, Display},
	view::View,
	views::{h_stack, label, Decorators, TextInput},
	EventPropagation,
};

use crate::ui::details::detail_view::LABEL_WIDTH;

pub struct DynFieldTitleForm {
	pub title_value: RwSignal<String>,
	pub title_editable: RwSignal<bool>,
	pub field_value: RwSignal<String>,
	pub reset_text: RwSignal<String>,
	pub is_dyn_field: bool,
	pub title_input: TextInput,
}

pub fn dyn_field_title_form(
	params: DynFieldTitleForm,
	on_save: impl Fn() + 'static,
) -> impl View {
	let DynFieldTitleForm {
		title_value,
		title_editable,
		field_value,
		reset_text,
		is_dyn_field,
		title_input,
	} = params;

	h_stack((
		label(move || title_value.get()).style(move |s| {
			s.flex().apply_if(title_editable.get() && is_dyn_field, |s| {
				s.display(Display::None)
			})
		}),
		title_input
			.style(move |s| {
				s.width(LABEL_WIDTH)
					.display(Display::None)
					.apply_if(title_editable.get() && is_dyn_field, |s| s.flex())
			})
			.on_event(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if key == PhysicalKey::Code(KeyCode::Escape) {
					field_value.set(reset_text.get());
					title_editable.set(false);
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
