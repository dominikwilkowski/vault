use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal, WriteSignal},
	style::{AlignItems, Display},
	view::View,
	views::{dyn_container, h_stack, v_stack, Decorators},
	EventPropagation,
};

use crate::{
	config::{Config, PresetFields},
	db::{DbFields, DynFieldKind},
	ui::primitives::{
		button::{icon_button, IconButton},
		input_field::input_field,
		select::select,
		tooltip::TooltipSignals,
	},
};

struct SaveNewField {
	pub id: usize,
	pub kind: RwSignal<DynFieldKind>,
	pub title_value: RwSignal<String>,
	pub field_value: RwSignal<String>,
	pub set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	pub tooltip_signals: TooltipSignals,
	pub config: Config,
}

fn save_new_field(params: SaveNewField) {
	let SaveNewField {
		id,
		kind,
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
			.add_dyn_field(&id, kind.get(), title_value.get(), field_value.get())
			.into();
		set_dyn_field_list.set(field_list);
		tooltip_signals.hide();
		title_value.set(String::from(""));
		field_value.set(String::from(""));
	}
}

pub fn new_field(
	id: usize,
	field_presets: RwSignal<PresetFields>,
	set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	tooltip_signals: TooltipSignals,
	main_scroll_to: RwSignal<f32>,
	config: Config,
) -> impl View {
	let show_minus_button = create_rw_signal(false);
	let preset_value = create_rw_signal(0);
	let title_value = create_rw_signal(String::from(""));
	let field_value = create_rw_signal(String::from(""));
	let kind = create_rw_signal(DynFieldKind::default());
	let kind_signal = create_rw_signal(0);

	let add_icon = include_str!("../icons/add.svg");
	let minus_icon = include_str!("../icons/minus.svg");
	let save_icon = include_str!("../icons/save.svg");

	let config_enter_title = config.clone();
	let config_enter_field = config.clone();
	let config_button = config.clone();

	let title_input = input_field(title_value);
	let title_input_id = title_input.id();

	let value_input = input_field(field_value);
	let value_input_id = value_input.id();

	v_stack((
		h_stack((
			dyn_container(
				move || field_presets.get(),
				move |field_presets_value| {
					if !field_presets_value
						.into_iter()
						.any(|(id, _, _, _)| id == preset_value.get())
					{
						preset_value.set(0);
					}

					Box::new(select(
						preset_value,
						field_presets
							.get()
							.iter()
							.map(|(id, title, _, _)| (*id, title.clone()))
							.collect(),
						move |id| {
							let selected =
								field_presets.get().into_iter().nth(id).unwrap_or((
									0,
									String::from("Custom"),
									String::from(""),
									DynFieldKind::default(),
								));
							title_value.set(selected.clone().2);
							let selected_kind = DynFieldKind::all_values()
								.into_iter()
								.enumerate()
								.find(|(_, kind)| *kind == selected.3)
								.unwrap_or((0, DynFieldKind::default()));
							kind_signal.set(selected_kind.0);
							kind.set(selected_kind.clone().1);

							if selected_kind.1 == DynFieldKind::Url
								&& field_value.get().is_empty()
							{
								field_value.set(String::from("https://"));
							} else if field_value.get() == "https://" {
								field_value.set(String::from(""));
							}

							if !selected.2.is_empty() {
								value_input_id.request_focus();
							} else {
								title_input_id.request_focus();
							}
						},
					))
				},
			),
			title_input
				.placeholder("Title of field")
				.on_event(EventListener::KeyDown, move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};

					if key == PhysicalKey::Code(KeyCode::Escape) {
						field_value.set(String::from(""));
						show_minus_button.set(false);
					}

					if key == PhysicalKey::Code(KeyCode::Enter) {
						let selected_kind = DynFieldKind::all_values()
							.into_iter()
							.nth(kind_signal.get())
							.unwrap_or_default();
						save_new_field(SaveNewField {
							id,
							kind: create_rw_signal(selected_kind),
							title_value,
							field_value,
							set_dyn_field_list,
							tooltip_signals,
							config: config_enter_title.clone(),
						});
						title_input_id.request_focus();
					}
					EventPropagation::Continue
				})
				.style(|s| s.width(100)),
			value_input
				.placeholder("Value of field")
				.style(move |s| s.width(150))
				.on_event(EventListener::KeyDown, move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};

					if key == PhysicalKey::Code(KeyCode::Escape) {
						field_value.set(String::from(""));
						show_minus_button.set(false);
					}

					if key == PhysicalKey::Code(KeyCode::Enter) {
						let selected_kind = DynFieldKind::all_values()
							.into_iter()
							.nth(kind_signal.get())
							.unwrap_or_default();
						save_new_field(SaveNewField {
							id,
							kind: create_rw_signal(selected_kind),
							title_value,
							field_value,
							set_dyn_field_list,
							tooltip_signals,
							config: config_enter_field.clone(),
						});
						title_input_id.request_focus();
					}
					EventPropagation::Continue
				}),
			select(
				kind_signal,
				DynFieldKind::all_values().into_iter().enumerate().collect(),
				|_| {},
			),
			icon_button(
				IconButton {
					icon: String::from(save_icon),
					tooltip: String::from("Save to database"),
					tooltip_signals,
					..IconButton::default()
				},
				move |_| {
					let selected_kind = DynFieldKind::all_values()
						.into_iter()
						.nth(kind_signal.get())
						.unwrap_or_default();
					save_new_field(SaveNewField {
						id,
						kind: create_rw_signal(selected_kind),
						title_value,
						field_value,
						set_dyn_field_list,
						tooltip_signals,
						config: config_button.clone(),
					});
				},
			),
		))
		.style(move |s| {
			s.gap(4.0, 0.0)
				.items_center()
				.justify_center()
				.display(Display::None)
				.apply_if(show_minus_button.get(), |s| s.display(Display::Flex))
		}),
		icon_button(
			IconButton {
				icon: String::from(add_icon),
				icon2: Some(String::from(minus_icon)),
				tooltip: String::from("Add a new field"),
				tooltip2: Some(String::from("Hide the new field form")),
				switch: Some(show_minus_button),
				tooltip_signals,
				..IconButton::default()
			},
			move |_| {
				if show_minus_button.get() {
					main_scroll_to.set(100.0);
					title_input_id.request_focus();
				} else {
					title_value.set(String::from(""));
				}
			},
		),
	))
	.style(move |s| {
		s.align_items(AlignItems::Center)
			.width_full()
			.gap(4.0, 0.0)
			.apply_if(show_minus_button.get(), |s| s.margin_bottom(80))
	})
}
