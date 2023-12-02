use floem::{
	event::EventListener,
	reactive::{create_rw_signal, RwSignal},
	style::{AlignContent, AlignItems},
	view::View,
	views::{container, h_stack, label, svg, v_stack, Decorators},
	Clipboard, EventPropagation,
};

use crate::db::{get_db_by_field, get_db_by_id, DbFields};
use crate::ui::primitives::{
	button::icon_button, input_field::input_field, tooltip::TooltipSignals,
};

const PASSWORD_PLACEHOLDER: &str = "****************";

fn list_item(
	id: usize,
	field: DbFields,
	value: RwSignal<String>,
	is_secret: bool,
	tooltip_signals: TooltipSignals,
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
				tooltip_signals.hide();
			}),
			icon_button(String::from(hide_icon), hide_btn_visible, move |_| {
				value.set(String::from(PASSWORD_PLACEHOLDER));
				see_btn_visible.set(true);
				hide_btn_visible.set(false);
				tooltip_signals.hide();
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
			tooltip_signals.show("Copy to clipboard");
			EventPropagation::Continue
		})
		.on_event(EventListener::PointerLeave, move |_| {
			tooltip_signals.hide();
			EventPropagation::Continue
		}),
		container(icon_button(
			String::from(edit_icon),
			create_rw_signal(true),
			|_| {},
		))
		.on_event(EventListener::PointerEnter, move |_event| {
			tooltip_signals.show("Edit this field");
			EventPropagation::Continue
		})
		.on_event(EventListener::PointerLeave, move |_| {
			tooltip_signals.hide();
			EventPropagation::Continue
		}),
		container(view_button_slot)
			.on_event(EventListener::PointerEnter, move |_event| {
				if is_secret {
					let text = if see_btn_visible.get() {
						"See contents of field"
					} else {
						"Hide contents of field"
					};
					tooltip_signals.show(text);
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
	.style(|s| s.align_items(AlignItems::Center).width_full().gap(4.0, 0.0))
}

pub fn detail_view(id: usize, tooltip_signals: TooltipSignals) -> impl View {
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
			list_item(id, DbFields::Title, title, false, tooltip_signals),
			list_item(id, DbFields::Url, url, false, tooltip_signals),
			list_item(id, DbFields::Username, username, true, tooltip_signals),
			list_item(id, DbFields::Password, password, true, tooltip_signals),
			list_item(id, DbFields::Notes, notes, false, tooltip_signals),
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
