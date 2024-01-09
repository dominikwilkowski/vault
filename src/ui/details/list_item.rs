use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, WriteSignal},
	style::{AlignItems, CursorStyle, Display, Position},
	view::View,
	views::{container, h_stack, label, svg, Decorators},
	EventPropagation,
};
use url_escape;
use webbrowser;

use crate::{
	config::Config,
	db::DbFields,
	ui::{
		colors::*,
		details::{
			button_slots::{
				clipboard_button_slot, delete_button_slot, history_button_slot,
				view_button_slot,
			},
			detail_view::{
				save_edit, SaveEdit, INPUT_LINE_WIDTH, SECRET_PLACEHOLDER,
			},
			dyn_field_title_form::{dyn_field_title_form, DynFieldTitleForm},
		},
		primitives::{
			button::icon_button, input_field::input_field, tooltip::TooltipSignals,
		},
	},
};

pub fn list_item(
	id: usize,
	field: DbFields,
	is_secret: bool,
	tooltip_signals: TooltipSignals,
	set_list: WriteSignal<im::Vector<(usize, &'static str, usize)>>,
	config: Config,
) -> impl View {
	let edit_btn_visible = create_rw_signal(true);
	let save_btn_visible = create_rw_signal(false);
	let reset_text = create_rw_signal(String::from(""));

	let field_title = match field {
		DbFields::Fields(_) => {
			config.db.read().unwrap().get_name_of_dyn_field(&id, &field)
		}
		other => format!("{}", other),
	};
	let title_value = create_rw_signal(field_title.clone());

	let field_value = if is_secret {
		create_rw_signal(String::from(SECRET_PLACEHOLDER))
	} else {
		create_rw_signal(config.db.read().unwrap().get_last_by_field(&id, &field))
	};

	let is_dyn_field = matches!(field, DbFields::Fields(_));

	let edit_icon = include_str!("../icons/edit.svg");
	let revert_icon = include_str!("../icons/revert.svg");
	let save_icon = include_str!("../icons/save.svg");

	let config_edit = config.clone();
	let config_save = config.clone();
	let config_submit = config.clone();
	let config_title = config.clone();
	let config_viewbtn = config.clone();
	let config_history = config.clone();
	let config_deletebtn = config.clone();

	let input = input_field(field_value);
	let input_id = input.id();

	let title_input = input_field(title_value);

	let input_line = h_stack((
		input
			.style(move |s| {
				s.width(INPUT_LINE_WIDTH)
					.padding_right(30)
					.display(Display::None)
					.apply_if(save_btn_visible.get(), |s| s.display(Display::Flex))
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
					config_submit.db.write().unwrap().edit_dyn_field_title(
						&id,
						&field,
						title_value.get(),
					);
					save_edit(SaveEdit {
						id,
						field,
						value: field_value,
						is_secret,
						tooltip_signals,
						edit_btn_visible,
						save_btn_visible,
						input_id,
						set_list,
						config: config_submit.clone(),
					});
				}
				EventPropagation::Continue
			}),
		container(
			svg(move || String::from(revert_icon)).style(|s| s.width(16).height(16)),
		)
		.on_click(move |_| {
			field_value.set(reset_text.get());
			edit_btn_visible.set(true);
			save_btn_visible.set(false);
			tooltip_signals.hide();
			EventPropagation::Continue
		})
		.on_event(EventListener::PointerEnter, move |_event| {
			tooltip_signals.show(String::from("Revert field"));
			EventPropagation::Continue
		})
		.on_event(EventListener::PointerLeave, move |_| {
			tooltip_signals.hide();
			EventPropagation::Continue
		})
		.style(|s| {
			s.position(Position::Absolute)
				.z_index(5)
				.display(Display::Flex)
				.items_center()
				.justify_center()
				.inset_top(0)
				.inset_right(0)
				.inset_bottom(0)
				.width(30)
				.cursor(CursorStyle::Pointer)
		}),
	));

	h_stack((
		dyn_field_title_form(
			DynFieldTitleForm {
				title_value,
				title_editable: save_btn_visible,
				title_not_editable: edit_btn_visible,
				field_value,
				reset_text,
				is_dyn_field,
				title_input,
			},
			move || {
				if is_dyn_field {
					config_title.db.write().unwrap().edit_dyn_field_title(
						&id,
						&field,
						title_value.get(),
					);
				}
				save_edit(SaveEdit {
					id,
					field,
					value: field_value,
					is_secret,
					tooltip_signals,
					edit_btn_visible,
					save_btn_visible,
					input_id,
					set_list,
					config: config_title.clone(),
				})
			},
		),
		h_stack((
			input_line,
			label(move || field_value.get())
				.style(move |s| {
					s.width(INPUT_LINE_WIDTH)
						.padding_top(5)
						.padding_right(6)
						.padding_left(6)
						.padding_bottom(5)
						.border_bottom(1)
						.border_color(C_TEXT_TOP)
						.display(Display::Flex)
						.apply_if(save_btn_visible.get(), |s| s.display(Display::None))
						.hover(|s| {
							s.apply_if(matches!(field, DbFields::Url), |s| {
								s.color(C_FOCUS).cursor(CursorStyle::Pointer)
							})
						})
				})
				.on_click(move |_| {
					if matches!(field, DbFields::Url) {
						let _ = webbrowser::open(&url_escape::encode_fragment(
							&field_value.get(),
						));
					}
					EventPropagation::Continue
				}),
		)),
		h_stack((
			icon_button(String::from(edit_icon), edit_btn_visible, move |_| {
				reset_text.set(field_value.get());
				edit_btn_visible.set(false);
				save_btn_visible.set(true);
				tooltip_signals.hide();
				if is_secret {
					field_value
						.set(config_edit.db.read().unwrap().get_last_by_field(&id, &field));
				}
				input_id.request_focus();
			}),
			icon_button(String::from(save_icon), save_btn_visible, move |_| {
				save_edit(SaveEdit {
					id,
					field,
					value: field_value,
					is_secret,
					tooltip_signals,
					edit_btn_visible,
					save_btn_visible,
					input_id,
					set_list,
					config: config_save.clone(),
				});
			}),
		))
		.on_event(EventListener::PointerEnter, move |_event| {
			let text = if edit_btn_visible.get() {
				"Edit this field"
			} else {
				"Save to database"
			};
			tooltip_signals.show(String::from(text));
			EventPropagation::Continue
		})
		.on_event(EventListener::PointerLeave, move |_| {
			tooltip_signals.hide();
			EventPropagation::Continue
		}),
		clipboard_button_slot(tooltip_signals, move || {
			config.db.read().unwrap().get_last_by_field(&id, &field)
		}),
		view_button_slot(is_secret, tooltip_signals, field_value, move || {
			config_viewbtn.db.read().unwrap().get_last_by_field(&id, &field)
		}),
		history_button_slot(
			id,
			field,
			is_secret,
			field_title,
			tooltip_signals,
			config_history,
		),
		delete_button_slot(is_dyn_field, tooltip_signals, config_deletebtn),
	))
	.style(|s| s.align_items(AlignItems::Center).width_full().gap(4.0, 0.0))
}
