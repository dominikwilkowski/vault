use floem::{
	event::EventListener,
	reactive::{Context, RwSignal, SignalGet, SignalUpdate},
	style::{AlignContent, AlignItems, Display},
	views::{Container, Decorators, Empty, Label},
	IntoView,
};

use crate::{
	db::DbFields,
	env::Environment,
	ui::{
		details::{
			button_slots::{
				delete_button_slot, drag_button_slot, empty_button_slot,
				DeleteButtonSlot,
			},
			detail_view::{INPUT_LINE_WIDTH, LABEL_WIDTH},
			list_item::GUTTER_WIDTH,
		},
		primitives::{
			button::{icon_button, IconButton},
			input_button_field::{input_button_field_with_enter, InputButtonField},
			tooltip::TooltipSignals,
		},
	},
};

pub fn heading_view(
	id: usize,
	field: DbFields,
	title_value: RwSignal<String>,
	hidden_field_list: RwSignal<imbl::Vector<DbFields>>,
	field_list: RwSignal<imbl::Vector<DbFields>>,
	hidden_field_len: RwSignal<usize>,
	is_hidden: bool,
) -> impl IntoView {
	let env = Context::get::<Environment>().expect("No env context provider");
	let tooltip_signals = Context::get::<TooltipSignals>()
		.expect("No tooltip_signals context provider");

	let env_keyboard_event = env.clone();

	let edit_button_switch = RwSignal::new(false);
	let reset_text = RwSignal::new(String::from(""));

	let edit_icon = include_str!("../icons/edit.svg");
	let save_icon = include_str!("../icons/save.svg");
	let revert_icon = include_str!("../icons/revert.svg");

	let heading_input = input_button_field(
		InputButtonField {
			value: title_value,
			icon: RwSignal::new(String::from(revert_icon)),
			placeholder: "",
			tooltip: String::from("Reset field"),
			tooltip_signals,
		},
		move || {
			title_value.set(reset_text.get());
			edit_button_switch.set(false);
			tooltip_signals.hide();
		},
	);
	let input_id = heading_input.input_id;

	let heading_edit_button_slot = if is_hidden {
		empty_button_slot().into_any()
	} else {
		Container::new(icon_button(
			IconButton {
				icon: String::from(edit_icon),
				icon2: Some(String::from(save_icon)),
				tooltip: String::from("Edit this field"),
				tooltip2: Some(String::from("Save to database")),
				switch: Some(edit_button_switch),
				tooltip_signals,
				..IconButton::default()
			},
			move |_| {
				if edit_button_switch.get() {
					reset_text.set(title_value.get());
					input_id.request_focus();
				} else if title_value.get() != reset_text.get() {
					env.db.edit_field_title(&id, &field, title_value.get());
					let _ = env.db.save();
				}
			},
		))
		.into_any()
	};

	(
		Empty::new().style(|s| s.width(LABEL_WIDTH)),
		heading_input
			.on_event_cont(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::Key(k) if k.state == KeyState::Down => k.code,
					_ => Code::F35,
				};

				if is_submit(key) && title_value.get() != reset_text.get() {
					env_keyboard_event.db.edit_field_title(
						&id,
						&field,
						title_value.get(),
					);
					let _ = env_keyboard_event.db.save();
					edit_button_switch.set(false);
					tooltip_signals.hide();
				}

				if key == Code::Escape {
					title_value.set(reset_text.get());
					edit_button_switch.set(false);
					tooltip_signals.hide();
				}
			})
			.style(move |s| {
				s.width(INPUT_LINE_WIDTH)
					.apply_if(!edit_button_switch.get(), |s| s.display(Display::None))
			}),
		Container::new(
			Label::derived(move || title_value.get())
				.style(|s| s.align_self(AlignItems::Center).font_size(16.0)),
		)
		.style(move |s| {
			s.justify_content(AlignContent::Center)
				.width(INPUT_LINE_WIDTH)
				.apply_if(edit_button_switch.get(), |s| s.display(Display::None))
		}),
		heading_edit_button_slot,
		empty_button_slot(),
		empty_button_slot(),
		empty_button_slot(),
		delete_button_slot(DeleteButtonSlot {
			id,
			field,
			hidden_field_list,
			field_list,
			hidden_field_len,
			is_dyn_field: matches!(field, DbFields::Fields(_)),
			is_hidden,
		}),
		drag_button_slot(),
	)
		.style(move |s| s.padding_top(50).padding_bottom(5).gap(GUTTER_WIDTH))
}
