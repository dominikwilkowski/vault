use std::rc::Rc;

use floem::{
	event::EventListener,
	reactive::{Context, RwSignal, SignalGet, SignalUpdate},
	style::{AlignContent, Display},
	views::{
		editor::{
			core::{cursor::CursorAffinity, editor::EditType, selection::Selection},
			text::Document,
		},
		Decorators, Label, TextInput,
	},
	IntoView,
};

use crate::ui::{
	details::detail_view::LABEL_WIDTH, primitives::tooltip::TooltipSignals,
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

	let tooltip_signals = Context::get::<TooltipSignals>()
		.expect("No tooltip_signals context provider");

	let is_overflow_label = RwSignal::new(false);

	(
		Label::derived(move || title_value.get())
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
			.on_enter(move || {
				on_save();
			})
			.on_event_cont(EventListener::FocusLost, move |_| {
				field_value.set(reset_text.get());
				doc.edit_single(
					Selection::region(0, doc.text().len(), CursorAffinity::Forward),
					&reset_text.get(),
					EditType::DeleteSelection,
				);
				title_editable.set(false);
			})
			.style(move |s| {
				s.width(LABEL_WIDTH)
					.height(24)
					.display(Display::None)
					.apply_if(title_editable.get() && is_dyn_field, |s| s.flex())
			}),
	)
		.style(move |s| {
			s.flex()
				.width(LABEL_WIDTH)
				.justify_content(AlignContent::End)
				.items_center()
		})
}
