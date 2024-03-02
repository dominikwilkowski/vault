use floem::{
	views::{
		editor::{
			color::EditorColor,
			core::indent::IndentStyle,
			text::SimpleStyling,
		}, text_editor, Decorators, TextEditor
	},
};

use crate::ui::colors::*;

pub fn multiline_input_field(value: String) -> TextEditor {
	let mut style = SimpleStyling::light();
	style.set_font_size(12);
	style.set_tab_width(2);
	style.set_indent_style(IndentStyle::Tabs);
	style.set_color(|c| match c {
		EditorColor::Background => C_MAIN_BG,
		EditorColor::Foreground => C_TOP_TEXT,
		EditorColor::CurrentLine => C_MAIN_BG,
		EditorColor::IndentGuide => C_MAIN_BG,
		EditorColor::PreeditUnderline => C_MAIN_BG,
		EditorColor::DropdownShadow => C_MAIN_BG,
		EditorColor::Dim => C_MAIN_BG,
		EditorColor::Caret => C_FOCUS.with_alpha_factor(0.5),
		EditorColor::StickyHeaderBackground => C_MAIN_BG,
		EditorColor::Focus => C_FOCUS.with_alpha_factor(0.5),
		EditorColor::Selection => C_FOCUS.with_alpha_factor(0.5),
		EditorColor::Link => C_FOCUS,
		EditorColor::VisibleWhitespace => C_MAIN_BG,
		EditorColor::Scrollbar => C_MAIN_BG,
	});

	text_editor(value).styling(style).gutter(false).keyboard_navigatable()
}
