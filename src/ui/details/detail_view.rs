use floem::{
	id::Id,
	reactive::{create_signal, ReadSignal, RwSignal, WriteSignal},
	style::{AlignContent, AlignItems},
	view::View,
	views::{
		h_stack, label, svg, v_stack, virtual_stack, Decorators, VirtualDirection,
		VirtualItemSize,
	},
};

use crate::{
	config::Config,
	db::DbFields,
	ui::{
		details::{list_item::list_item, new_field::new_field},
		primitives::tooltip::TooltipSignals,
	},
};

pub const SECRET_PLACEHOLDER: &str = "••••••••••••••••";
pub const INPUT_LINE_WIDTH: f64 = 250.0;
pub const LABEL_WIDTH: f64 = 92.0;
const LINE_WIDTH: f64 = 500.0;

pub struct SaveEdit {
	pub id: usize,
	pub field: DbFields,
	pub value: RwSignal<String>,
	pub is_secret: bool,
	pub tooltip_signals: TooltipSignals,
	pub edit_btn_visible: RwSignal<bool>,
	pub save_btn_visible: RwSignal<bool>,
	pub input_id: Id,
	pub set_list: WriteSignal<im::Vector<(usize, &'static str, usize)>>,
	pub config: Config,
}

pub fn save_edit(params: SaveEdit) {
	let SaveEdit {
		id,
		field,
		value,
		is_secret,
		tooltip_signals,
		edit_btn_visible,
		save_btn_visible,
		input_id,
		set_list,
		config,
	} = params;

	config.db.write().unwrap().edit_field(id, &field, value.get());
	if field == DbFields::Title {
		let new_list = config.db.read().unwrap().get_list();
		set_list.update(|list: &mut im::Vector<(usize, &'static str, usize)>| {
			*list = new_list;
		});
	}

	edit_btn_visible.set(true);
	save_btn_visible.set(false);
	tooltip_signals.hide();
	input_id.request_focus();

	if is_secret {
		value.set(String::from(SECRET_PLACEHOLDER));
	}
}

pub struct SaveNewField {
	pub id: usize,
	pub value: RwSignal<String>,
	pub set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	pub show_add_field_line: RwSignal<bool>,
	pub show_add_btn: RwSignal<bool>,
	pub show_minus_btn: RwSignal<bool>,
	pub tooltip_signals: TooltipSignals,
	pub config: Config,
}

pub fn save_new_field(params: SaveNewField) {
	let SaveNewField {
		id,
		value,
		set_dyn_field_list,
		show_add_field_line,
		show_add_btn,
		show_minus_btn,
		tooltip_signals,
		config,
	} = params;

	let field_list: im::Vector<DbFields> =
		config.db.write().unwrap().add_dyn_field(&id, value.get()).into();
	set_dyn_field_list.set(field_list);
	tooltip_signals.hide();
	show_add_field_line.set(false);
	show_add_btn.set(true);
	show_minus_btn.set(false);
	value.set(String::from(""));
}

pub fn detail_view(
	id: usize,
	main_scroll_to: RwSignal<f32>,
	tooltip_signals: TooltipSignals,
	set_list: WriteSignal<im::Vector<(usize, &'static str, usize)>>,
	list: ReadSignal<im::Vector<(usize, &'static str, usize)>>,
	config: Config,
) -> impl View {
	let password_icon = include_str!("../icons/password.svg");

	let field_list: im::Vector<DbFields> =
		config.db.read().unwrap().get_fields(&id).into();
	let (dyn_field_list, set_dyn_field_list) = create_signal(field_list);

	let config_fields = config.clone();

	v_stack((
		h_stack((
			svg(move || String::from(password_icon))
				.style(|s| s.width(24).height(24)),
			label(move || {
				list
					.get()
					.iter()
					.find(|item| item.0 == id)
					.unwrap_or(&(0, "Details", 0))
					.1
			})
			.style(|s| s.font_size(24.0)),
		))
		.style(|s| {
			s.align_items(AlignItems::Center)
				.gap(5, 0)
				.margin_top(15)
				.margin_bottom(20)
		}),
		v_stack((
			list_item(
				id,
				DbFields::Title,
				false,
				tooltip_signals,
				set_list,
				config.clone(),
			),
			list_item(
				id,
				DbFields::Url,
				false,
				tooltip_signals,
				set_list,
				config.clone(),
			),
			list_item(
				id,
				DbFields::Username,
				true,
				tooltip_signals,
				set_list,
				config.clone(),
			),
			list_item(
				id,
				DbFields::Password,
				true,
				tooltip_signals,
				set_list,
				config.clone(),
			),
			virtual_stack(
				VirtualDirection::Vertical,
				VirtualItemSize::Fixed(Box::new(|| 35.0)),
				move || dyn_field_list.get(),
				move |item| *item,
				move |field| {
					list_item(
						id,
						field,
						true,
						tooltip_signals,
						set_list,
						config_fields.clone(),
					)
					.style(|s| s.padding_bottom(5))
				},
			),
			new_field(
				id,
				set_dyn_field_list,
				tooltip_signals,
				main_scroll_to,
				config,
			),
		))
		.style(|s| s.gap(0, 5).width(LINE_WIDTH)),
	))
	.style(|s| {
		s.padding(8.0)
			.width_full()
			.justify_content(AlignContent::Center)
			.align_items(AlignItems::Center)
	})
}
