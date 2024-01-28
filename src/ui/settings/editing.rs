use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal},
	style::Display,
	view::View,
	views::{
		container, h_stack, label, v_stack, v_stack_from_iter, Container,
		Decorators,
	},
	EventPropagation,
};

use crate::{
	config::{Config, PresetFields},
	db::DynFieldKind,
	ui::primitives::{
		button::{icon_button, IconButton},
		input_field::input_field,
		select::select,
		styles,
		tooltip::TooltipSignals,
	},
};

fn save_edit_preset(
	id: usize,
	title: String,
	kind: DynFieldKind,
	preset_fields: RwSignal<PresetFields>,
	config: Config,
) {
	if !title.is_empty() {
		println!("edit: {:?} {:?} {:?}", id, title, kind);
		// TODO: implement edit for preset fields
		preset_fields.set(config.get_field_presets());
	}
}

fn prefix_line(
	id: usize,
	title: String,
	kind: DynFieldKind,
	tooltip_signals: TooltipSignals,
	preset_fields: RwSignal<PresetFields>,
	config: Config,
) -> impl View {
	let title_value = create_rw_signal(title.clone());
	let kind_value = create_rw_signal(kind.clone());
	let kind_id = DynFieldKind::all_values()
		.into_iter()
		.enumerate()
		.find(|(_, this_kind)| *this_kind == kind)
		.unwrap_or((0, DynFieldKind::default()))
		.0;
	let kind_signal = create_rw_signal(kind_id);

	let delete_icon = include_str!("../icons/delete.svg");
	let save_icon = include_str!("../icons/save.svg");

	let config_enter_save = config.clone();
	let config_button_save = config.clone();

	let delete_slot = if id == 0 {
		container(label(|| ""))
	} else {
		container(icon_button(
			IconButton {
				icon: String::from(delete_icon),
				tooltip: String::from("Delete preset"),
				tooltip_signals,
				..IconButton::default()
			},
			move |_| {
				println!("delete: {:?}", id);
			},
		))
		.style(|s| s.margin_right(5))
	};

	h_stack((
		input_field(title_value).on_event(EventListener::KeyDown, move |event| {
			let key = match event {
				Event::KeyDown(k) => k.key.physical_key,
				_ => PhysicalKey::Code(KeyCode::F35),
			};

			if key == PhysicalKey::Code(KeyCode::Enter) {
				save_edit_preset(
					id,
					title_value.get(),
					kind_value.get(),
					preset_fields,
					config_enter_save.clone(),
				);
			}

			EventPropagation::Continue
		}),
		select(
			kind_signal,
			DynFieldKind::all_values().into_iter().enumerate().collect(),
			move |id| {
				let selected =
					DynFieldKind::all_values().into_iter().nth(id).unwrap_or_default();
				kind_value.set(selected);
			},
		),
		h_stack((
			delete_slot,
			container(
				icon_button(
					IconButton {
						icon: String::from(save_icon),
						tooltip: String::from("Save to database"),
						tooltip_signals,
						..IconButton::default()
					},
					move |_| {
						save_edit_preset(
							id,
							title_value.get(),
							kind_value.get(),
							preset_fields,
							config_button_save.clone(),
						);
					},
				)
				.style(move |s| {
					s.display(Display::None)
						.apply_if(title_value.get() != title, |s| s.display(Display::Flex))
				}),
			)
			.style(|s| s.width(30)),
		)),
	))
	.style(|s| s.gap(5, 0).items_center())
}

fn save_new_preset(
	title: String,
	kind: DynFieldKind,
	preset_fields: RwSignal<PresetFields>,
	config: Config,
) {
	if !title.is_empty() {
		println!("add: {:?} {:?}", title, kind);
		// TODO: implement save for preset fields
		preset_fields.set(config.get_field_presets());
	}
}

pub fn editing_view(
	tooltip_signals: TooltipSignals,
	config: Config,
) -> Container {
	let preset_fields = create_rw_signal(config.get_field_presets());
	let show_form = create_rw_signal(false);
	let title_value = create_rw_signal(String::from(""));
	let kind_value = create_rw_signal(DynFieldKind::default());
	let kind_signal = create_rw_signal(0);

	let add_icon = include_str!("../icons/add.svg");
	let minus_icon = include_str!("../icons/minus.svg");
	let save_icon = include_str!("../icons/save.svg");

	let config_enter_save = config.clone();
	let config_button_save = config.clone();

	let title_input = input_field(title_value);
	let title_input_id = title_input.id();

	container(
		v_stack((
			label(|| "Preset fields"),
			v_stack_from_iter(preset_fields.get().into_iter().map(
				|(id, title, _, kind)| {
					prefix_line(
						id,
						title,
						kind,
						tooltip_signals,
						preset_fields,
						config.clone(),
					)
				},
			))
			.style(|s| s.gap(0, 5)),
			label(|| ""),
			v_stack((
				h_stack((
					title_input.on_event(EventListener::KeyDown, move |event| {
						let key = match event {
							Event::KeyDown(k) => k.key.physical_key,
							_ => PhysicalKey::Code(KeyCode::F35),
						};

						if key == PhysicalKey::Code(KeyCode::Escape) {
							show_form.set(false);
						}

						if key == PhysicalKey::Code(KeyCode::Enter) {
							save_new_preset(
								title_value.get(),
								kind_value.get(),
								preset_fields,
								config_enter_save.clone(),
							);
						}

						EventPropagation::Continue
					}),
					select(
						kind_signal,
						DynFieldKind::all_values().into_iter().enumerate().collect(),
						move |id| {
							let selected = DynFieldKind::all_values()
								.into_iter()
								.nth(id)
								.unwrap_or_default();
							kind_value.set(selected);
						},
					),
					icon_button(
						IconButton {
							icon: String::from(save_icon),
							tooltip: String::from("Save new preset to database"),
							tooltip_signals,
							..IconButton::default()
						},
						move |_| {
							save_new_preset(
								title_value.get(),
								kind_value.get(),
								preset_fields,
								config_button_save.clone(),
							);
						},
					),
				))
				.style(move |s| {
					s.gap(5, 5)
						.items_center()
						.margin_top(15)
						.display(Display::None)
						.apply_if(show_form.get(), |s| s.display(Display::Flex))
				}),
				container(icon_button(
					IconButton {
						icon: String::from(add_icon),
						icon2: Some(String::from(minus_icon)),
						tooltip: String::from("Add a new field"),
						tooltip2: Some(String::from("Hide the new field form")),
						switch: Some(show_form),
						tooltip_signals,
						..IconButton::default()
					},
					move |_| {
						if show_form.get() {
							title_input_id.request_focus();
						}
					},
				)),
			)),
		))
		.style(styles::settings_line),
	)
}
