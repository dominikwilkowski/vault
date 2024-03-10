use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{
		create_effect, create_rw_signal, create_signal, use_context, RwSignal,
	},
	style::Display,
	view::View,
	views::{
		container, empty, h_stack, label, v_stack, virtual_stack, Decorators,
		VirtualDirection, VirtualItemSize,
	},
	EventPropagation,
};

use crate::{
	config::PresetFields,
	db::DynFieldKind,
	env::Environment,
	ui::{
		app_view::PresetFieldSignal,
		primitives::{
			button::{icon_button, IconButton},
			input_field::input_field,
			select::select,
			styles,
			tooltip::TooltipSignals,
		},
	},
};

fn save_new_preset(
	title: RwSignal<String>,
	kind: RwSignal<DynFieldKind>,
	kind_signal: RwSignal<usize>,
	field_presets: RwSignal<PresetFields>,
	env: Environment,
) {
	if !title.get().is_empty() {
		let presets = env.config.add_field_preset(title.get(), kind.get());
		let _ = env.config.save();
		field_presets.set(presets);
		title.set(String::from(""));
		kind.set(DynFieldKind::default());
		kind_signal.set(0);
	}
}

fn save_edit_preset(
	id: usize,
	title: String,
	kind: DynFieldKind,
	field_presets: RwSignal<PresetFields>,
	env: Environment,
) {
	if !title.is_empty() {
		let presets = env.config.edit_field_preset(id, title, kind);
		let _ = env.config.save();
		field_presets.set(presets.clone());
	}
}

fn delete_preset(
	id: usize,
	field_presets: RwSignal<PresetFields>,
	env: Environment,
) {
	let presets = env.config.delete_field_preset(id);
	let _ = env.config.save();
	field_presets.set(presets);
}

fn preset_line(
	id: usize,
	title: String,
	kind: DynFieldKind,
	tooltip_signals: TooltipSignals,
	field_presets: RwSignal<PresetFields>,
	env: Environment,
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

	let env_enter_save = env.clone();
	let env_button_save = env.clone();

	let delete_slot = if id == 0 {
		empty().any()
	} else {
		container(icon_button(
			IconButton {
				icon: String::from(delete_icon),
				tooltip: String::from("Delete preset"),
				tooltip_signals,
				..IconButton::default()
			},
			move |_| {
				delete_preset(id, field_presets, env.clone());
				tooltip_signals.hide();
			},
		))
		.any()
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
					field_presets,
					env_enter_save.clone(),
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
							field_presets,
							env_button_save.clone(),
						);
					},
				)
				.style(move |s| {
					s.display(Display::None).apply_if(
						title_value.get() != title || kind_value.get() != kind,
						|s| s.display(Display::Flex),
					)
				}),
			)
			.style(|s| s.width(30)),
		)),
	))
	.style(|s| s.gap(5, 0).items_center().padding_bottom(5))
}

pub fn editing_view() -> impl View {
	let tooltip_signals: TooltipSignals =
		use_context().expect("No tooltip_signals context provider");
	let env: Environment = use_context().expect("No env context provider");

	let field_presets: PresetFieldSignal =
		use_context().expect("No field_presets context provider");

	let show_form = create_rw_signal(false);
	let title_value = create_rw_signal(String::from(""));
	let kind_value = create_rw_signal(DynFieldKind::default());
	let kind_signal = create_rw_signal(0);

	let add_icon = include_str!("../icons/add.svg");
	let minus_icon = include_str!("../icons/minus.svg");
	let save_icon = include_str!("../icons/save.svg");

	let env_enter_save = env.clone();
	let env_button_save = env.clone();

	let title_input = input_field(title_value);
	let title_input_id = title_input.id();

	let preset_list_data: im::Vector<(usize, String, String, DynFieldKind)> =
		field_presets.get().into();
	let (preset_list, set_preset_list) = create_signal(preset_list_data);

	create_effect(move |_| {
		set_preset_list.update(
			|list: &mut im::Vector<(usize, String, String, DynFieldKind)>| {
				let preset_list_data: im::Vector<(
					usize,
					String,
					String,
					DynFieldKind,
				)> = field_presets.get().into();
				*list = preset_list_data;
			},
		);
	});

	container(
		v_stack((
			label(|| "Preset fields"),
			virtual_stack(
				VirtualDirection::Vertical,
				VirtualItemSize::Fixed(Box::new(|| 32.0)),
				move || preset_list.get(),
				move |(id, title, val, kind)| {
					(*id, title.clone(), val.clone(), kind.clone())
				},
				move |(id, title, _, kind)| {
					preset_line(
						id,
						title,
						kind,
						tooltip_signals,
						field_presets,
						env.clone(),
					)
				},
			)
			.style(|s| s.gap(5, 5)),
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
								title_value,
								kind_value,
								kind_signal,
								field_presets,
								env_enter_save.clone(),
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
								title_value,
								kind_value,
								kind_signal,
								field_presets,
								env_button_save.clone(),
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
				))
				.style(|s| s.margin_top(10)),
			)),
		))
		.style(|s| s.margin_bottom(60).min_width(440))
		.style(styles::settings_line),
	)
}
