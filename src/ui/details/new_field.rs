use std::rc::Rc;

use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, use_context, RwSignal},
	style::{AlignItems, Display},
	views::{
		container, dyn_container, editor::text::Document, text_editor, Decorators,
	},
	IntoView, View,
};

use crate::{
	config::PresetFields,
	db::{DbFields, DynFieldKind},
	env::Environment,
	ui::{
		keyboard::is_submit,
		primitives::{
			button::{icon_button, IconButton},
			input_field::input_field,
			multiline_input_field::multiline_input_field,
			select::select,
			styles,
			tooltip::TooltipSignals,
		},
	},
};

const ROW_GAP: i32 = 4;

struct SaveNewField {
	pub id: usize,
	pub kind: RwSignal<DynFieldKind>,
	pub preset_value: RwSignal<usize>,
	pub title_value: RwSignal<String>,
	pub field_value: RwSignal<String>,
	pub multiline_field_value: RwSignal<Rc<dyn Document>>,
	pub field_list: RwSignal<im::Vector<DbFields>>,
}

fn save_new_field(params: SaveNewField) {
	let SaveNewField {
		id,
		kind,
		preset_value,
		title_value,
		field_value,
		multiline_field_value,
		field_list,
	} = params;

	let env = use_context::<Environment>().expect("No env context provider");
	let tooltip_signals = use_context::<TooltipSignals>()
		.expect("No tooltip_signals context provider");

	let value = match kind.get() {
		DynFieldKind::Url
		| DynFieldKind::Heading
		| DynFieldKind::TextLine
		| DynFieldKind::TextLineSecret => field_value.get(),
		DynFieldKind::MultiLine | DynFieldKind::MultiLineSecret => {
			String::from(multiline_field_value.get().text())
		},
	};

	if !title_value.get().is_empty() && !value.is_empty()
		|| !title_value.get().is_empty()
			&& matches!(kind.get(), DynFieldKind::Heading)
	{
		env.db.add_field(&id, kind.get(), title_value.get(), value);
		let _ = env.db.save();
		let field_list_db = env.db.get_visible_fields(&id);
		field_list.set(field_list_db.into());
		tooltip_signals.hide();
		preset_value.set(0);
		title_value.set(String::from(""));
		field_value.set(String::from(""));
	}
}

