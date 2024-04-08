use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_effect, create_rw_signal, use_context, RwSignal},
	style::{CursorStyle, Display},
	view::View,
	views::{
		container, empty, h_stack, label, v_stack, virtual_stack, Decorators,
		VirtualDirection, VirtualItemSize,
	},
	widgets::slider::slider,
};

use crate::{
	config::PresetFields,
	db::DynFieldKind,
	env::Environment,
	ui::{
		app_view::{PresetFieldSignal, TooltipSignalsSettings},
		colors::*,
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
		input_field(title_value).on_event_cont(
			EventListener::KeyDown,
			move |event| {
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
			},
		),
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

const MIN: f32 = 8.0;
const MAX: f32 = 42.0;

fn convert_pct_2_letter_count(pct: f32) -> usize {
	(((MAX / 100.0) * pct) + MIN).round() as usize
}

fn convert_letter_count_2_pct(timeout: f32) -> f32 {
	((timeout - MIN) / MAX) * 100.0
}

pub fn editing_view() -> impl View {
	let tooltip_signals = use_context::<TooltipSignalsSettings>()
		.expect("No tooltip_signals context provider")
		.inner;
	let env = use_context::<Environment>().expect("No env context provider");

	let field_presets = use_context::<PresetFieldSignal>()
		.expect("No field_presets context provider");

	let show_form = create_rw_signal(false);
	let title_value = create_rw_signal(String::from(""));
	let kind_value = create_rw_signal(DynFieldKind::default());
	let kind_signal = create_rw_signal(0);

	let db_passgen_letter_count_pct = convert_letter_count_2_pct(
		env.config.general.read().pass_gen_letter_count as f32,
	);
	let passgen_letter_count_pct = create_rw_signal(db_passgen_letter_count_pct);
	let passgen_letter_count_pct_backup =
		create_rw_signal(db_passgen_letter_count_pct);

	let add_icon = include_str!("../icons/add.svg");
	let minus_icon = include_str!("../icons/minus.svg");
	let save_icon = include_str!("../icons/save.svg");
	let revert_icon = include_str!("../icons/revert.svg");

	let env_passgen = env.clone();
	let env_enter_save = env.clone();
	let env_button_save = env.clone();

	let title_input = input_field(title_value);
	let title_input_id = title_input.id();

	let preset_list_data: im::Vector<(usize, String, String, DynFieldKind)> =
		field_presets.get().into();
	let preset_list = create_rw_signal(preset_list_data);

	create_effect(move |_| {
		preset_list.update(
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
			label(move || "Password generator"),
			v_stack((
				label(move || {
					format!(
						"{} characters",
						convert_pct_2_letter_count(passgen_letter_count_pct.get())
					)
				}),
				h_stack((
					slider(move || passgen_letter_count_pct.get())
						.slider_style(|s| {
							s.handle_color(C_FOCUS)
								.accent_bar_color(C_FOCUS.with_alpha_factor(0.5))
								.bar_height(5)
								.bar_color(C_FOCUS.with_alpha_factor(0.2))
								.handle_radius(6)
						})
						.style(|s| s.width(241).hover(|s| s.cursor(CursorStyle::Pointer)))
						.on_change_pct(move |pct| {
							passgen_letter_count_pct.set(pct);
						}),
					container(
						h_stack((
							icon_button(
								IconButton {
									icon: String::from(revert_icon),
									tooltip: String::from("Reset"),
									tooltip_signals,
									..IconButton::default()
								},
								move |_| {
									passgen_letter_count_pct
										.set(passgen_letter_count_pct_backup.get());
									tooltip_signals.hide();
								},
							),
							icon_button(
								IconButton {
									icon: String::from(save_icon),
									tooltip: String::from("Save to database"),
									tooltip_signals,
									..IconButton::default()
								},
								move |_| {
									env_passgen.config.general.write().pass_gen_letter_count =
										convert_pct_2_letter_count(passgen_letter_count_pct.get());
									let _ = env_passgen.config.save();

									passgen_letter_count_pct_backup
										.set(passgen_letter_count_pct.get());
									tooltip_signals.hide();
								},
							),
						))
						.style(move |s| {
							s.gap(5, 0).display(Display::Flex).apply_if(
								convert_pct_2_letter_count(passgen_letter_count_pct.get())
									== convert_pct_2_letter_count(
										passgen_letter_count_pct_backup.get(),
									),
								|s| s.display(Display::None),
							)
						}),
					)
					.style(|s| s.height(25)),
				))
				.style(|s| s.items_center().gap(5, 0)),
			)),
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
			.style(|s| s.margin_top(20).gap(5, 5)),
			label(|| ""),
			v_stack((
				h_stack((
					title_input.on_event_cont(EventListener::KeyDown, move |event| {
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
