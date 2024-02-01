use floem::{
	event::EventListener,
	id::Id,
	reactive::{
		create_rw_signal, create_signal, ReadSignal, RwSignal, WriteSignal,
	},
	style::{AlignContent, AlignItems},
	view::View,
	views::{
		h_stack, label, svg, v_stack, virtual_stack, Decorators, VirtualDirection,
		VirtualItemSize,
	},
	EventPropagation,
};

use crate::{
	config::{Config, PresetFields},
	db::DbFields,
	ui::{
		details::{
			hidden_fields::{hidden_fields, HiddeFields},
			list_item::{list_item, ListItem},
			new_field::new_field,
		},
		primitives::tooltip::TooltipSignals,
	},
	Que,
};

pub const SECRET_PLACEHOLDER: &str = "••••••••••••••••";
pub const INPUT_LINE_WIDTH: f64 = 250.0;
pub const LABEL_WIDTH: f64 = 142.0;
pub const LINE_WIDTH: f64 = 550.0;

pub struct SaveEdit {
	pub id: usize,
	pub field: DbFields,
	pub value: RwSignal<String>,
	pub dates: RwSignal<Vec<(usize, u64)>>,
	pub is_secret: bool,
	pub input_id: Id,
	pub set_list: WriteSignal<im::Vector<(usize, &'static str, usize)>>,
	pub config: Config,
}

pub fn save_edit(params: SaveEdit) {
	let SaveEdit {
		id,
		field,
		value,
		dates,
		is_secret,
		input_id,
		set_list,
		config,
	} = params;

	let last_val = config.db.read().get_last_by_field(&id, &field);
	if last_val != value.get() {
		config.db.write().edit_field(id, &field, value.get());
		let _ = config.save();
		if field == DbFields::Title {
			let new_list = config.db.read().get_list();
			set_list.update(|list: &mut im::Vector<(usize, &'static str, usize)>| {
				*list = new_list;
			});
		}

		dates.set(config.db.read().get_history_dates(&id, &field));
		input_id.request_focus();
	}

	if is_secret {
		value.set(String::from(SECRET_PLACEHOLDER));
	}
}

pub struct DetailView {
	pub id: usize,
	pub field_presets: RwSignal<PresetFields>,
	pub main_scroll_to: RwSignal<f32>,
	pub tooltip_signals: TooltipSignals,
	pub set_list: WriteSignal<im::Vector<(usize, &'static str, usize)>>,
	pub list: ReadSignal<im::Vector<(usize, &'static str, usize)>>,
	pub que: Que,
	pub config: Config,
}

pub fn detail_view(param: DetailView) -> impl View {
	let DetailView {
		id,
		field_presets,
		main_scroll_to,
		tooltip_signals,
		set_list,
		list,
		que,
		config,
	} = param;
	let is_overflowing = create_rw_signal(false);

	let password_icon = include_str!("../icons/password.svg");

	let field_list: im::Vector<DbFields> =
		config.db.read().get_dyn_fields(&id).into();
	let (dyn_field_list, set_dyn_field_list) = create_signal(field_list);

	let hidden_field_list: im::Vector<DbFields> =
		config.db.read().get_hidden_dyn_fields(&id).into();
	let hidden_field_len = create_rw_signal(hidden_field_list.len());
	let (hidden_field_list, set_hidden_field_list) =
		create_signal(hidden_field_list);

	let config_fields = config.clone();

	v_stack((
		h_stack((
			svg(move || String::from(password_icon))
				.style(|s| s.width(24).height(24).min_width(24)),
			label(move || {
				list
					.get()
					.iter()
					.find(|item| item.0 == id)
					.unwrap_or(&(0, "Details", 0))
					.1
			})
			.on_text_overflow(move |is_overflown| {
				is_overflowing.set(is_overflown);
			})
			.on_event(EventListener::PointerEnter, move |_| {
				if is_overflowing.get() {
					tooltip_signals.show(String::from(
						list
							.get()
							.iter()
							.find(|item| item.0 == id)
							.unwrap_or(&(0, "Details", 0))
							.1,
					));
				}
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				tooltip_signals.hide();
				EventPropagation::Continue
			})
			.style(|s| s.text_ellipsis().font_size(24.0).max_width_full()),
		))
		.style(|s| {
			s.flex()
				.flex_row()
				.align_items(AlignItems::Center)
				.max_width_pct(90.0)
				.gap(5, 0)
				.margin(5)
				.margin_right(20)
				.margin_top(15)
				.margin_bottom(20)
		}),
		v_stack((
			list_item(ListItem {
				id,
				field: DbFields::Title,
				set_hidden_field_list,
				set_dyn_field_list,
				hidden_field_len,
				is_hidden: false,
				tooltip_signals,
				set_list,
				que,
				config: config.clone(),
			}),
			virtual_stack(
				VirtualDirection::Vertical,
				VirtualItemSize::Fixed(Box::new(|| 35.0)),
				move || dyn_field_list.get(),
				move |item| *item,
				move |field| {
					list_item(ListItem {
						id,
						field,
						set_hidden_field_list,
						set_dyn_field_list,
						hidden_field_len,
						is_hidden: false,
						tooltip_signals,
						set_list,
						que,
						config: config_fields.clone(),
					})
					.style(|s| s.padding_bottom(5))
				},
			)
			.style(|s| s.margin_bottom(10)),
			hidden_fields(HiddeFields {
				id,
				hidden_field_list,
				set_hidden_field_list,
				set_dyn_field_list,
				hidden_field_len,
				tooltip_signals,
				set_list,
				main_scroll_to,
				que,
				config: config.clone(),
			})
			.style(|s| s.margin_bottom(10)),
			new_field(
				id,
				field_presets,
				set_dyn_field_list,
				tooltip_signals,
				main_scroll_to,
				config,
			),
		))
		.style(|s| s.gap(0, 5)),
	))
	.style(|s| {
		s.padding(8.0)
			.width_full()
			.max_width_full()
			.justify_content(AlignContent::Center)
			.align_items(AlignItems::Center)
	})
}
