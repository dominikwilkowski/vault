use floem::{
	reactive::RwSignal,
	style::Style,
	views::{Decorators, TextInput},
	widgets::{text_input, PlaceholderTextClass},
};

use crate::ui::colors::*;

pub fn input_field(
	value: RwSignal<String>,
	override_styles: impl Fn(Style) -> Style + 'static,
) -> TextInput {
	text_input(value).style(move |s| {
		override_styles(s)
			.padding(5.0)
			.padding_top(4)
			.border_radius(2)
			.border_color(C_TEXT_TOP)
			.cursor_color(C_FOCUS.with_alpha_factor(0.5))
			.hover(|s| s.background(C_FOCUS.with_alpha_factor(0.05)))
			.focus(|s| s.border_color(C_FOCUS).outline_color(C_FOCUS))
			.class(PlaceholderTextClass, |s| {
				s.color(C_TEXT_MAIN.with_alpha_factor(0.5))
			})
	})
}
