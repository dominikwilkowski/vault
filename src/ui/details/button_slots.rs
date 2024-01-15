use floem::{
	kurbo::Size,
	reactive::{create_rw_signal, RwSignal, WriteSignal},
	view::View,
	views::{container, h_stack, label},
	Clipboard,
};

use crate::{
	config::Config,
	db::DbFields,
	ui::{
		details::detail_view::SECRET_PLACEHOLDER,
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

pub fn view_button_slot(
	is_secret: bool,
	tooltip_signals: TooltipSignals,
	value: RwSignal<String>,
	getter: impl Fn() -> String + 'static,
) -> impl View {
	let switch = create_rw_signal(false);

	let see_icon = include_str!("../icons/see.svg");
	let hide_icon = include_str!("../icons/hide.svg");

	if is_secret {
		h_stack((icon_button(
			IconButton {
				icon: String::from(see_icon),
				icon2: Some(String::from(hide_icon)),
				bubble: None::<RwSignal<Vec<u8>>>,
				tooltip: String::from("See contents of field"),
				tooltip2: Some(String::from("Hide contents of field")),
				switch: Some(switch),
				tooltip_signals,
			},
			move |_| {
				if switch.get() {
					let data = getter();
					value.set(data);
				} else {
					value.set(String::from(SECRET_PLACEHOLDER));
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
			icon2: None,
			bubble: None::<RwSignal<Vec<u8>>>,
			tooltip: String::from("Copy to clipboard"),
			tooltip2: None,
			switch: None,
			tooltip_signals,
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

	if is_secret {
		let config_history = config.clone();

		container(icon_button(
			IconButton {
				icon: String::from(history_icon),
				icon2: Some(String::from(hide_history_icon)),
				bubble: Some(dates),
				tooltip: String::from("See history of field"),
				tooltip2: Some(String::from("Hide history of field")),
				switch: Some(hide_history_btn_visible),
				tooltip_signals,
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
				bubble: None::<RwSignal<Vec<u8>>>,
				tooltip: if is_hidden {
					String::from("Unarchive this field")
				} else {
					String::from("Archive this field")
				},
				tooltip2: None,
				switch: None,
				tooltip_signals,
			},
			move |_| {
				tooltip_signals.hide();
				if is_hidden {
					let hidden_field_list: im::Vector<DbFields> = config
						.db
						.write()
						.unwrap()
						.edit_dyn_field_visbility(&id, &field, true)
						.into();
					hidden_field_len.set(hidden_field_list.len());
					set_hidden_field_list.set(hidden_field_list);
					let field_list: im::Vector<DbFields> =
						config.db.read().unwrap().get_dyn_fields(&id).into();
					set_dyn_field_list.set(field_list);
				} else {
					let hidden_field_list: im::Vector<DbFields> = config
						.db
						.write()
						.unwrap()
						.edit_dyn_field_visbility(&id, &field, false)
						.into();
					hidden_field_len.set(hidden_field_list.len());
					set_hidden_field_list.set(hidden_field_list);
					let field_list: im::Vector<DbFields> =
						config.db.read().unwrap().get_dyn_fields(&id).into();
					set_dyn_field_list.set(field_list);
				}
			},
		))
	} else {
		container(label(|| ""))
	}
}
