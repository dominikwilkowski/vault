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
			button::icon_button, input_field::input_field, tooltip::TooltipSignals,
		},
	},
};

struct SaveNewField {
	pub id: usize,
	pub title_value: RwSignal<String>,
	pub field_value: RwSignal<String>,
	pub set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	pub show_add_btn: RwSignal<bool>,
	pub show_minus_btn: RwSignal<bool>,
	pub tooltip_signals: TooltipSignals,
	pub config: Config,
}

fn save_new_field(params: SaveNewField) {
	let SaveNewField {
		id,
		title_value,
		field_value,
		set_dyn_field_list,
		show_add_btn,
		show_minus_btn,
		tooltip_signals,
		config,
	} = params;

	let field_list: im::Vector<DbFields> = config
		.db
		.write()
		.unwrap()
		.add_dyn_field(&id, title_value.get(), field_value.get())
		.into();
	set_dyn_field_list.set(field_list);
	tooltip_signals.hide();
	show_add_btn.set(true);
	show_minus_btn.set(false);
	title_value.set(String::from(""));
	field_value.set(String::from(""));
}

pub fn new_field(
	id: usize,
	set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	tooltip_signals: TooltipSignals,
	main_scroll_to: RwSignal<f32>,
	config: Config,
) -> impl View {
	let show_add_btn = create_rw_signal(true);
	let show_minus_btn = create_rw_signal(false);
	let title_value = create_rw_signal(String::from(""));
	let field_value = create_rw_signal(String::from(""));

	let add_icon = include_str!("../icons/add.svg");
	let minus_icon = include_str!("../icons/minus.svg");
	let save_icon = include_str!("../icons/save.svg");

	let config_enter_title = config.clone();
	let config_enter_field = config.clone();
	let config_btn = config.clone();

	let input = input_field(field_value);
	let input_id = input.id();

	v_stack((
		h_stack((
			dyn_field_title_form(
				DynFieldTitleForm {
					title_value,
					title_editable: show_minus_btn,
					title_not_editable: show_add_btn,
					field_value: create_rw_signal(String::from("")),
					reset_text: create_rw_signal(String::from("")),
					is_dyn_field: true,
				},
				move || {
					save_new_field(SaveNewField {
						id,
						title_value,
						field_value,
						set_dyn_field_list,
						show_add_btn,
						show_minus_btn,
						tooltip_signals,
						config: config_enter_title.clone(),
					});
				},
			),
			input
				.style(move |s| s.width(INPUT_LINE_WIDTH).padding_right(30))
				.on_event(EventListener::KeyDown, move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};

					if key == PhysicalKey::Code(KeyCode::Escape) {
						field_value.set(String::from(""));
						show_add_btn.set(true);
						show_minus_btn.set(false);
					}

					if key == PhysicalKey::Code(KeyCode::Enter) {
						save_new_field(SaveNewField {
							id,
							title_value,
							field_value,
							set_dyn_field_list,
							show_add_btn,
							show_minus_btn,
							tooltip_signals,
							config: config_enter_field.clone(),
						});
					}
					EventPropagation::Continue
				}),
			container(
				icon_button(
					String::from(save_icon),
					create_rw_signal(true),
					move |_| {
						save_new_field(SaveNewField {
							id,
							title_value,
							field_value,
							set_dyn_field_list,
							show_add_btn,
							show_minus_btn,
							tooltip_signals,
							config: config_btn.clone(),
						});
					},
				)
				.on_event(EventListener::PointerEnter, move |_event| {
					tooltip_signals.show(String::from("Save to database"));
					EventPropagation::Continue
				})
				.on_event(EventListener::PointerLeave, move |_| {
					tooltip_signals.hide();
					EventPropagation::Continue
				}),
			)
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
		icon_button(String::from(add_icon), show_add_btn, move |_| {
			main_scroll_to.set(100.0);
			tooltip_signals.hide();
			show_add_btn.set(false);
			show_minus_btn.set(true);
			input_id.request_focus();
		})
		.on_event(EventListener::PointerEnter, move |_event| {
			tooltip_signals.show(String::from("Add a new field"));
			EventPropagation::Continue
		})
		.on_event(EventListener::PointerLeave, move |_| {
			tooltip_signals.hide();
			EventPropagation::Continue
		}),
		icon_button(String::from(minus_icon), show_minus_btn, move |_| {
			tooltip_signals.hide();
			show_add_btn.set(true);
			show_minus_btn.set(false);
			title_value.set(String::from(""));
		})
		.on_event(EventListener::PointerEnter, move |_event| {
			tooltip_signals.show(String::from("Hide the new field form"));
			EventPropagation::Continue
		})
		.on_event(EventListener::PointerLeave, move |_| {
			tooltip_signals.hide();
			EventPropagation::Continue
		}),
	))
	.style(|s| s.align_items(AlignItems::Center).width_full().gap(4.0, 0.0))
}
