use std::rc::Rc;

use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, use_context, RwSignal},
	style::{AlignContent, Display},
	views::{
		editor::{
			core::{editor::EditType, selection::Selection},
			text::Document,
		},
		label, Decorators, TextInput,
	},
	IntoView,
};

use crate::ui::{
	details::detail_view::LABEL_WIDTH, keyboard::is_submit,
	primitives::tooltip::TooltipSignals,
};

pub struct DynFieldTitleForm {
	pub title_value: RwSignal<String>,
	pub title_editable: RwSignal<bool>,
	pub field_value: RwSignal<String>,
	pub doc: Rc<dyn Document>,
	pub reset_text: RwSignal<String>,
	pub is_dyn_field: bool,
	pub title_input: TextInput,
}

pub fn dyn_field_title_form(
	params: DynFieldTitleForm,
	on_save: impl Fn() + 'static,
) -> impl IntoView {
	let DynFieldTitleForm {
		title_value,
		title_editable,
		field_value,
		doc,
		reset_text,
		is_dyn_field,
		title_input,
	} = params;

	let tooltip_signals = use_context::<TooltipSignals>()
		.expect("No tooltip_signals context provider");

	let is_overflow_label = create_rw_signal(false);

	(
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
			.on_event_cont(EventListener::PointerEnter, move |_| {
				if is_overflow_label.get() {
					tooltip_signals.show(title_value.get());
				}
			})
			.on_event_cont(EventListener::PointerLeave, move |_| {
				tooltip_signals.hide();
			}),
		title_input
			.style(move |s| {
				s.width(LABEL_WIDTH)
					.height(24)
					.display(Display::None)
					.apply_if(title_editable.get() && is_dyn_field, |s| s.flex())
			})
			.on_event_cont(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if key == PhysicalKey::Code(KeyCode::Escape) {
					field_value.set(reset_text.get());
					doc.edit_single(
						Selection::region(0, doc.text().len()),
						&reset_text.get(),
						EditType::DeleteSelection,
					);
					title_editable.set(false);
				}

				if is_submit(key) {
					on_save();
				}
			}),
	)
		.style(move |s| {
			s.flex()
				.width(LABEL_WIDTH)
				.justify_content(AlignContent::End)
				.items_center()
		})
}
