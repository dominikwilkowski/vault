use std::time::Duration;

use floem::{
	action::exec_after,
	reactive::{create_effect, create_rw_signal, RwSignal},
	style::{CursorStyle, Display, Position},
	view::View,
	views::{
		h_stack, label, scroll, svg, v_stack, v_stack_from_iter, Decorators,
	},
};

use crate::ui::colors::*;

pub fn select(
	value: RwSignal<usize>,
	options: Vec<(usize, String)>,
	on_change: impl Fn(usize) + 'static,
) -> impl View {
	let first_option = options.first().unwrap_or(&(0, String::from(""))).clone();

	let chevron_icon = include_str!("../icons/chevron.svg");

	value.set(first_option.0);
	let select_text = create_rw_signal(first_option.1);
	let is_open = create_rw_signal(false);

	let options_backup = options.clone();
	create_effect(move |_| {
		let new_value = options_backup
			.clone()
			.into_iter()
			.find(|(option_id, _)| *option_id == value.get())
			.unwrap_or((0, String::from("")))
			.1;
		select_text.set(new_value);
		is_open.set(false);
		on_change(value.get());
	});

	let height = 25;

	v_stack((
		h_stack((
			label(move || select_text.get()).style(|s| s.width(80).text_ellipsis()),
			svg(move || String::from(chevron_icon))
				.style(move |s| s.height(height - 5).width(height - 5)),
		))
		.keyboard_navigatable()
		.on_event(floem::event::EventListener::FocusLost, move |_| {
			exec_after(Duration::from_millis(16), move |_| {
				is_open.set(false);
			});
			floem::EventPropagation::Continue
		})
		.on_click_cont(move |_| {
			is_open.set(!is_open.get());
		})
		.style(move |s| {
			s.position(Position::Relative)
				.items_center()
				.flex_grow(1.0)
				.height(height)
				.padding(3)
				.padding_left(8)
				.margin(3)
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
		}),
		scroll(
			v_stack((v_stack_from_iter(options.into_iter().map(|(id, option)| {
				label(move || option.clone())
					.on_click_cont(move |_| {
						value.set(id);
					})
					.keyboard_navigatable()
					.style(move |s| {
						s.padding(4)
							.padding_right(10)
							.margin(1)
							.width_full()
							.flex_grow(1.0)
							.hover(|s| {
								s.background(C_FOCUS.with_alpha_factor(0.2))
									.cursor(CursorStyle::Pointer)
							})
							.focus_visible(|s| s.outline(1).outline_color(C_FOCUS))
							.apply_if(id == value.get(), |s| s.background(C_BG_SIDE_SELECTED))
					})
			}))
			.style(|s| s.flex_grow(1.0).width_full()),))
			.style(|s| s.flex_grow(1.0).width_full()),
		)
		.style(move |s| {
			s.position(Position::Absolute)
				.inset_top(0)
				.margin_top(height + 5)
				.flex_grow(1.0)
				.min_width_full()
				.max_height(100)
				.background(C_BG_SIDE.with_alpha_factor(0.6))
				.box_shadow_blur(4)
				.box_shadow_color(C_SHADOW_2)
				.box_shadow_spread(2)
				.box_shadow_h_offset(2)
				.box_shadow_v_offset(2)
				.border_radius(3)
				.border(1)
				.border_color(C_TEXT_TOP)
				.z_index(200)
				.display(Display::None)
				.apply_if(is_open.get(), |s| s.display(Display::Flex))
		}),
	))
	.style(|s| s.position(Position::Relative))
}
