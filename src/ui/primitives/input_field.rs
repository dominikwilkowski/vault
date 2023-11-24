use floem::{
	reactive::RwSignal,
	views::{Decorators, TextInput},
	widgets::{text_input, PlaceholderTextClass},
};

use crate::ui::colors::*;

pub fn input_field(value: RwSignal<String>) -> TextInput {
	text_input(value).style(|s| {
		s.padding(5.0)
			.width_full()
			.padding_right(30)
			.margin(3.0)
			.border_radius(2)
			.z_index(3)
			.border_color(C_TEXT_TOP)
			.cursor_color(C_FOCUS.with_alpha_factor(0.5))
			.hover(|s| s.background(C_FOCUS.with_alpha_factor(0.05)))
			.focus(|s| s.border_color(C_FOCUS).outline_color(C_FOCUS))
			.class(PlaceholderTextClass, |s| {
				s.color(C_TEXT_MAIN.with_alpha_factor(0.5))
			})
	})
}
