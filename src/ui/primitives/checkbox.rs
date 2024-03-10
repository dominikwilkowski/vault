use floem::{
	style::CursorStyle,
	views::{
		container, create_value_container_signals, svg, value_container,
		Decorators, ValueContainer,
	},
};

use crate::ui::colors::*;

pub fn checkbox(checked: impl Fn() -> bool + 'static) -> ValueContainer<bool> {
	let (inbound_signal, outbound_signal) =
		create_value_container_signals(checked);

	let check_icon = include_str!("../icons/check.svg");

	value_container(
		container(
			svg(move || {
				if inbound_signal.read_only().get() {
					String::from(check_icon)
				} else {
					String::from("")
				}
			})
			.style(|s| s.width(10).height(10)),
		)
		.style(|s| {
			s.height(16)
				.width(16)
				.border_radius(3)
				.border(1)
				.border_color(C_TOP_TEXT)
				.border_radius(2)
				.background(C_MAIN_BG)
				.items_center()
				.justify_center()
				.hover(|s| {
					s.background(C_SIDE_BG_SELECTED.with_alpha_factor(0.6))
						.cursor(CursorStyle::Pointer)
				})
				.focus_visible(|s| s.outline(1).outline_color(C_FOCUS))
		})
		.keyboard_navigatable()
		.on_click_stop(move |_| {
			let checked = inbound_signal.get_untracked();
			outbound_signal.set(!checked);
		}),
		move || outbound_signal.get(),
	)
}
