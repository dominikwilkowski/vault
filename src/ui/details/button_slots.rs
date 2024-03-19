use std::{rc::Rc, sync::Arc};

use floem::{
	id::Id,
	kurbo::Size,
	reactive::{create_effect, create_rw_signal, use_context, RwSignal},
	view::View,
	views::{
		container,
		editor::{
			core::{editor::EditType, selection::Selection},
			text::Document,
		},
		empty, h_stack, label, Decorators,
	},
	Clipboard,
};

use crate::{
	db::{Db, DbFields},
	env::Environment,
	ui::{
		details::detail_view::{
			save_edit, SaveEdit, SECRET_MULTILINE_PLACEHOLDER, SECRET_PLACEHOLDER,
		},
		history_view::history_view,
		primitives::{
			button::{icon_button, IconButton},
			que::Que,
			tooltip::TooltipSignals,
		},
		window_management::{
			closing_window, make_field_path, opening_window, WindowSpec,
		},
	},
};

pub fn empty_button_slot() -> impl View {
	container(label(|| "")).style(|s| s.width(28.5))
}

pub struct EditButtonSlot {
	pub id: usize,
	pub field: DbFields,
	pub switch: RwSignal<bool>,
	pub is_hidden: bool,
	pub is_secret: bool,
	pub is_multiline: bool,
	pub input_id: Id,
	pub dates: RwSignal<Vec<(usize, u64)>>,
	pub field_value: RwSignal<String>,
	pub multiline_field_value: RwSignal<Rc<dyn Document>>,
	pub reset_text: RwSignal<String>,
	pub view_button_switch: RwSignal<bool>,
}

pub fn edit_button_slot(param: EditButtonSlot) -> impl View {
	let EditButtonSlot {
		id,
		field,
		switch,
		is_hidden,
		is_secret,
		is_multiline,
		input_id,
		dates,
		field_value,
		multiline_field_value,
		reset_text,
		view_button_switch,
	} = param;

	let env = use_context::<Environment>().expect("No env context provider");
	let tooltip_signals = use_context::<TooltipSignals>()
		.expect("No tooltip_signals context provider");

	let edit_icon = include_str!("../icons/edit.svg");
	let save_icon = include_str!("../icons/save.svg");

	let doc = multiline_field_value.get();

	if is_hidden {
		empty_button_slot().any()
	} else {
		container(icon_button(
			IconButton {
				icon: String::from(edit_icon),
				icon2: Some(String::from(save_icon)),
				tooltip: String::from("Edit this field"),
				tooltip2: Some(String::from("Save to database")),
				switch: Some(switch),
				tooltip_signals,
				..IconButton::default()
			},
			move |_| {
				let doc_save = multiline_field_value.get();

				view_button_switch.set(false);
				if switch.get() {
					reset_text.set(if is_multiline {
						String::from(doc.text())
					} else {
						field_value.get()
					});

					if is_secret {
						match is_multiline {
							true => {
								doc.edit_single(
									Selection::region(0, doc.text().len()),
									&env.db.get_last_by_field(&id, &field),
									EditType::DeleteSelection,
								);
							},
							false => field_value.set(env.db.get_last_by_field(&id, &field)),
						}
					}
					input_id.request_focus();
				} else {
					save_edit(SaveEdit {
						id,
						field,
						value: field_value,
						doc: doc_save,
						dates,
						is_secret,
						is_multiline,
						input_id,
					});
				}
			},
		))
		.any()
	}
}

pub struct ViewButtonSlot {
	pub switch: RwSignal<bool>,
	pub is_shown: bool,
	pub is_multiline: bool,
	pub field_value: RwSignal<String>,
}

pub fn view_button_slot(
	param: ViewButtonSlot,
	getter: impl Fn() -> String + 'static,
) -> impl View {
	let ViewButtonSlot {
		switch,
		is_shown,
		is_multiline,
		field_value,
	} = param;

	let tooltip_signals = use_context::<TooltipSignals>()
		.expect("No tooltip_signals context provider");

	let see_icon = include_str!("../icons/see.svg");
	let hide_icon = include_str!("../icons/hide.svg");

	if is_shown {
		h_stack((icon_button(
			IconButton {
				icon: String::from(see_icon),
				icon2: Some(String::from(hide_icon)),
				tooltip: String::from("See contents of field"),
				tooltip2: Some(String::from("Hide contents of field")),
				switch: Some(switch),
				tooltip_signals,
				..IconButton::default()
			},
			move |_| {
				if switch.get() {
					let data = getter();
					field_value.set(data);
				} else {
					field_value.set(if is_multiline {
						String::from(SECRET_MULTILINE_PLACEHOLDER)
					} else {
						String::from(SECRET_PLACEHOLDER)
					});
				}
			},
		),))
		.any()
	} else {
		empty_button_slot().any()
	}
}

