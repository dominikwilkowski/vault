use std::time::Instant;
use url_escape;
use webbrowser;

use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal, WriteSignal},
	style::{AlignItems, CursorStyle, Display, Foreground, Position},
	view::View,
	views::{container, empty, h_stack, label, Decorators},
	widgets::slider::{slider, AccentBarClass, BarClass, HandleRadius},
	EventPropagation,
};

use crate::{
	db::{DbFields, DynFieldKind},
	env::Environment,
	password_gen::generate_password,
	ui::{
		colors::*,
		details::{
			button_slots::{
				clipboard_button_slot, delete_button_slot, edit_button_slot,
				history_button_slot, view_button_slot, DeleteButtonSlot,
				EditButtonSlot, HistoryButtonSlot, ViewButtonSlot,
			},
			detail_view::{
				save_edit, SaveEdit, INPUT_LINE_WIDTH, LINE_WIDTH, SECRET_PLACEHOLDER, MULTILINE_HEIGHT,
			},
			dyn_field_title_form::{dyn_field_title_form, DynFieldTitleForm},
		},
		primitives::{
			styles,
			button::{icon_button, IconButton},
			input_button_field::{input_button_field, InputButtonField},
			input_field::input_field,
			multiline_input_field::multiline_input_field,
			tooltip::TooltipSignals,
		},
	},
};

