use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal, WriteSignal},
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
				view_button_slot, DeleteButtonSlot, HistoryButtonSlot,
			},
			detail_view::{
				save_edit, SaveEdit, INPUT_LINE_WIDTH, LINE_WIDTH, SECRET_PLACEHOLDER,
			},
			dyn_field_title_form::{dyn_field_title_form, DynFieldTitleForm},
		},
		primitives::{
			button::{icon_button, IconButton},
			input_field::input_field,
			tooltip::TooltipSignals,
		},
	},
};

pub struct ListItem {
	pub id: usize,
	pub field: DbFields,
	pub set_hidden_field_list: WriteSignal<im::Vector<DbFields>>,
	pub set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	pub hidden_field_len: RwSignal<usize>,
	pub is_secret: bool,
	pub is_hidden: bool,
	pub tooltip_signals: TooltipSignals,
	pub set_list: WriteSignal<im::Vector<(usize, &'static str, usize)>>,
	pub config: Config,
}

pub fn list_item(param: ListItem) -> impl View {
	let ListItem {
		id,
		field,
		set_hidden_field_list,
		set_dyn_field_list,
		hidden_field_len,
		is_secret,
		is_hidden,
		tooltip_signals,
		set_list,
		config,
	} = param;

	let save_btn_visible = create_rw_signal(false);
	let reset_text = create_rw_signal(String::from(""));
	let dates =
		create_rw_signal(config.db.read().unwrap().get_history_dates(&id, &field));

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
					save_btn_visible.set(false);
				}

				if key == PhysicalKey::Code(KeyCode::Enter) {
					save_btn_visible.set(false);
					config_submit.db.write().unwrap().edit_dyn_field_title(
						&id,
						&field,
						title_value.get(),
					);
					save_edit(SaveEdit {
						id,
						field,
						value: field_value,
						dates,
						is_secret,
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

	let edit_slot = if is_hidden {
		container(label(|| "")).style(|s| s.width(26.5))
	} else {
		container(icon_button(
			IconButton {
				icon: String::from(edit_icon),
				icon2: Some(String::from(save_icon)),
				bubble: None::<RwSignal<Vec<u8>>>,
				tooltip: String::from("Edit this field"),
				tooltip2: Some(String::from("Save to database")),
				switch: Some(save_btn_visible),
				tooltip_signals,
			},
			move |_| {
				if save_btn_visible.get() {
					reset_text.set(field_value.get());
					if is_secret {
						field_value.set(
							config_edit.db.read().unwrap().get_last_by_field(&id, &field),
						);
					}
					input_id.request_focus();
				} else {
					save_edit(SaveEdit {
						id,
						field,
						value: field_value,
						dates,
						is_secret,
						input_id,
						set_list,
						config: config_edit.clone(),
					});
				}
			},
		))
	};

	h_stack((
		dyn_field_title_form(
			DynFieldTitleForm {
				title_value,
				title_editable: save_btn_visible,
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
					dates,
					is_secret,
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
						.apply_if(is_hidden, |s| s.border_color(C_TEXT_MAIN_INACTIVE))
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
		edit_slot,
		clipboard_button_slot(tooltip_signals, move || {
			config.db.read().unwrap().get_last_by_field(&id, &field)
		}),
		view_button_slot(is_secret, tooltip_signals, field_value, move || {
			config_viewbtn.db.read().unwrap().get_last_by_field(&id, &field)
		}),
		history_button_slot(HistoryButtonSlot {
			id,
			field,
			dates,
			is_secret,
			field_title,
			tooltip_signals,
			config: config_history,
		}),
		delete_button_slot(DeleteButtonSlot {
			id,
			field,
			set_hidden_field_list,
			set_dyn_field_list,
			hidden_field_len,
			is_dyn_field,
			is_hidden,
			tooltip_signals,
			config: config_deletebtn,
		}),
	))
	.style(move |s| {
		s.align_items(AlignItems::Center)
			.width_full()
			.gap(4.0, 0.0)
			.width(LINE_WIDTH)
			.apply_if(is_hidden, |s| s.color(C_TEXT_MAIN_INACTIVE))
	})
}
