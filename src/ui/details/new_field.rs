use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal, WriteSignal},
	style::{AlignItems, Display},
	view::View,
	views::{h_stack, v_stack, Decorators},
	EventPropagation,
};

use crate::{
	config::Config,
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
	set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	tooltip_signals: TooltipSignals,
	main_scroll_to: RwSignal<f32>,
	config: Config,
) -> impl View {
	let show_minus_btn = create_rw_signal(false);
	let preset_value = create_rw_signal(0);
	let title_value = create_rw_signal(String::from(""));
	let field_value = create_rw_signal(String::from(""));
	let kind = create_rw_signal(DynFieldKind::SecretLine); // TODO: hook up to dropdown

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
			select(
				preset_value,
				vec![
					(0, String::from("Custom")),
					(1, String::from("Username")),
					(2, String::from("Password")),
					(3, String::from("Password2")),
					(4, String::from("Password3 ong and long")),
					(5, String::from("Password4")),
				],
				|_| {},
			),
			title_input
				.placeholder("Title of field")
				.on_click_cont(move |_| {
					save_new_field(SaveNewField {
						id,
						kind,
						title_value,
						field_value,
						set_dyn_field_list,
						tooltip_signals,
						config: config_enter_title.clone(),
					});
					input_id.request_focus();
				})
				.style(|s| s.width(100)),
			input_field(field_value)
				.placeholder("Value of field")
				.style(move |s| s.width(150))
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
							kind,
							title_value,
							field_value,
							set_dyn_field_list,
							tooltip_signals,
							config: config_enter_field.clone(),
						});
						input_id.request_focus();
					}
					EventPropagation::Continue
				}),
			floem::views::label(|| "Dropdown"), // TODO: dropdown of the kinds of fields to be chosen: Text, Secret, Url
			icon_button(
				IconButton {
					icon: String::from(save_icon),
					tooltip: String::from("Save to database"),
					tooltip_signals,
					..IconButton::default()
				},
				move |_| {
					save_new_field(SaveNewField {
						id,
						kind,
						title_value,
						field_value,
						set_dyn_field_list,
						tooltip_signals,
						config: config_btn.clone(),
					});
				},
			),
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
				tooltip: String::from("Add a new field"),
				tooltip2: Some(String::from("Hide the new field form")),
				switch: Some(show_minus_btn),
				tooltip_signals,
				..IconButton::default()
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
	.style(move |s| {
		s.align_items(AlignItems::Center)
			.width_full()
			.gap(4.0, 0.0)
			.apply_if(show_minus_btn.get(), |s| s.margin_bottom(80))
	})
}
