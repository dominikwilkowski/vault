use floem::{
	event::Event,
	reactive::{ReadSignal, WriteSignal},
	style::{AlignItems, CursorStyle, Display, Position},
	view::View,
	views::{label, svg, v_stack, Decorators},
};

use crate::ui::{colors::*, settings_view::Tabs};

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
		label(move || "").style(move |s| {
			s.position(Position::Absolute)
				.z_index(5)
				.width(58)
				.height(3)
				.inset_left(0)
				.inset_top(55)
				.background(C_BG_MAIN)
				.display(Display::None)
				.apply_if(
					active_tab.get()
						== tabs
							.get_untracked()
							.iter()
							.position(|it| *it == this_tab)
							.unwrap(),
					|s| s.display(Display::Flex),
				)
		}),
	))
	.keyboard_navigatable()
	.on_click_stop(move |_| {
		set_active_tab.update(|v: &mut usize| {
			*v = tabs.get_untracked().iter().position(|it| *it == this_tab).unwrap();
		});
	})
	.style(move |s| {
		s.flex()
			.width(60)
			.height(52)
			.align_items(AlignItems::Center)
			.background(C_BG_TOP)
			.border_radius(6)
			.padding(3)
			.gap(0, 2.0)
			.border(1)
			.border_color(C_BG_TOP)
			.focus_visible(|s| s.outline(1).outline_color(C_FOCUS))
			.hover(|s| {
				s.background(C_BG_MAIN)
					.cursor(CursorStyle::Pointer)
					.border_color(C_BG_MAIN)
			})
			.apply_if(
				active_tab.get()
					== tabs
						.get_untracked()
						.iter()
						.position(|it| *it == this_tab)
						.unwrap(),
				|s| {
					s.background(C_BG_MAIN)
						.height(63)
						.padding_top(6)
						.padding_bottom(11)
						.inset_top(0)
						.border_color(C_BG_TOP_BORDER)
						.hover(|s| s.border_color(C_BG_TOP_BORDER))
				},
			)
	})
}

pub fn icon_button(
	icon: String,
	notification: usize,
	on_click: impl Fn(&Event) + 'static,
) -> impl View {
	let notification_icon = include_str!("../icons/notification.svg");

	let bubble = if notification > 0 {
		let notification_text = if notification < 100 {
			format!("{notification}")
		} else {
			String::from("x")
		};

		let right = if notification < 10 {
			-2.5
		} else if notification < 100 {
			-0.5
		} else {
			-2.5
		};

		v_stack((v_stack((
			svg(move || String::from(notification_icon))
				.style(|s| s.height(10).width(10)),
			label(move || notification_text.clone()).style(move |s| {
				s.color(C_TEXT_MAIN)
					.height(8)
					.width(10)
					.font_size(8.0)
					.position(Position::Absolute)
					.inset_top(0)
					.inset_right(right)
			}),
		)),))
		.style(|s| s.position(Position::Absolute).inset_top(0).inset_right(0))
	} else {
		v_stack((label(|| "").style(|s| s.display(Display::None)),))
	};

	v_stack((svg(move || icon.clone()).style(|s| s.height(17).width(17)), bubble))
		.keyboard_navigatable()
		.style(move |s| {
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
				.focus_visible(|s| s.outline(1).outline_color(C_FOCUS))
		})
		.on_click_stop(on_click)
}
