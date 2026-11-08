use std::rc::Rc;

use floem::{
	event::listener::{self},
	reactive::{Context, RwSignal, SignalGet, SignalUpdate},
	style::{AlignContent, Display},
	text::TextOverflowChanged,
	ui_events::keyboard::Code,
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
	details::detail_view::LABEL_WIDTH, focus::has_focus, keyboard::is_submit,
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
			.on_event_cont(
				TextOverflowChanged::listener(),
				move |_, overflow_event| {
					let is_overflown = overflow_event.is_overflowing;
					is_overflow_label.set(is_overflown);
				},
			)
			.on_event_cont(listener::PointerEnter, move |_, _| {
				if is_overflow_label.get() {
					tooltip_signals.show(title_value.get());
				}
			})
			.on_event_cont(listener::PointerLeave, move |_, _| {
				tooltip_signals.hide();
			}),
		title_input
			.style(move |s| {
				s.width(LABEL_WIDTH)
					.height(24)
					.display(Display::None)
					.apply_if(title_editable.get() && is_dyn_field, |s| s.flex())
			})
			.on_event_cont(listener::KeyDown, move |cx, event| {
				if !has_focus(cx) {
					return;
				}

				let key = event.code;

				if key == Code::Escape {
					field_value.set(reset_text.get());
					doc.edit_single(
						Selection::region(0, doc.text().len(), CursorAffinity::Forward),
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