pub fn clipboard_button_slot(
	getter: impl Fn() -> String + 'static,
) -> impl View {
	let tooltip_signals = use_context::<TooltipSignals>()
		.expect("No tooltip_signals context provider");

	let clipboard_icon = include_str!("../icons/clipboard.svg");

	icon_button(
		IconButton {
			icon: String::from(clipboard_icon),
			tooltip: String::from("Copy to clipboard"),
			tooltip_signals,
			..IconButton::default()
		},
		move |_| {
			let data = getter();
			let _ = Clipboard::set_contents(data);
		},
	)
}

pub struct HistoryButtonSlot {
	pub id: usize,
	pub field: DbFields,
	pub dates: RwSignal<Vec<(usize, u64)>>,
	pub is_shown: bool,
	pub field_title: String,
	pub db: Arc<Db>,
}

pub fn history_button_slot(param: HistoryButtonSlot) -> impl View {
	let HistoryButtonSlot {
		id,
		field,
		dates,
		is_shown,
		field_title,
		db,
	} = param;

	let tooltip_signals = use_context::<TooltipSignals>()
		.expect("No tooltip_signals context provider");

	let history_icon = include_str!("../icons/history.svg");
	let hide_history_icon = include_str!("../icons/hide_history.svg");

	let hide_history_button_visible = create_rw_signal(false);
	let dates_len = create_rw_signal(dates.get().len());

	create_effect(move |_| {
		dates_len.set(dates.get().len());
	});

	if is_shown {
		let db_history = db.clone();

		container(icon_button(
			IconButton {
				icon: String::from(history_icon),
				icon2: Some(String::from(hide_history_icon)),
				bubble: Some(dates_len),
				tooltip: String::from("See history of field"),
				tooltip2: Some(String::from("Hide history of field")),
				switch: Some(hide_history_button_visible),
				tooltip_signals,
				..IconButton::default()
			},
			move |_| {
				if hide_history_button_visible.get() {
					let db_history_inner = db_history.clone();
					let window_title = format!("{} Field History", field_title);
					let dates_window = dates.get();
					let que_history = Que::default();
					let tooltip_signals_history = TooltipSignals::new(que_history);

					opening_window(
						move || {
							history_view(
								id,
								field,
								dates_window.clone(),
								tooltip_signals_history,
								db_history_inner.clone(),
							)
						},
						WindowSpec {
							id: make_field_path(id, &field),
							title: window_title,
						},
						Size::new(350.0, 300.0),
						move || {
							que_history.unque_all_tooltips();
							hide_history_button_visible.set(false);
						},
					);
				} else {
					closing_window(make_field_path(id, &field), || {});
				}
			},
		))
		.any()
	} else {
		empty().any()
	}
}

pub struct DeleteButtonSlot {
	pub id: usize,
	pub field: DbFields,
	pub hidden_field_list: RwSignal<im::Vector<DbFields>>,
	pub field_list: RwSignal<im::Vector<DbFields>>,
	pub hidden_field_len: RwSignal<usize>,
	pub is_dyn_field: bool,
	pub is_hidden: bool,
}

pub fn delete_button_slot(param: DeleteButtonSlot) -> impl View {
	let DeleteButtonSlot {
		id,
		field,
		hidden_field_list,
		field_list,
		hidden_field_len,
		is_dyn_field,
		is_hidden,
	} = param;

	let env = use_context::<Environment>().expect("No env context provider");
	let tooltip_signals = use_context::<TooltipSignals>()
		.expect("No tooltip_signals context provider");

	let delete_icon = include_str!("../icons/delete.svg");
	let add_icon = include_str!("../icons/add.svg");

	if is_dyn_field {
		container(icon_button(
			IconButton {
				icon: if is_hidden {
					String::from(add_icon)
				} else {
					String::from(delete_icon)
				},
				icon2: None,
				tooltip: if is_hidden {
					String::from("Unarchive this field")
				} else {
					String::from("Archive this field")
				},
				tooltip2: None,
				switch: None,
				tooltip_signals,
				..IconButton::default()
			},
			move |_| {
				tooltip_signals.hide();
				if is_hidden {
					let hidden_field_list_db: im::Vector<DbFields> =
						env.db.edit_field_visbility(&id, &field, true).into();
					hidden_field_len.set(hidden_field_list_db.len());
					hidden_field_list.set(hidden_field_list_db);
					let field_list_db: im::Vector<DbFields> =
						env.db.get_visible_fields(&id).into();
					field_list.set(field_list_db);
				} else {
					let hidden_field_list_db: im::Vector<DbFields> =
						env.db.edit_field_visbility(&id, &field, false).into();
					hidden_field_len.set(hidden_field_list_db.len());
					hidden_field_list.set(hidden_field_list_db);
					let field_list_db: im::Vector<DbFields> =
						env.db.get_visible_fields(&id).into();
					field_list.set(field_list_db);
				}
				let _ = env.db.save();
			},
		))
		.any()
	} else {
		empty().any()
	}
}
