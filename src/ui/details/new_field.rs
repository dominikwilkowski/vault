use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal, WriteSignal},
	style::{AlignContent, AlignItems, Display},
	view::View,
	views::{container, h_stack, label, v_stack, Decorators},
	EventPropagation,
};

use crate::{
	config::Config,
	db::DbFields,
	ui::{
		details::detail_view::{
			save_new_field, SaveNewField, INPUT_LINE_WIDTH, LABEL_WIDTH,
		},
		primitives::{
			button::icon_button, input_field::input_field, tooltip::TooltipSignals,
		},
	},
};

pub fn new_field(
	id: usize,
	set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	tooltip_signals: TooltipSignals,
	main_scroll_to: RwSignal<f32>,
	config: Config,
) -> impl View {
	let show_add_field_line = create_rw_signal(false);
	let show_add_btn = create_rw_signal(true);
	let show_minus_btn = create_rw_signal(false);
	let value = create_rw_signal(String::from(""));

	let add_icon = include_str!("../icons/add.svg");
	let minus_icon = include_str!("../icons/minus.svg");
	let save_icon = include_str!("../icons/save.svg");

	let config_enter = config.clone();
	let config_btn = config.clone();

	let input = input_field(value);
	let input_id = input.id();

	v_stack((
		h_stack((
			container(label(move || "Field Name"))
				.style(move |s| {
					s.flex().width(LABEL_WIDTH).justify_content(AlignContent::End)
				})
				.on_click_stop(move |_| {
					input_id.request_focus();
				}),
			input
				.style(|s| s.flex().width(INPUT_LINE_WIDTH).padding_right(30))
				.on_event(EventListener::KeyDown, move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};

					if key == PhysicalKey::Code(KeyCode::Escape) {
						value.set(String::from(""));
						show_add_field_line.set(false);
						show_add_btn.set(true);
						show_minus_btn.set(false);
					}

					if key == PhysicalKey::Code(KeyCode::Enter) {
						save_new_field(SaveNewField {
							id,
							value,
							set_dyn_field_list,
							show_add_field_line,
							show_add_btn,
							show_minus_btn,
							tooltip_signals,
							config: config_enter.clone(),
						});
					}
					EventPropagation::Continue
				}),
			icon_button(String::from(save_icon), create_rw_signal(true), move |_| {
				save_new_field(SaveNewField {
					id,
					value,
					set_dyn_field_list,
					show_add_field_line,
					show_add_btn,
					show_minus_btn,
					tooltip_signals,
					config: config_btn.clone(),
				});
			})
			.on_event(EventListener::PointerEnter, move |_event| {
				tooltip_signals.show(String::from("Save to database"));
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				tooltip_signals.hide();
				EventPropagation::Continue
			}),
		))
		.style(move |s| {
			s.align_items(AlignItems::Center)
				.width_full()
				.gap(4.0, 0.0)
				.display(Display::None)
				.apply_if(show_add_field_line.get(), |s| s.display(Display::Flex))
		}),
		icon_button(String::from(add_icon), show_add_btn, move |_| {
			main_scroll_to.set(100.0);
			tooltip_signals.hide();
			show_add_field_line.set(true);
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
			show_add_field_line.set(false);
			show_add_btn.set(true);
			show_minus_btn.set(false);
			value.set(String::from(""));
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
