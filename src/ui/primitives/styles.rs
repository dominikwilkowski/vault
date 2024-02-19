use floem::{
	peniko::Color,
	style::{CursorStyle, Foreground, Style},
	taffy::style_helpers::{fr, points},
	views::scroll,
};

use crate::ui::colors::*;

pub fn scrollbar_styles(s: Style) -> Style {
	s.background(C_FOCUS.with_alpha_factor(0.3))
		.hover(|s| s.background(C_FOCUS.with_alpha_factor(0.7)))
		.active(|s| s.background(C_FOCUS))
		.set(scroll::Thickness, 5.0)
}

pub fn toggle_button(s: Style) -> Style {
	s.cursor(CursorStyle::Pointer)
		.background(Color::TRANSPARENT)
		.set(Foreground, C_FOCUS.with_alpha_factor(0.5))
		.border_color(C_TEXT_TOP)
		.hover(|s| {
			s.background(C_FOCUS.with_alpha_factor(0.05))
				.border_color(C_FOCUS)
				.set(Foreground, C_FOCUS)
		})
		.focus(|s| {
			s.hover(|s| {
				s.background(C_FOCUS.with_alpha_factor(0.05)).border_color(C_FOCUS)
			})
			.border_color(C_FOCUS)
			.set(Foreground, C_FOCUS)
		})
		.active(|s| {
			s.hover(|s| {
				s.background(C_FOCUS.with_alpha_factor(0.2)).border_color(C_FOCUS)
			})
			.border_color(C_FOCUS)
			.set(Foreground, C_FOCUS)
		})
}

pub fn settings_line(s: Style) -> Style {
	s.grid()
		.grid_template_columns(vec![points(125.0), fr(1.0)])
		.items_center()
		.gap(0, 5)
}

pub fn button(s: Style) -> Style {
	s.padding(3)
		.margin(3)
		.margin_left(0)
		.border_radius(3)
		.border(1)
		.border_color(C_TEXT_TOP)
		.border_radius(2)
		.box_shadow_blur(0.3)
		.box_shadow_color(C_SHADOW_3)
		.box_shadow_spread(0)
		.box_shadow_h_offset(2)
		.box_shadow_v_offset(2)
		.background(C_BG_MAIN)
		.hover(|s| {
			s.background(C_BG_SIDE_SELECTED.with_alpha_factor(0.6))
				.cursor(CursorStyle::Pointer)
		})
		.active(|s| {
			s.background(C_BG_SIDE_SELECTED)
				.margin_top(4)
				.padding_bottom(2)
				.box_shadow_h_offset(0)
				.box_shadow_v_offset(0)
		})
		.focus_visible(|s| s.outline(1).outline_color(C_FOCUS))
}
