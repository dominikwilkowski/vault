use floem::{
	style::Position,
	view::View,
	views::{container, scroll, Decorators},
};

use crate::ui::colors::*;
use crate::ui::settings_view::TABBAR_HEIGHT;

pub fn main<V: View + 'static>(children: impl Fn() -> V) -> impl View {
	container(scroll(children().style(|s| s.flex_col().items_start().padding_bottom(10.0))).style(|s| {
		s.flex_col()
			.flex_basis(0)
			.min_width(0)
			.flex_grow(1.0)
			.background(C_BG_MAIN)
			.class(scroll::Handle, |s| s.set(scroll::Thickness, 5.0))
	}))
	.style(|s| s.position(Position::Absolute).inset_top(TABBAR_HEIGHT).inset_bottom(0.0).width_full())
}