pub struct ListItem {
	pub id: usize,
	pub field: DbFields,
	pub set_hidden_field_list: WriteSignal<im::Vector<DbFields>>,
	pub set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	pub hidden_field_len: RwSignal<usize>,
	pub is_hidden: bool,
	pub tooltip_signals: TooltipSignals,
	pub set_list: WriteSignal<im::Vector<(usize, &'static str, usize)>>,
	pub env: Environment,
}

pub fn list_item(param: ListItem) -> impl View {
	let ListItem {
		id,
		field,
		set_hidden_field_list,
		set_dyn_field_list,
		hidden_field_len,
		is_hidden,
		tooltip_signals,
		set_list,
		env,
	} = param;

	let edit_button_switch = create_rw_signal(false);
	let view_button_switch = create_rw_signal(false);
	let reset_text = create_rw_signal(String::from(""));
	let dates = create_rw_signal(env.db.get_history_dates(&id, &field));

	let secret_generator_progress = create_rw_signal(0.0);
	let show_generator_progress = create_rw_signal(false);
	let generator_entropy_value = create_rw_signal(String::from(""));
	let generator_entropy_timing = create_rw_signal(Vec::new());
	let generator_entropy_mouse = create_rw_signal(Vec::new());

	let field_title = match field {
		DbFields::Fields(_) => env.db.get_name_of_field(&id, &field),
		other => format!("{}", other),
	};
	let title_value = create_rw_signal(field_title.clone());
	let dyn_field_kind = env.db.get_field_kind(&id, &field);
	let is_secret = match dyn_field_kind {
		DynFieldKind::TextLine | DynFieldKind::MultiLine | DynFieldKind::Url => {
			false
		},
		DynFieldKind::TextLineSecret | DynFieldKind::MultiLineSecret => true,
	};

	let is_multiline = match dyn_field_kind {
			DynFieldKind::MultiLine | DynFieldKind::MultiLineSecret => true,
			_ => false,
		};

	let field_value = if is_secret {
		create_rw_signal(if is_multiline {
			format!("{SECRET_PLACEHOLDER}\n{SECRET_PLACEHOLDER}\n{SECRET_PLACEHOLDER}")
		} else {
			String::from(SECRET_PLACEHOLDER)
		})
	} else {
		create_rw_signal(env.db.get_last_by_field(&id, &field))
	};

	let is_dyn_field = matches!(field, DbFields::Fields(_));
	let is_url_field = matches!(dyn_field_kind, DynFieldKind::Url);

	let revert_icon = include_str!("../icons/revert.svg");
	let generate_icon = include_str!("../icons/generate.svg");
	let no_generate_icon = include_str!("../icons/no_generate.svg");

	let env_edit = env.clone();
	let env_submit = env.clone();
	let env_title = env.clone();
	let env_view_button = env.clone();
	let env_history = env.clone();
	let env_delete_button = env.clone();

	let multiline_input = multiline_input_field(field_value.get());
	let mut input_id = multiline_input.id();

	let input = if is_multiline {
		container(multiline_input).style(styles::multiline).any()
	} else {
		let view = input_button_field(
			InputButtonField {
				value: field_value,
				icon: create_rw_signal(String::from(revert_icon)),
				placeholder: "",
				tooltip: String::from("Reset field"),
				tooltip_signals,
			},
			move || {
				field_value.set(reset_text.get());
				edit_button_switch.set(false);
				tooltip_signals.hide();
			},
		);
		if !is_multiline {
			input_id = view.input_id;
		}
		view.any()
	};

	let title_input = input_field(title_value);
	let generator_input = input_field(generator_entropy_value);
	let generator_input_id = generator_input.id();

	let generate_slot = if is_secret {
		let start_time = Instant::now();

		h_stack((
			icon_button(
				IconButton {
					icon: String::from(generate_icon),
					icon2: Some(String::from(no_generate_icon)),
					tooltip: String::from("Generate a secret"),
					tooltip2: Some(String::from("Hide generator")),
					switch: Some(show_generator_progress),
					tooltip_signals,
					..IconButton::default()
				},
				move |_| {
					generator_input_id.request_focus();
				},
			),
			generator_input
				.on_event_cont(EventListener::FocusLost, move |_| {
					if show_generator_progress.get() {
						generator_input_id.request_focus();
					}
				})
				.on_event_cont(EventListener::KeyDown, move |_| {
					let current_time = Instant::now();
					generator_entropy_timing.update(|collection| {
						collection.push(
							current_time.duration_since(start_time).as_millis().to_string(),
						)
					});
					generator_entropy_mouse.update(|collection| {
						collection.push(format!(
							"{}{}",
							tooltip_signals.mouse_pos.get().0,
							tooltip_signals.mouse_pos.get().1
						))
					});

					let generator_keystrokes = generator_entropy_value.get().len() as f32;
					let pct = generator_keystrokes
						/ (env.config.general.read().pass_gen_letter_count / 100.0);
					if pct > 100.0 {
						let seed = format!(
							"{}{}{}",
							generator_entropy_value.get(),
							generator_entropy_timing.get().join(""),
							generator_entropy_mouse.get().join(""),
						);

						let pass = generate_password(seed);
						field_value.set(pass);
						generator_entropy_value.set(String::from(""));
						secret_generator_progress.set(0.0);
						show_generator_progress.set(false);
						input_id.request_focus();
					} else {
						secret_generator_progress.set(pct);
					}
				})
				.style(|s| {
					s.position(Position::Absolute).width(0).height(0).border(0).padding(0)
				}),
			container(label(|| "Start typing to generate password"))
				.on_click_stop(move |_| {
					generator_input_id.request_focus();
				})
				.style(move |s| {
					s.position(Position::Absolute)
						.inset_left(INPUT_LINE_WIDTH * -1.0 + 25.0)
						.inset_top(2)
						.width(INPUT_LINE_WIDTH - 25.0 - 3.0)
						.border_radius(2)
						.height(24 + 3)
						.background(C_MAIN_BG_INACTIVE.with_alpha_factor(0.9))
						.items_center()
						.justify_center()
						.display(Display::None)
						.apply_if(show_generator_progress.get(), |s| {
							s.display(Display::Flex)
						})
				}),
			slider(move || secret_generator_progress.get()).style(move |s| {
				s.position(Position::Absolute)
					.inset_bottom(-4)
					.inset_left(INPUT_LINE_WIDTH * -1.0 + 25.0 + 2.0)
					.width(INPUT_LINE_WIDTH - 1.0)
					.padding(0)
					.height(5)
					.class(AccentBarClass, |s| s.background(C_FOCUS))
					.class(BarClass, |s| {
						s.height(5)
							.background(C_FOCUS.with_alpha_factor(0.1))
							.border_radius(0)
					})
					.set(Foreground, C_FOCUS)
					.set(HandleRadius, 0)
					.display(Display::None)
					.apply_if(show_generator_progress.get(), |s| s.display(Display::Flex))
			}),
		))
		.style(|s| s.position(Position::Relative))
		.any()
	} else {
		empty().any()
	};

	let input_line = h_stack((
		input
			.on_event_cont(EventListener::FocusGained, move |_| {
				if show_generator_progress.get() {
					generator_input_id.request_focus();
				}
			})
			.style(move |s| s.flex_grow(1.0).apply_if(is_multiline, |s| s.height(MULTILINE_HEIGHT)))
			.on_event(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if key == PhysicalKey::Code(KeyCode::Escape) {
					field_value.set(reset_text.get());
					edit_button_switch.set(false);
				}

				if key == PhysicalKey::Code(KeyCode::Enter) {
					edit_button_switch.set(false);
					env_submit.db.edit_field_title(&id, &field, title_value.get());
					save_edit(SaveEdit {
						id,
						field,
						value: field_value,
						dates,
						is_secret,
						input_id,
						set_list,
						env: env_submit.clone(),
					});
				}
				EventPropagation::Continue
			}),
		generate_slot,
	))
	.style(move |s| {
		s.flex()
			.items_center()
			.justify_center()
			.gap(4, 0)
			.width(INPUT_LINE_WIDTH)
			.display(Display::None)
			.apply_if(edit_button_switch.get(), |s| s.display(Display::Flex))
	});

