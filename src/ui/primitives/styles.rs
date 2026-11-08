use floem::{
	peniko::{Brush, Color},
	style::{CursorStyle, Foreground, ScrollbarWidth, Style},
	taffy::style_helpers::{fr, length},
	views::{
		scroll::{Handle, ScrollClass, ScrollCustomStyle},
		slider::SliderClass,
		LabelClass, LabelCustomStyle, SvgClass, SvgColor,
	},
};

use crate::ui::colors::*;

pub fn default_window_styles(s: Style) -> Style {
	// text wrapped by default before; floem now clips unless told otherwise
	s.text_wrap()
		.class(LabelClass, |s| {
			s.apply(
				LabelCustomStyle::new()
					.selection_color(C_FOCUS.multiply_alpha(0.3))
					.into(),
			)
		})
		// floem tints every svg with the inherited text colour; keep the icons at the
		// colour they were drawn with
		.class(SvgClass, |s| s.set(SvgColor, Some(Brush::Solid(C_TOP_TEXT))))
		// sliders have no intrinsic height any more
		.class(SliderClass, |s| s.height(15))
		.class(ScrollClass, |s| {
			s.set(ScrollbarWidth, 5.0)
				.apply(ScrollCustomStyle::new().show_bars_when_idle(false).into())
		})
		.class(Handle, |s| {
			s.background(C_FOCUS.multiply_alpha(0.3))
				.hover(|s| s.background(C_FOCUS.multiply_alpha(0.7)))
				.active(|s| s.background(C_FOCUS))
		})
}

pub fn toggle_button(s: Style) -> Style {
	s.cursor(CursorStyle::Pointer)
		.background(Color::TRANSPARENT)
		.set(Foreground, Brush::Solid(C_FOCUS.multiply_alpha(0.5)))
		.border_color(C_TOP_TEXT)
		.hover(|s| {
			s.background(C_FOCUS.multiply_alpha(0.05))
				.border_color(C_FOCUS)
				.set(Foreground, Brush::Solid(C_FOCUS))
		})
		.focus(|s| {
			s.hover(|s| {
				s.background(C_FOCUS.multiply_alpha(0.05)).border_color(C_FOCUS)
			})
			.border_color(C_FOCUS)
			.set(Foreground, Brush::Solid(C_FOCUS))
		})
		.active(|s| {
			s.hover(|s| {
				s.background(C_FOCUS.multiply_alpha(0.2)).border_color(C_FOCUS)
			})
			.border_color(C_FOCUS)
			.set(Foreground, Brush::Solid(C_FOCUS))
		})
}

pub fn settings_line(s: Style) -> Style {
	s.grid()
		.grid_template_columns(vec![length(135.0_f32), fr(1.0_f32)])
		.items_center()
		.row_gap(5)
}

pub fn multiline(s: Style) -> Style {
	s.size_full().padding(5).border_radius(2).border(1).border_color(C_TOP_TEXT)
}

pub fn button(s: Style) -> Style {
	s.padding(3)
		.padding_left(4)
		.padding_right(4)
		.border_radius(3)
		.border(1)
		.border_color(C_TOP_TEXT)
		.border_radius(2)
		.box_shadow_blur(0.3)
		.box_shadow_color(C_SHADOW_3)
		.box_shadow_spread(0)
		.box_shadow_h_offset(2)
		.box_shadow_v_offset(2)
		.background(C_MAIN_BG)
		.items_center()
		.hover(|s| {
			s.background(C_SIDE_BG_SELECTED.multiply_alpha(0.6))
				.cursor(CursorStyle::Pointer)
		})
		.active(|s| {
			s.background(C_SIDE_BG_SELECTED)
				.margin_top(1)
				.padding_bottom(2)
				.box_shadow_h_offset(0)
				.box_shadow_v_offset(0)
		})
		.focus_visible(|s| s.outline(1).outline_color(C_FOCUS))
}

pub fn tag(s: Style) -> Style {
	s.padding_vert(2)
		.padding_horiz(3)
		.background(C_TOP_BG_INACTIVE)
		.border(1)
		.border_color(C_TOP_BG_BORDER)
		.border_radius(2)
}
