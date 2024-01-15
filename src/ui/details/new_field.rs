use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal, WriteSignal},
	style::{AlignItems, Display},
	view::View,
	views::{container, h_stack, v_stack, Decorators},
	EventPropagation,
};

use crate::{
	config::Config,
	db::DbFields,
	ui::{
		details::{
			detail_view::{BUTTON_SLOTS_WIDTH, INPUT_LINE_WIDTH},
			dyn_field_title_form::{dyn_field_title_form, DynFieldTitleForm},
		},
		primitives::{
			button::{icon_button, IconButton},
			input_field::input_field,
			tooltip::TooltipSignals,
		},
	},
};

struct SaveNewField {
	pub id: usize,
	pub title_value: RwSignal<String>,
	pub field_value: RwSignal<String>,
	pub set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	pub tooltip_signals: TooltipSignals,
	pub config: Config,
}

fn save_new_field(params: SaveNewField) {
	let SaveNewField {
		id,
		title_value,
		field_value,
		set_dyn_field_list,
		tooltip_signals,
		config,
	} = params;

	if !title_value.get().is_empty() && !field_value.get().is_empty() {
		let field_list: im::Vector<DbFields> = config
			.db
			.write()
			.unwrap()
			.add_dyn_field(&id, title_value.get(), field_value.get())
			.into();
		set_dyn_field_list.set(field_list);
		tooltip_signals.hide();
		title_value.set(String::from(""));
		field_value.set(String::from(""));
	}
}

pub fn new_field(
	id: usize,
	set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	tooltip_signals: TooltipSignals,
	main_scroll_to: RwSignal<f32>,
	config: Config,
) -> impl View {
	let show_minus_btn = create_rw_signal(false);
	let title_value = create_rw_signal(String::from(""));
	let field_value = create_rw_signal(String::from(""));

	let add_icon = include_str!("../icons/add.svg");
	let minus_icon = include_str!("../icons/minus.svg");
	let save_icon = include_str!("../icons/save.svg");

	let config_enter_title = config.clone();
	let config_enter_field = config.clone();
	let config_btn = config.clone();

	let title_input = input_field(title_value);
	let input_id = title_input.id();

	v_stack((
		h_stack((
			dyn_field_title_form(
				DynFieldTitleForm {
					title_value,
					title_editable: show_minus_btn,
					field_value: create_rw_signal(String::from("")),
					reset_text: create_rw_signal(String::from("")),
					is_dyn_field: true,
					title_input,
				},
				move || {
					save_new_field(SaveNewField {
						id,
						title_value,
						field_value,
						set_dyn_field_list,
						tooltip_signals,
						config: config_enter_title.clone(),
					});
				},
			),
			input_field(field_value)
				.style(move |s| s.width(INPUT_LINE_WIDTH).padding_right(30))
				.on_event(EventListener::KeyDown, move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};

					if key == PhysicalKey::Code(KeyCode::Escape) {
						field_value.set(String::from(""));
						show_minus_btn.set(false);
					}

					if key == PhysicalKey::Code(KeyCode::Enter) {
						save_new_field(SaveNewField {
							id,
							title_value,
							field_value,
							set_dyn_field_list,
							tooltip_signals,
							config: config_enter_field.clone(),
						});
					}
					EventPropagation::Continue
				}),
			container(icon_button(
				IconButton {
					icon: String::from(save_icon),
					icon2: None,
					bubble: None::<RwSignal<Vec<u8>>>,
					tooltip: String::from("Save to database"),
					tooltip2: None,
					switch: None,
					tooltip_signals,
				},
				move |_| {
					save_new_field(SaveNewField {
						id,
						title_value,
						field_value,
						set_dyn_field_list,
						tooltip_signals,
						config: config_btn.clone(),
					});
				},
			))
			.style(move |s| {
				s.align_items(AlignItems::Center).width(BUTTON_SLOTS_WIDTH)
			}),
		))
		.style(move |s| {
			s.gap(4.0, 0.0)
				.items_center()
				.justify_center()
				.display(Display::None)
				.apply_if(show_minus_btn.get(), |s| s.display(Display::Flex))
		}),
		icon_button(
			IconButton {
				icon: String::from(add_icon),
				icon2: Some(String::from(minus_icon)),
				bubble: None::<RwSignal<Vec<u8>>>,
				tooltip: String::from("Add a new field"),
				tooltip2: Some(String::from("Hide the new field form")),
				switch: Some(show_minus_btn),
				tooltip_signals,
			},
			move |_| {
				if show_minus_btn.get() {
					main_scroll_to.set(100.0);
					input_id.request_focus();
				} else {
					title_value.set(String::from(""));
				}
			},
		),
	))
	.style(|s| s.align_items(AlignItems::Center).width_full().gap(4.0, 0.0))
}
