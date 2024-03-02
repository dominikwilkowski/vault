use floem::{
	id::Id,
	reactive::RwSignal,
	view::{AnyView, View},
	views::{
		container,
		editor::{
			color::EditorColor, core::indent::IndentStyle, text::SimpleStyling,
		},
		text_editor, Decorators,
	},
};

use crate::ui::colors::*;

pub struct Multiline {
	pub view: AnyView,
	pub input_id: Id,
}

pub fn multiline_input_field(value: RwSignal<String>) -> Multiline {
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

	let input = text_editor(value.get());
	let input_id = input.id();

	let view = container(input.styling(style).gutter(false))
		.style(|s| {
			s.size_full()
				.padding(5)
				.border_radius(2)
				.border(1)
				.border_color(C_TOP_TEXT)
		})
		.any();

	Multiline { view, input_id }
}
