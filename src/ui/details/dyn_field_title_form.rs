use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal},
	style::{AlignContent, Display},
	view::View,
	views::{h_stack, label, Decorators, TextInput},
	EventPropagation,
};

use crate::ui::{
	details::detail_view::LABEL_WIDTH, primitives::tooltip::TooltipSignals,
};

pub struct DynFieldTitleForm {
	pub title_value: RwSignal<String>,
	pub title_editable: RwSignal<bool>,
	pub field_value: RwSignal<String>,
	pub reset_text: RwSignal<String>,
	pub is_dyn_field: bool,
	pub title_input: TextInput,
	pub tooltip_signals: TooltipSignals,
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
		tooltip_signals,
	} = params;
	let is_overflow_label = create_rw_signal(false);

	h_stack((
		label(move || title_value.get())
			.style(move |s| {
				s.flex()
					.max_width(LABEL_WIDTH)
					.text_ellipsis()
					.apply_if(title_editable.get() && is_dyn_field, |s| {
						s.display(Display::None)
					})
			})
			.on_text_overflow(move |is_overflown| {
				is_overflow_label.set(is_overflown);
			})
			.on_event(EventListener::PointerEnter, move |_| {
				if is_overflow_label.get() {
					tooltip_signals.show(title_value.get());
				}
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				tooltip_signals.hide();
				EventPropagation::Continue
			}),
		title_input
			.style(move |s| {
				s.width(LABEL_WIDTH)
					.height(24)
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
