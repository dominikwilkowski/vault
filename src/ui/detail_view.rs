use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_rw_signal, RwSignal, WriteSignal},
	style::{AlignContent, AlignItems, CursorStyle, Display, Position},
	view::View,
	views::{container, h_stack, label, svg, v_stack, Decorators},
	Clipboard, EventPropagation,
};
// use zeroize::Zeroize;

use crate::config::SharedConfig;
use crate::db::DbFields;
use crate::ui::colors::*;
use crate::ui::primitives::{
	button::icon_button, input_field::input_field, tooltip::TooltipSignals,
};

const PASSWORD_PLACEHOLDER: &str = "••••••••••••••••";

fn list_item(
	id: usize,
	field: DbFields,
	value: RwSignal<String>,
	is_secret: bool,
	tooltip_signals: TooltipSignals,
	set_list: WriteSignal<im::Vector<(usize, &'static str, usize)>>,
	config: SharedConfig,
) -> impl View {
	let see_btn_visible = create_rw_signal(true);
	let hide_btn_visible = create_rw_signal(false);
	let edit_btn_visible = create_rw_signal(true);
	let save_btn_visible = create_rw_signal(false);
	let reset_text = create_rw_signal(String::from(""));

	let clipboard_icon = include_str!("./icons/clipboard.svg");
	let edit_icon = include_str!("./icons/edit.svg");
	let revert_icon = include_str!("./icons/revert.svg");
	let save_icon = include_str!("./icons/save.svg");
	let see_icon = include_str!("./icons/see.svg");
	let hide_icon = include_str!("./icons/hide.svg");
	let history_icon = include_str!("./icons/history.svg");

	let config_edit = config.clone();

	let input = input_field(value, move |s| {
		s.width(250)
			.padding_right(30)
			.display(Display::None)
			.apply_if(save_btn_visible.get(), |s| s.display(Display::Flex))
	});
	let input_id = input.id();

	let input_line = h_stack((
		input.on_event(EventListener::KeyDown, move |event| {
			let key = match event {
				Event::KeyDown(k) => k.key.physical_key,
				_ => PhysicalKey::Code(KeyCode::F35),
			};

			if key == PhysicalKey::Code(KeyCode::Escape) {
				value.set(reset_text.get());
				edit_btn_visible.set(true);
				save_btn_visible.set(false);
			}
			EventPropagation::Continue
		}),
		container(
			svg(move || String::from(revert_icon)).style(|s| s.width(16).height(16)),
		)
		.on_click(move |_| {
			value.set(reset_text.get());
			edit_btn_visible.set(true);
			save_btn_visible.set(false);
			tooltip_signals.hide();
			EventPropagation::Continue
		})
		.on_event(EventListener::PointerEnter, move |_event| {
			tooltip_signals.show("Revert field");
			EventPropagation::Continue
		})
		.on_event(EventListener::PointerLeave, move |_| {
			tooltip_signals.hide();
			EventPropagation::Continue
		})
		.style(|s| {
			s.position(Position::Absolute)
				.z_index(5)
				.display(Display::Flex)
				.items_center()
				.justify_center()
				.inset_top(0)
				.inset_right(0)
				.inset_bottom(0)
				.width(30)
				.cursor(CursorStyle::Pointer)
		}),
	));
	let config_clipboard = config.clone();

	let view_button_slot = if is_secret {
		h_stack((
			icon_button(String::from(see_icon), see_btn_visible, move |_| {
				let data =
					config.config.read().unwrap().db.get_db_by_field(&id, &field);
				value.set(data);
				see_btn_visible.set(false);
				hide_btn_visible.set(true);
				tooltip_signals.hide();
			})
			.on_event(EventListener::PointerEnter, move |_event| {
				if is_secret {
					tooltip_signals.show("See contents of field");
				}
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				if is_secret {
					tooltip_signals.hide();
				}
				EventPropagation::Continue
			}),
			icon_button(String::from(hide_icon), hide_btn_visible, move |_| {
				value.set(String::from(PASSWORD_PLACEHOLDER));
				see_btn_visible.set(true);
				hide_btn_visible.set(false);
				tooltip_signals.hide();
			})
			.on_event(EventListener::PointerEnter, move |_event| {
				if is_secret {
					tooltip_signals.show("Hide contents of field");
				}
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				if is_secret {
					tooltip_signals.hide();
				}
				EventPropagation::Continue
			}),
			container(icon_button(
				String::from(history_icon),
				create_rw_signal(true),
				move |_| {
					tooltip_signals.hide();
				},
			))
			.style(|s| s.margin_left(4))
			.on_event(EventListener::PointerEnter, move |_event| {
				if is_secret {
					tooltip_signals.show("See history of field");
				}
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				if is_secret {
					tooltip_signals.hide();
				}
				EventPropagation::Continue
			}),
		))
	} else {
		h_stack((label(|| ""), label(|| "")))
	};

	h_stack((
		container(label(move || format!("{}", &field)))
			.style(|s| s.width(70).justify_content(AlignContent::End))
			.on_click_stop(move |_| {
				input_id.request_focus();
			}),
		h_stack((
			input_line,
			label(move || value.get()).style(move |s| {
				s.width(250)
					.padding_top(5)
					.padding_right(6)
					.padding_left(6)
					.padding_bottom(5)
					.border_bottom(1)
					.border_color(C_TEXT_TOP)
					.display(Display::Flex)
					.apply_if(save_btn_visible.get(), |s| s.display(Display::None))
			}),
		)),
		h_stack((
			icon_button(String::from(edit_icon), edit_btn_visible, move |_| {
				reset_text.set(value.get());
				edit_btn_visible.set(false);
				save_btn_visible.set(true);
				tooltip_signals.hide();
				if is_secret {
					value.set(String::from(""));
				}
				input_id.request_focus();
			}),
			icon_button(String::from(save_icon), save_btn_visible, move |_| {
				config_edit.clone().config.write().unwrap().db.edit_field(
					id,
					&field,
					value.get(),
				);
				let new_list = config_edit.config.read().unwrap().db.get_list();
				set_list.update(
					|list: &mut im::Vector<(usize, &'static str, usize)>| {
						*list = new_list;
					},
				);

				if is_secret {
					// TODO: use Zeroize somehow?
				}
				edit_btn_visible.set(true);
				save_btn_visible.set(false);
				tooltip_signals.hide();
				input_id.request_focus();
			}),
		))
		.on_event(EventListener::PointerEnter, move |_event| {
			let text = if edit_btn_visible.get() {
				"Edit this field"
			} else {
				"Save to database"
			};
			tooltip_signals.show(text);
			EventPropagation::Continue
		})
		.on_event(EventListener::PointerLeave, move |_| {
			tooltip_signals.hide();
			EventPropagation::Continue
		}),
		container(icon_button(
			String::from(clipboard_icon),
			create_rw_signal(true),
			move |_| {
				let data = config_clipboard
					.config
					.read()
					.unwrap()
					.db
					.get_db_by_field(&id, &field);
				let _ = Clipboard::set_contents(data);
			},
		))
		.on_event(EventListener::PointerEnter, move |_event| {
			tooltip_signals.show("Copy to clipboard");
			EventPropagation::Continue
		})
		.on_event(EventListener::PointerLeave, move |_| {
			tooltip_signals.hide();
			EventPropagation::Continue
		}),
		view_button_slot,
	))
	.style(|s| s.align_items(AlignItems::Center).width_full().gap(4.0, 0.0))
}

pub fn detail_view(
	id: usize,
	tooltip_signals: TooltipSignals,
	set_list: WriteSignal<im::Vector<(usize, &'static str, usize)>>,
	config: SharedConfig,
) -> impl View {
	let data = config.config.read().unwrap().db.get_by_id(&id);
	let title = create_rw_signal(data.title);
	let url = create_rw_signal(data.url);
	let username = create_rw_signal(String::from(PASSWORD_PLACEHOLDER));
	let password = create_rw_signal(String::from(PASSWORD_PLACEHOLDER));
	let notes = create_rw_signal(data.notes);

	let password_icon = include_str!("./icons/password.svg");

	v_stack((
		h_stack((
			svg(move || String::from(password_icon))
				.style(|s| s.width(24).height(24)),
			label(move || String::from("Details")).style(|s| s.font_size(24.0)),
		))
		.style(|s| {
			s.align_items(AlignItems::Center)
				.gap(5, 0)
				.margin_top(15)
				.margin_bottom(20)
		}),
		v_stack((
			list_item(
				id,
				DbFields::Title,
				title,
				false,
				tooltip_signals,
				set_list,
				config.clone(),
			),
			list_item(
				id,
				DbFields::Url,
				url,
				false,
				tooltip_signals,
				set_list,
				config.clone(),
			),
			list_item(
				id,
				DbFields::Username,
				username,
				true,
				tooltip_signals,
				set_list,
				config.clone(),
			),
			list_item(
				id,
				DbFields::Password,
				password,
				true,
				tooltip_signals,
				set_list,
				config.clone(),
			),
			list_item(
				id,
				DbFields::Notes,
				notes,
				false,
				tooltip_signals,
				set_list,
				config.clone(),
			),
		))
		.style(|s| s.gap(0, 5)),
	))
	.style(|s| {
		s.padding(8.0)
			.width_full()
			.justify_content(AlignContent::Center)
			.align_items(AlignItems::Center)
	})
}
