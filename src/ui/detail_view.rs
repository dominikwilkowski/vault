use floem::{
	reactive::{create_rw_signal, RwSignal},
	style::{AlignContent, AlignItems},
	view::View,
	views::{container, h_stack, label, svg, v_stack, Decorators},
};

use crate::db::db::get_db_by_id;
use crate::ui::primitives::input_field::input_field;

fn list_item(name: String, value: RwSignal<String>) -> impl View {
	let input = input_field(value, |s| s.width_full());
	let input_id = input.id();

	h_stack((
		container(label(move || name.clone()))
			.style(|s| s.width(50).justify_content(AlignContent::Start))
			.on_click_stop(move |_| {
				input_id.request_focus();
			}),
		input,
	))
	.style(|s| s.flex_row().align_items(AlignItems::Center).width_full())
}

pub fn detail_view(id: usize) -> impl View {
	let data = get_db_by_id(id);
	let title = create_rw_signal(String::from(data.1));
	let body = create_rw_signal(String::from(data.2));

	let password_icon = include_str!("./icons/password.svg");

	v_stack((
		h_stack((
			svg(move || String::from(password_icon))
				.style(|s| s.width(24).height(24)),
			label(move || String::from("Details")).style(|s| s.font_size(24.0)),
		))
		.style(|s| {
			s.width_full()
				.justify_content(AlignContent::Center)
				.align_items(AlignItems::Center)
				.gap(10, 0)
				.margin_bottom(20)
		}),
		v_stack((
			list_item(String::from("Title:"), title),
			list_item(String::from("Body:"), body),
		))
		.style(|s| s.gap(0, 5)),
	))
	.style(|s| s.padding(8.0).width_full())
}
