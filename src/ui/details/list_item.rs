use std::time::Instant;
use url_escape;
use webbrowser;
use zeroize::Zeroize;

use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, use_context, RwSignal},
	style::{AlignItems, CursorStyle, Display, Position},
	views::{
		container,
		editor::core::{editor::EditType, selection::Selection},
		empty, label, scroll,
		slider::slider,
		Decorators,
	},
	IntoView, View,
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
				save_edit, SaveEdit, INPUT_LINE_WIDTH, LINE_WIDTH, MULTILINE_HEIGHT,
				SECRET_MULTILINE_PLACEHOLDER, SECRET_PLACEHOLDER,
			},
			dyn_field_title_form::{dyn_field_title_form, DynFieldTitleForm},
		},
		keyboard::is_submit,
		primitives::{
			button::{icon_button, IconButton},
			input_button_field::{input_button_field, InputButtonField},
			input_field::input_field,
			multiline_input_field::multiline_input_field,
			styles,
			tooltip::TooltipSignals,
		},
	},
};

const BORDER_WIDTH: f64 = 1.0;
const GUTTER_WIDTH: f64 = 4.0;
const BUTTON_WIDTH: f64 = 25.0;

pub fn replace_consecutive_newlines(input: String) -> String {
	let mut output = input.clone();
	if input.contains("\n\n") {
		output = replace_consecutive_newlines(input.replace("\n\n", "\n \n"));
	}
	output
}

pub struct ListItem {
	pub id: usize,
	pub field: DbFields,
	pub hidden_field_list: RwSignal<im::Vector<DbFields>>,
	pub field_list: RwSignal<im::Vector<DbFields>>,
	pub hidden_field_len: RwSignal<usize>,
	pub is_hidden: bool,
}

