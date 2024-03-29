use floem::views::{
	editor::{
		core::indent::IndentStyle,
		text::{default_light_theme, SimpleStyling},
	},
	text_editor, Decorators, TextEditor,
};

use crate::ui::colors::*;

pub fn multiline_input_field(value: String) -> TextEditor {
	let mut style = SimpleStyling::new();
	style.set_tab_width(2);
	style.set_font_size(12);

	text_editor(value)
		.styling(style)
		.style(|s| {
			s.size_full()
				.color(C_TOP_TEXT)
				.background(C_MAIN_BG)
				.font_size(12.0)
				.box_shadow_color(C_MAIN_BG)
			// TODO: This is buggy
			// .border(1)
			// .border_color(C_MAIN_BG_BORDER)
			// .border_radius(3)
			// .focus(|s| s.border_color(C_FOCUS))
		})
		.editor_style(default_light_theme)
		.editor_style(move |s| {
			s.hide_gutter(true)
				.indent_style(IndentStyle::Tabs)
				.current_line_color(C_MAIN_BG)
				.indent_guide_color(C_MAIN_BG)
				.preedit_underline_color(C_MAIN_BG)
				.placeholder_color(C_MAIN_TEXT.with_alpha_factor(0.5))
				.cursor_color(C_FOCUS.with_alpha_factor(0.5))
				.selection_color(C_FOCUS.with_alpha_factor(0.5))
				.visible_whitespace(C_MAIN_BG)
				.scroll_beyond_last_line(true)
		})
		.keyboard_navigatable()
}
