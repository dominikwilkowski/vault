use floem::{
	id::Id,
	kurbo::Size,
	reactive::{create_effect, create_rw_signal, RwSignal, WriteSignal},
	view::View,
	views::{container, h_stack, label, Decorators},
	Clipboard,
};

use crate::{
	config::Config,
	db::DbFields,
	ui::{
		details::detail_view::{save_edit, SaveEdit, SECRET_PLACEHOLDER},
		history_view::history_view,
		primitives::{
			button::{icon_button, IconButton},
			tooltip::TooltipSignals,
		},
		window_management::{
			closing_window, make_field_path, opening_window, WindowSpec,
		},
	},
};

pub struct EditButtonSlot {
	pub id: usize,
	pub field: DbFields,
	pub switch: RwSignal<bool>,
	pub is_hidden: bool,
	pub is_secret: bool,
	pub input_id: Id,
	pub dates: RwSignal<Vec<(usize, u64)>>,
	pub field_value: RwSignal<String>,
	pub reset_text: RwSignal<String>,
	pub set_list: WriteSignal<im::Vector<(usize, &'static str, usize)>>,
	pub view_button_switch: RwSignal<bool>,
	pub tooltip_signals: TooltipSignals,
	pub config: Config,
}

pub fn edit_button_slot(param: EditButtonSlot) -> impl View {
	let EditButtonSlot {
		id,
		field,
		switch,
		is_hidden,
		is_secret,
		input_id,
		dates,
		field_value,
		reset_text,
		set_list,
		view_button_switch,
		tooltip_signals,
		config,
	} = param;
	let edit_icon = include_str!("../icons/edit.svg");
	let save_icon = include_str!("../icons/save.svg");

	if is_hidden {
		container(label(|| "")).style(|s| s.width(26.5))
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
				view_button_switch.set(false);
				if switch.get() {
					reset_text.set(field_value.get());
					if is_secret {
						field_value.set(config.db.read().get_last_by_field(&id, &field));
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
						config: config.clone(),
					});
				}
			},
		))
	}
}

pub struct ViewButtonSlot {
	pub switch: RwSignal<bool>,
	pub is_secret: bool,
	pub tooltip_signals: TooltipSignals,
	pub field_value: RwSignal<String>,
}

pub fn view_button_slot(
	param: ViewButtonSlot,
	getter: impl Fn() -> String + 'static,
) -> impl View {
	let ViewButtonSlot {
		switch,
		is_secret,
		tooltip_signals,
		field_value,
	} = param;

	let see_icon = include_str!("../icons/see.svg");
	let hide_icon = include_str!("../icons/hide.svg");

	if is_secret {
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
					field_value.set(String::from(SECRET_PLACEHOLDER));
				}
			},
		),))
	} else {
		h_stack((label(|| ""),))
	}
}

pub fn clipboard_button_slot(
	tooltip_signals: TooltipSignals,
	getter: impl Fn() -> String + 'static,
) -> impl View {
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
	pub is_secret: bool,
	pub field_title: String,
	pub tooltip_signals: TooltipSignals,
	pub config: Config,
}

pub fn history_button_slot(param: HistoryButtonSlot) -> impl View {
	let HistoryButtonSlot {
		id,
		field,
		dates,
		is_secret,
		field_title,
		tooltip_signals,
		config,
	} = param;
	let history_icon = include_str!("../icons/history.svg");
	let hide_history_icon = include_str!("../icons/hide_history.svg");

	let hide_history_btn_visible = create_rw_signal(false);
	let dates_len = create_rw_signal(dates.get().len());

	create_effect(move |_| {
		dates.track();
		dates_len.set(dates.get().len());
	});

	if is_secret {
		let config_history = config.clone();

		container(icon_button(
			IconButton {
				icon: String::from(history_icon),
				icon2: Some(String::from(hide_history_icon)),
				bubble: Some(dates_len),
				tooltip: String::from("See history of field"),
				tooltip2: Some(String::from("Hide history of field")),
				switch: Some(hide_history_btn_visible),
				tooltip_signals,
				..IconButton::default()
			},
			move |_| {
				if hide_history_btn_visible.get() {
					let config_history_inner = config_history.clone();
					let window_title = format!("{} Field History", field_title);
					let dates_window = dates.get();

					opening_window(
						move || {
							history_view(
								id,
								field,
								dates_window.clone(),
								config_history_inner.clone(),
							)
						},
						WindowSpec {
							id: make_field_path(id, &field),
							title: window_title,
						},
						Size::new(350.0, 300.0),
						move || {
							hide_history_btn_visible.set(false);
						},
					);
				} else {
					closing_window(make_field_path(id, &field), || {});
				}
			},
		))
	} else {
		container(label(|| ""))
	}
}

pub struct DeleteButtonSlot {
	pub id: usize,
	pub field: DbFields,
	pub set_hidden_field_list: WriteSignal<im::Vector<DbFields>>,
	pub set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	pub hidden_field_len: RwSignal<usize>,
	pub is_dyn_field: bool,
	pub is_hidden: bool,
	pub tooltip_signals: TooltipSignals,
	pub config: Config,
}

pub fn delete_button_slot(param: DeleteButtonSlot) -> impl View {
	let DeleteButtonSlot {
		id,
		field,
		set_hidden_field_list,
		set_dyn_field_list,
		hidden_field_len,
		is_dyn_field,
		is_hidden,
		tooltip_signals,
		config,
	} = param;
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
					let hidden_field_list: im::Vector<DbFields> = config
						.db
						.write()
						.edit_dyn_field_visbility(&id, &field, true)
						.into();
					hidden_field_len.set(hidden_field_list.len());
					set_hidden_field_list.set(hidden_field_list);
					let field_list: im::Vector<DbFields> =
						config.db.read().get_dyn_fields(&id).into();
					set_dyn_field_list.set(field_list);
				} else {
					let hidden_field_list: im::Vector<DbFields> = config
						.db
						.write()
						.edit_dyn_field_visbility(&id, &field, false)
						.into();
					hidden_field_len.set(hidden_field_list.len());
					set_hidden_field_list.set(hidden_field_list);
					let field_list: im::Vector<DbFields> =
						config.db.read().get_dyn_fields(&id).into();
					set_dyn_field_list.set(field_list);
				}
			},
		))
	} else {
		container(label(|| ""))
	}
}