	h_stack((
		dyn_field_title_form(
			DynFieldTitleForm {
				title_value,
				title_editable: edit_button_switch,
				field_value,
				reset_text,
				is_dyn_field,
				title_input,
				tooltip_signals,
			},
			move || {
				edit_button_switch.set(false);
				if is_dyn_field {
					env_title.db.edit_field_title(&id, &field, title_value.get());
					let _ = env_title.db.save();
				}
				save_edit(SaveEdit {
					id,
					field,
					value: field_value,
					dates,
					is_secret,
					input_id,
					set_list,
					env: env_title.clone(),
				})
			},
		),
		h_stack((
			input_line,
			label(move || field_value.get())
				.style(move |s| {
					s.width(INPUT_LINE_WIDTH)
						.padding_top(5)
						.padding_right(6)
						.padding_left(6)
						.padding_bottom(5)
						.border_bottom(1)
						.border_color(C_TOP_TEXT)
						.apply_if(is_hidden, |s| s.border_color(C_MAIN_TEXT_INACTIVE))
						.display(Display::Flex)
						.apply_if(edit_button_switch.get(), |s| s.display(Display::None))
						.apply_if(is_multiline, |s| s.height(MULTILINE_HEIGHT))
						.hover(|s| {
							s.apply_if(is_url_field, |s| {
								s.color(C_FOCUS).cursor(CursorStyle::Pointer)
							})
						})
				})
				.on_click(move |_| {
					if is_url_field {
						let _ = webbrowser::open(&url_escape::encode_fragment(
							&field_value.get(),
						));
					}
					EventPropagation::Continue
				}),
		)),
		edit_button_slot(EditButtonSlot {
			id,
			field,
			switch: edit_button_switch,
			is_hidden,
			is_secret,
			input_id,
			dates,
			field_value,
			reset_text,
			set_list,
			view_button_switch,
			tooltip_signals,
			env: env_edit,
		}),
		clipboard_button_slot(tooltip_signals, move || {
			env.db.get_last_by_field(&id, &field)
		}),
		view_button_slot(
			ViewButtonSlot {
				switch: view_button_switch,
				is_shown: is_secret,
				tooltip_signals,
				field_value,
			},
			move || {
				field_value.set(reset_text.get());
				edit_button_switch.set(false);
				env_view_button.db.get_last_by_field(&id, &field)
			},
		),
		history_button_slot(HistoryButtonSlot {
			id,
			field,
			dates,
			is_shown: !matches!(field, DbFields::Title),
			field_title,
			tooltip_signals,
			db: env_history.db,
		}),
		delete_button_slot(DeleteButtonSlot {
			id,
			field,
			set_hidden_field_list,
			set_dyn_field_list,
			hidden_field_len,
			is_dyn_field,
			is_hidden,
			tooltip_signals,
			env: env_delete_button,
		}),
	))
	.style(move |s| {
		s.align_items(AlignItems::Center)
			.width_full()
			.gap(4.0, 0.0)
			.width(LINE_WIDTH)
			.apply_if(is_hidden, |s| s.color(C_MAIN_TEXT_INACTIVE))
	})
}
