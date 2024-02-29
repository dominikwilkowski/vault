use floem::{
	reactive::RwSignal,
	view::View,
	views::{editor::text::SimpleStyling, text_editor, Decorators},
};

use crate::ui::colors::*;

pub fn multiline_input_field(value: RwSignal<String>) -> impl View {
	text_editor(value.get()).styling(SimpleStyling::light()).gutter(false).style(
		|s| {
			s.padding_left(5)
				.padding_bottom(5)
				.padding_top(4)
				.border_radius(2)
				.border_color(C_TOP_TEXT)
				.cursor_color(C_FOCUS.with_alpha_factor(0.5))
				.hover(|s| s.background(C_FOCUS.with_alpha_factor(0.05)))
				.focus(|s| s.border_color(C_FOCUS).outline_color(C_FOCUS))
				.focus_visible(|s| s.outline(1))
		},
	)
}
