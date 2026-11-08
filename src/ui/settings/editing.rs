use floem::{
	event::listener::{self},
	peniko::Brush,
	reactive::{Effect, RwSignal, SignalGet, SignalUpdate},
	style::{CursorStyle, Display},
	ui_events::keyboard::Code,
	views::{
		slider::{slider, SliderChanged},
		virtual_stack, Container, Decorators, Empty, Label,
	},
	IntoView, View,
};

use crate::{
	config::PresetFields,
	db::DynFieldKind,
	env::Environment,
	ui::focus::{focus_later, has_focus},
	ui::shared_context,
	ui::{
		app_view::{PresetFieldSignal, TooltipSignalsSettings},
		colors::*,
		details::button_slots::empty_button_slot,
		keyboard::is_submit,
		primitives::{
			button::{icon_button, IconButton},
			input_field::input_field,
			select::select,
			styles,
			tooltip::TooltipSignals,
		},
	},
};

const MIN: f32 = 8.0;
const MAX: f32 = 42.0;

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
) -> impl IntoView {
	let title_value = RwSignal::new(title.clone());
	let kind_value = RwSignal::new(kind.clone());
	let kind_id = DynFieldKind::all_values()
		.into_iter()
		.enumerate()
		.find(|(_, this_kind)| *this_kind == kind)
		.unwrap_or((0, DynFieldKind::default()))
		.0;
	let kind_signal = RwSignal::new(kind_id);

	let delete_icon = include_str!("../icons/delete.svg");
	let save_icon = include_str!("../icons/save.svg");

	let env_enter_save = env.clone();
	let env_button_save = env.clone();

	let delete_slot = if id == 0 {
		empty_button_slot().into_any()
	} else {
		Container::new(icon_button(
			IconButton {
				icon: String::from(delete_icon),
				tooltip: String::from("Delete preset"),
				tooltip_signals,
				..IconButton::default()
			},
			move || {
				delete_preset(id, field_presets, env.clone());
				tooltip_signals.hide();
			},
		))
		.into_any()
	};

	(
		input_field(title_value).style(|s| s.width(122)).on_event_cont(
			listener::KeyDown,
			move |cx, event| {
				if !has_focus(cx) {
					return;
				}

				let key = event.code;

				if is_submit(key) {
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
		(
			delete_slot.style(|s| s.margin_right(5)),
			Container::new(
				icon_button(
					IconButton {
						icon: String::from(save_icon),
						tooltip: String::from("Save to database"),
						tooltip_signals,
						..IconButton::default()
					},
					move || {
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
		),
	)
		.style(|s| s.col_gap(5).items_center())
}

fn convert_pct_2_letter_count(pct: f32) -> usize {
	(((MAX / 100.0) * pct) + MIN).round() as usize
}

fn convert_letter_count_2_pct(timeout: f32) -> f32 {
	((timeout - MIN) / MAX) * 100.0
}

pub fn editing_view() -> impl IntoView {
	let tooltip_signals = shared_context::get::<TooltipSignalsSettings>()
		.expect("No tooltip_signals context provider")
		.inner;
	let env =
		shared_context::get::<Environment>().expect("No env context provider");

	let field_presets = shared_context::get::<PresetFieldSignal>()
		.expect("No field_presets context provider");

	let show_form = RwSignal::new(false);
	let title_value = RwSignal::new(String::from(""));
	let kind_value = RwSignal::new(DynFieldKind::default());
	let kind_signal = RwSignal::new(0);

	let db_passgen_letter_count_pct = convert_letter_count_2_pct(
		env.config.general.read().pass_gen_letter_count as f32,
	);
	let passgen_letter_count_pct = RwSignal::new(db_passgen_letter_count_pct);
	let passgen_letter_count_pct_backup =
		RwSignal::new(db_passgen_letter_count_pct);

	let add_icon = include_str!("../icons/add.svg");
	let minus_icon = include_str!("../icons/minus.svg");
	let save_icon = include_str!("../icons/save.svg");
	let revert_icon = include_str!("../icons/revert.svg");

	let env_passgen = env.clone();
	let env_enter_save = env.clone();
	let env_button_save = env.clone();

	let title_input = input_field(title_value).style(|s| s.width(122));
	let title_input_id = title_input.id();

	let preset_list_data: imbl::Vector<(usize, String, String, DynFieldKind)> =
		field_presets.get().into();
	let preset_list = RwSignal::new(preset_list_data);

	Effect::new(move |_| {
		preset_list.update(
			|list: &mut imbl::Vector<(usize, String, String, DynFieldKind)>| {
				let preset_list_data: imbl::Vector<(
					usize,
					String,
					String,
					DynFieldKind,
				)> = field_presets.get().into();
				*list = preset_list_data;
			},
		);
	});

	Container::new(
		(
			"Password generator",
			(
				Label::derived(move || {
					format!(
						"{} characters",
						convert_pct_2_letter_count(passgen_letter_count_pct.get())
					)
				}),
				(
					slider(move || passgen_letter_count_pct.get())
						.slider_style(|s| {
							s.handle_color(Brush::Solid(C_FOCUS))
								.accent_bar_color(C_FOCUS.multiply_alpha(0.5))
								.bar_height(5)
								.bar_color(C_FOCUS.multiply_alpha(0.2))
								.handle_radius(6)
						})
						.style(|s| s.width(241).cursor(CursorStyle::Pointer))
						.on_event_cont(
							SliderChanged::listener(),
							move |_, slider_event| {
								let pct = slider_event.pct.0 as f32;
								passgen_letter_count_pct.set(pct);
							},
						),
					Container::new(
						(
							icon_button(
								IconButton {
									icon: String::from(revert_icon),
									tooltip: String::from("Reset"),
									tooltip_signals,
									..IconButton::default()
								},
								move || {
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
								move || {
									env_passgen.config.general.write().pass_gen_letter_count =
										convert_pct_2_letter_count(passgen_letter_count_pct.get());
									let _ = env_passgen.config.save();

									passgen_letter_count_pct_backup
										.set(passgen_letter_count_pct.get());
									tooltip_signals.hide();
								},
							),
						)
							.style(move |s| {
								s.col_gap(5).display(Display::Flex).apply_if(
									convert_pct_2_letter_count(passgen_letter_count_pct.get())
										== convert_pct_2_letter_count(
											passgen_letter_count_pct_backup.get(),
										),
									|s| s.display(Display::None),
								)
							}),
					)
					.style(|s| s.height(25)),
				)
					.style(|s| s.items_center().col_gap(5)),
			)
				.style(|s| s.flex_col()),
			"Preset fields",
			virtual_stack(
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
			.item_size_fixed(|| 33.0)
			.style(|s| s.margin_top(20).col_gap(0).row_gap(0).grid()),
			Empty::new(),
			(
				(
					title_input.on_event_cont(listener::KeyDown, move |cx, event| {
						if !has_focus(cx) {
							return;
						}

						let key = event.code;

						if key == Code::Escape {
							show_form.set(false);
						}

						if is_submit(key) {
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
						move || {
							save_new_preset(
								title_value,
								kind_value,
								kind_signal,
								field_presets,
								env_button_save.clone(),
							);
						},
					),
				)
					.style(move |s| {
						s.col_gap(5)
							.row_gap(5)
							.margin_top(-5)
							.margin_bottom(5)
							.items_center()
							.display(Display::None)
							.apply_if(show_form.get(), |s| s.display(Display::Flex))
					}),
				Container::new(icon_button(
					IconButton {
						icon: String::from(add_icon),
						icon2: Some(String::from(minus_icon)),
						tooltip: String::from("Add a new field"),
						tooltip2: Some(String::from("Hide the new field form")),
						switch: Some(show_form),
						tooltip_signals,
						..IconButton::default()
					},
					move || {
						if show_form.get() {
							focus_later(title_input_id);
						}
					},
				))
				.style(|s| s.margin_top(5)),
			)
				.style(|s| s.flex_col()),
		)
			.style(|s| s.flex_col().margin_bottom(60).min_width(440))
			.style(styles::settings_line),
	)
}