pub fn new_field(
	id: usize,
	field_presets: RwSignal<PresetFields>,
	field_list: RwSignal<im::Vector<DbFields>>,
	main_scroll_to: RwSignal<f32>,
) -> impl IntoView {
	let tooltip_signals = use_context::<TooltipSignals>()
		.expect("No tooltip_signals context provider");

	let show_minus_button = create_rw_signal(false);
	let preset_value = create_rw_signal(0);
	let title_value = create_rw_signal(String::from(""));
	let field_value = create_rw_signal(String::from(""));
	let kind = create_rw_signal(DynFieldKind::default());
	let kind_signal = create_rw_signal(0);
	let multiline_doc = create_rw_signal(text_editor("").doc());

	let add_icon = include_str!("../icons/add.svg");
	let minus_icon = include_str!("../icons/minus.svg");
	let save_icon = include_str!("../icons/save.svg");

	let title_input = input_field(title_value);
	let title_input_id = title_input.id();

	(
		(
			dyn_container(
				move || field_presets.get(),
				move |field_presets| {
					if !field_presets
						.clone()
						.into_iter()
						.any(|(id, _, _, _)| id == preset_value.get())
					{
						preset_value.set(0);
					}

					select(
						preset_value,
						field_presets
							.iter()
							.map(|(id, title, _, _)| (*id, title.clone()))
							.collect(),
						move |id| {
							let selected =
								field_presets.clone().into_iter().nth(id).unwrap_or((
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
						},
					)
					.into_any()
				},
			),
			title_input
				.placeholder("Title of field")
				.on_event_cont(EventListener::KeyDown, move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};

					if key == PhysicalKey::Code(KeyCode::Escape) {
						field_value.set(String::from(""));
						show_minus_button.set(false);
					}

					if is_submit(key) {
						let selected_kind = DynFieldKind::all_values()
							.into_iter()
							.nth(kind_signal.get())
							.unwrap_or_default();
						save_new_field(SaveNewField {
							id,
							kind: create_rw_signal(selected_kind),
							preset_value,
							title_value,
							field_value,
							multiline_field_value: multiline_doc,
							field_list,
						});
						title_input_id.request_focus();
					}
				})
				.style(move |s| {
					s.width(100).apply_if(
						matches!(
							DynFieldKind::all_values()
								.into_iter()
								.nth(kind_signal.get())
								.unwrap_or_default(),
							DynFieldKind::Heading
						),
						|s| s.width(100 + ROW_GAP + 177),
					)
				}),
			dyn_container(
				move || kind_signal.get(),
				move |kind_signal| {
					let selected_kind = DynFieldKind::all_values()
						.into_iter()
						.nth(kind_signal)
						.unwrap_or_default();

					match selected_kind {
						DynFieldKind::Url
						| DynFieldKind::Heading
						| DynFieldKind::TextLine
						| DynFieldKind::TextLineSecret => input_field(field_value)
							.placeholder("Value of field")
							.style(move |s| s.width(177))
							.on_event_cont(EventListener::KeyDown, move |event| {
								let key = match event {
									Event::KeyDown(k) => k.key.physical_key,
									_ => PhysicalKey::Code(KeyCode::F35),
								};

								if key == PhysicalKey::Code(KeyCode::Escape) {
									field_value.set(String::from(""));
									show_minus_button.set(false);
								}

								if is_submit(key) {
									let selected_kind = DynFieldKind::all_values()
										.into_iter()
										.nth(kind_signal)
										.unwrap_or_default();
									save_new_field(SaveNewField {
										id,
										kind: create_rw_signal(selected_kind),
										preset_value,
										title_value,
										field_value,
										multiline_field_value: multiline_doc,
										field_list,
									});
									title_input_id.request_focus();
								}
							})
							.into_any(),
						DynFieldKind::MultiLine | DynFieldKind::MultiLineSecret => {
							let multiline_input = multiline_input_field(String::from(""))
								.placeholder("Value of field");
							multiline_doc.set(multiline_input.doc());
							container(multiline_input)
								.style(styles::multiline)
								.style(|s| s.width(177).height(150))
								.into_any()
						},
					}
				},
			)
			.style(move |s| {
				s.width(177).apply_if(
					matches!(
						DynFieldKind::all_values()
							.into_iter()
							.nth(kind_signal.get())
							.unwrap_or_default(),
						DynFieldKind::Heading
					),
					|s| s.display(Display::None),
				)
			}),
			select(
				kind_signal,
				DynFieldKind::all_values().into_iter().enumerate().collect(),
				move |_| {
					// println!(
					// 	"changing kind: {:?} and {:?}",
					// 	field_value.get(),
					// 	multiline_doc.get().text()
					// );
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
					let selected_kind = DynFieldKind::all_values()
						.into_iter()
						.nth(kind_signal.get())
						.unwrap_or_default();
					save_new_field(SaveNewField {
						id,
						kind: create_rw_signal(selected_kind),
						preset_value,
						title_value,
						field_value,
						multiline_field_value: multiline_doc,
						field_list,
					});
				},
			),
		)
			.style(move |s| {
				s.row_gap(ROW_GAP)
					.items_start()
					.justify_center()
					.display(Display::None)
					.apply_if(show_minus_button.get(), |s| {
						s.display(Display::Flex).margin_bottom(10)
					})
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
	)
		.style(move |s| {
			s.flex_col()
				.align_items(AlignItems::Center)
				.width_full()
				.row_gap(ROW_GAP)
				.apply_if(show_minus_button.get(), |s| s.margin_bottom(80))
		})
}
