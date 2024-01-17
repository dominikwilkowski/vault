use floem::{
	reactive::RwSignal,
	views::{Decorators, TextInput},
	widgets::{text_input, PlaceholderTextClass},
};

use crate::ui::colors::*;

pub fn input_field(value: RwSignal<String>) -> TextInput {
	text_input(value).style(move |s| {
		s.padding_left(5)
			.padding_bottom(5)
			.padding_top(4)
			.border_radius(2)
			.border_color(C_TEXT_TOP)
			.cursor_color(C_FOCUS.with_alpha_factor(0.5))
			.hover(|s| s.background(C_FOCUS.with_alpha_factor(0.05)))
			.focus(|s| s.border_color(C_FOCUS).outline_color(C_FOCUS))
			.focus_visible(|s| s.outline(1))
			.class(PlaceholderTextClass, |s| {
				s.color(C_TEXT_MAIN.with_alpha_factor(0.5))
			})
	})
}
