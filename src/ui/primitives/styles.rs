use floem::{style::Style, views::scroll};

use crate::ui::colors::*;

pub fn scrollbar_styles(s: Style) -> Style {
	s.background(C_FOCUS.with_alpha_factor(0.3))
		.hover(|s| s.background(C_FOCUS.with_alpha_factor(0.7)))
		.active(|s| s.background(C_FOCUS))
		.set(scroll::Thickness, 5.0)
}
