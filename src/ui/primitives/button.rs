use floem::{
	cosmic_text::Weight,
	event::Event,
	reactive::{ReadSignal, WriteSignal},
	style::{AlignItems, CursorStyle},
	view::View,
	views::{container, label, svg, v_stack, Decorators},
};

use crate::ui::colors::*;
use crate::ui::settings_view::Tabs;

pub fn tab_button(
	icon: String,
	this_tab: Tabs,
	tabs: ReadSignal<im::Vector<Tabs>>,
	set_active_tab: WriteSignal<usize>,
	active_tab: ReadSignal<usize>,
) -> impl View {
	v_stack((
		svg(move || icon.clone()).style(|s| s.width(30).height(30)),
		label(move || this_tab).style(|s| s.justify_center()),
	))
	.on_click_stop(move |_| {
		set_active_tab.update(|v: &mut usize| {
			*v = tabs.get_untracked().iter().position(|it| *it == this_tab).unwrap();
		});
	})
	.style(move |s| {
		s.flex()
			.width(58)
			.align_items(AlignItems::Center)
			.background(C_BG_SIDE_SELECTED.with_alpha_factor(0.2))
			.border_radius(3)
			.padding(3)
			.gap(0, 2.0)
			.hover(|s| s.background(C_BG_MAIN).cursor(CursorStyle::Pointer))
			.apply_if(
				active_tab.get()
					== tabs
						.get_untracked()
						.iter()
						.position(|it| *it == this_tab)
						.unwrap(),
				|s| s.background(C_BG_MAIN).font_weight(Weight::BOLD).gap(0, 0),
			)
	})
}

pub fn icon_button(
	icon: String,
	on_click: impl Fn(&Event) + 'static,
) -> impl View {
	container(svg(move || icon.clone()).style(|s| s.height(17.0).width(17.0)))
		.style(|s| {
			s.padding(3)
				.margin(3)
				.margin_left(0)
				.margin_right(1.5)
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
		})
		.on_click_stop(on_click)
}
