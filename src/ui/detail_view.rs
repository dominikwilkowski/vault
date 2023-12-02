use floem::{
	action::exec_after,
	event::EventListener,
	reactive::{create_rw_signal, RwSignal},
	style::{AlignContent, AlignItems},
	view::View,
	views::{container, h_stack, label, svg, v_stack, Decorators},
	Clipboard, EventPropagation,
};

use std::time::Duration;

use crate::db::{get_db_by_field, get_db_by_id, DbFields};
use crate::ui::primitives::{button::icon_button, input_field::input_field};

const PASSWORD_PLACEHOLDER: &str = "****************";

fn list_item(
	id: usize,
	field: DbFields,
	value: RwSignal<String>,
	is_secret: bool,
	tooltip_text: RwSignal<String>,
	tooltip_visible: RwSignal<bool>,
	tooltip_pos: RwSignal<(f64, f64)>,
	mouse_pos: RwSignal<(f64, f64)>,
	window_size: RwSignal<(f64, f64)>,
) -> impl View {
	let input = input_field(value, |s| s.width(250));
	let input_id = input.id();
	let see_btn_visible = create_rw_signal(true);
	let hide_btn_visible = create_rw_signal(false);

	let clipboard_icon = include_str!("./icons/clipboard.svg");
	let edit_icon = include_str!("./icons/edit.svg");
	let see_icon = include_str!("./icons/see.svg");
	let hide_icon = include_str!("./icons/hide.svg");

	let view_button_slot = if is_secret {
		h_stack((
			icon_button(String::from(see_icon), see_btn_visible, move |_| {
				let data = get_db_by_field(&id, &field);
				value.set(data);
				see_btn_visible.set(false);
				hide_btn_visible.set(true);
			}),
			icon_button(String::from(hide_icon), hide_btn_visible, move |_| {
				value.set(String::from(PASSWORD_PLACEHOLDER));
				see_btn_visible.set(true);
				hide_btn_visible.set(false);
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
		input,
		container(icon_button(
			String::from(clipboard_icon),
			create_rw_signal(true),
			move |_| {
				let data = get_db_by_field(&id, &field);
				let _ = Clipboard::set_contents(data);
			},
		))
		.on_event(EventListener::PointerEnter, move |_event| {
			tooltip_text.set(String::from("Copy to clipboard"));
			exec_after(Duration::from_secs_f64(0.6), move |_| {
				if tooltip_text.get() == "Copy to clipboard" {
					let pos = mouse_pos.get();
					let y = if window_size.get().1 > pos.1 + 33.0 {
						pos.1 + 13.0
					} else {
						pos.1 - 23.0
					};
					tooltip_pos.set((pos.0 + 13.0, y));
					tooltip_text.set(String::from("Copy to clipboard"));
					tooltip_visible.set(true);
				}
			});
			EventPropagation::Continue
		})
		.on_event(EventListener::PointerLeave, move |_| {
			tooltip_text.set(String::from(""));
			tooltip_visible.set(false);
			EventPropagation::Continue
		}),
		icon_button(String::from(edit_icon), create_rw_signal(true), |_| {}),
		view_button_slot,
	))
	.style(|s| s.align_items(AlignItems::Center).width_full().gap(4.0, 0.0))
}

pub fn detail_view(
	id: usize,
	tooltip_text: RwSignal<String>,
	tooltip_visible: RwSignal<bool>,
	tooltip_pos: RwSignal<(f64, f64)>,
	mouse_pos: RwSignal<(f64, f64)>,
	window_size: RwSignal<(f64, f64)>,
) -> impl View {
	let data = get_db_by_id(id);
	let title = create_rw_signal(String::from(data.1));
	let url = create_rw_signal(String::from(data.2));
	let username = create_rw_signal(String::from(PASSWORD_PLACEHOLDER));
	let password = create_rw_signal(String::from(PASSWORD_PLACEHOLDER));
	let notes = create_rw_signal(String::from(data.3));

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
				tooltip_text,
				tooltip_visible,
				tooltip_pos,
				mouse_pos,
				window_size,
			),
			list_item(
				id,
				DbFields::Url,
				url,
				false,
				tooltip_text,
				tooltip_visible,
				tooltip_pos,
				mouse_pos,
				window_size,
			),
			list_item(
				id,
				DbFields::Username,
				username,
				true,
				tooltip_text,
				tooltip_visible,
				tooltip_pos,
				mouse_pos,
				window_size,
			),
			list_item(
				id,
				DbFields::Password,
				password,
				true,
				tooltip_text,
				tooltip_visible,
				tooltip_pos,
				mouse_pos,
				window_size,
			),
			list_item(
				id,
				DbFields::Notes,
				notes,
				false,
				tooltip_text,
				tooltip_visible,
				tooltip_pos,
				mouse_pos,
				window_size,
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