pub fn list_item(param: ListItem) -> impl IntoView {
	let ListItem {
		id,
		field,
		hidden_field_list,
		field_list,
		hidden_field_len,
		is_hidden,
	} = param;

	let env = use_context::<Environment>().expect("No env context provider");
	let tooltip_signals = use_context::<TooltipSignals>()
		.expect("No tooltip_signals context provider");

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

	let is_multiline = matches!(
		dyn_field_kind,
		DynFieldKind::MultiLine | DynFieldKind::MultiLineSecret
	);

	let field_value = if is_secret {
		create_rw_signal(if is_multiline {
			String::from(SECRET_MULTILINE_PLACEHOLDER)
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

	let env_submit = env.clone();
	let env_title = env.clone();
	let env_view_button = env.clone();
	let env_history = env.clone();

	let multiline_input = multiline_input_field(field_value.get());
	let field_doc = create_rw_signal(multiline_input.doc());
	let mut input_id = multiline_input.id();

	let title_input = input_field(title_value);
	let generator_input = input_field(generator_entropy_value);
	let generator_input_id = generator_input.id();

	let input = if is_multiline {
		(
			container(multiline_input).style(styles::multiline),
			icon_button(
				IconButton {
					icon: String::from(revert_icon),
					tooltip: String::from("Reset field"),
					tooltip_signals,
					..IconButton::default()
				},
				move |_| {
					field_doc.get().edit_single(
						Selection::region(0, field_doc.get().text().len()),
						&reset_text.get(),
						EditType::DeleteSelection,
					);
					field_value.update(|field| field.zeroize());
					field_value.set(reset_text.get());
					edit_button_switch.set(false);
					tooltip_signals.hide();
				},
			)
			.style(move |s| {
				s.position(Position::Absolute)
					.inset_top(0)
					.inset_right(-27 - 5 - 5)
					.apply_if(is_secret, |s| s.inset_right(-27 - 5))
			}),
		)
			.style(|s| s.position(Position::Relative))
			.into_any()
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
				field_value.update(|field| field.zeroize());
				field_value.set(reset_text.get());
				edit_button_switch.set(false);
				tooltip_signals.hide();
			},
		);

		if !is_multiline {
			input_id = view.input_id;
		}

		view
			.on_event_cont(EventListener::FocusGained, move |_| {
				if show_generator_progress.get() {
					generator_input_id.request_focus();
				}
			})
			.on_event_cont(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if key == PhysicalKey::Code(KeyCode::Escape) {
					field_value.set(reset_text.get());
					edit_button_switch.set(false);
				}

				if is_submit(key) {
					edit_button_switch.set(false);
					env_submit.db.edit_field_title(&id, &field, title_value.get());
					save_edit(SaveEdit {
						id,
						field,
						value: field_value,
						doc: field_doc.get(),
						dates,
						is_secret,
						is_multiline,
						input_id,
					});
				}
			})
			.into_any()
	};

	let generate_slot = if is_secret {
		let start_time = Instant::now();

		(
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
				.on_event_cont(EventListener::KeyDown, move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};

					if key == PhysicalKey::Code(KeyCode::Escape) {
						show_generator_progress.set(false);
					}

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
					let pct = generator_keystrokes / 0.1; // 10 key strokes
					if pct > 100.0 {
						let entropy = format!(
							"{}{}{}",
							generator_entropy_value.get(),
							generator_entropy_timing.get().join(""),
							generator_entropy_mouse.get().join(""),
						);

						let mut pass = generate_password(entropy);
						field_value.set(pass.clone());
						field_doc.get().edit_single(
							Selection::region(0, field_doc.get().text().len()),
							&format!("{}\n{}\n", field_doc.get().text(), pass),
							EditType::DeleteSelection,
						);
						pass.zeroize();
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
			container("Start typing to generate password")
				.on_click_stop(move |_| {
					generator_input_id.request_focus();
				})
				.style(move |s| {
					s.position(Position::Absolute)
						.inset_left(INPUT_LINE_WIDTH * -1.0 + BUTTON_WIDTH)
						.inset_top(0)
						.width(INPUT_LINE_WIDTH - BUTTON_WIDTH)
						.border_radius(2)
						.height(24 + 3)
						.background(C_MAIN_BG_INACTIVE.with_alpha_factor(0.9))
						.items_center()
						.justify_center()
						.apply_if(is_multiline, |s| {
							s.height(MULTILINE_HEIGHT).inset_top(
								((MULTILINE_HEIGHT / 2.0) * -1.0) + BUTTON_WIDTH / 2.0,
							)
						})
						.display(Display::None)
						.apply_if(show_generator_progress.get(), |s| {
							s.display(Display::Flex)
						})
				}),
			slider(move || secret_generator_progress.get())
				.slider_style(|s| {
					s.accent_bar_color(C_FOCUS)
						.bar_height(5)
						.bar_color(C_FOCUS.with_alpha_factor(0.1))
						.handle_radius(0)
				})
				.style(move |s| {
					s.position(Position::Absolute)
						.inset_bottom(-5)
						.inset_left(INPUT_LINE_WIDTH * -1.0 + BUTTON_WIDTH + GUTTER_WIDTH)
						.width(
							INPUT_LINE_WIDTH
								- BORDER_WIDTH - BUTTON_WIDTH
								- GUTTER_WIDTH - GUTTER_WIDTH,
						)
						.padding(0)
						.height(5)
						.display(Display::None)
						.apply_if(show_generator_progress.get(), |s| {
							s.display(Display::Flex)
						})
				}),
		)
			.style(|s| s.position(Position::Relative))
			.into_any()
	} else {
		empty().into_any()
	};

	let input_line = (
		input.style(move |s| {
			s.flex_grow(1.0).apply_if(is_multiline, |s| s.height(MULTILINE_HEIGHT))
		}),
		generate_slot,
	)
		.style(move |s| {
			s.flex()
				.items_center()
				.justify_center()
				.row_gap(4)
				.width(INPUT_LINE_WIDTH)
				.display(Display::None)
				.apply_if(edit_button_switch.get(), |s| s.display(Display::Flex))
		});

	(
		dyn_field_title_form(
			DynFieldTitleForm {
				title_value,
				title_editable: edit_button_switch,
				field_value,
				doc: field_doc.get(),
				reset_text,
				is_dyn_field,
				title_input,
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
					doc: field_doc.get(),
					dates,
					is_secret,
					is_multiline,
					input_id,
				})
			},
		),
		(
			input_line,
			scroll(
				label(move || replace_consecutive_newlines(field_value.get())).style(
					|s| s.padding_bottom(3).font_family(String::from("Monospace")),
				),
			)
			.style(move |s| {
				s.flex_grow(1.0)
					.width_full()
					.margin_left(5)
					.margin_right(5)
					.border_bottom(BORDER_WIDTH)
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
			.on_click_cont(move |_| {
				if is_url_field {
					let _ =
						webbrowser::open(&url_escape::encode_fragment(&field_value.get()));
				}
			}),
		)
			.style(|s| s.width(INPUT_LINE_WIDTH)),
		edit_button_slot(EditButtonSlot {
			id,
			field,
			switch: edit_button_switch,
			is_hidden,
			is_secret,
			is_multiline,
			input_id,
			dates,
			field_value,
			multiline_field_value: field_doc,
			reset_text,
			view_button_switch,
		}),
		clipboard_button_slot(move || env.db.get_last_by_field(&id, &field)),
		view_button_slot(
			ViewButtonSlot {
				switch: view_button_switch,
				is_shown: is_secret,
				is_multiline,
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
			db: env_history.db,
		}),
		delete_button_slot(DeleteButtonSlot {
			id,
			field,
			hidden_field_list,
			field_list,
			hidden_field_len,
			is_dyn_field,
			is_hidden,
		}),
	)
		.style(move |s| {
			s.align_items(AlignItems::Center)
				.width_full()
				.row_gap(GUTTER_WIDTH)
				.width(LINE_WIDTH)
				.apply_if(is_hidden, |s| s.color(C_MAIN_TEXT_INACTIVE))
		})
}
