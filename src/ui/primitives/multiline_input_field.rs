use floem::{
	id::Id,
	reactive::RwSignal,
	view::{AnyView, View},
	views::{
		container,
		editor::{
			color::EditorColor,
			core::indent::IndentStyle,
			text::{default_light_color, SimpleStyling},
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
		_ => default_light_color(c),
	});

	let input = text_editor(value.get());
	let input_id = input.id();

	let view = container(input.styling(style).gutter(false))
		.style(|s| {
			s.size_full().border_radius(2).border(1).border_color(C_TOP_TEXT)
		})
		.any();

	Multiline { view, input_id }
}
