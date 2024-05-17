use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	reactive::{create_effect, create_rw_signal, RwSignal},
	style::{CursorStyle, Display, Position},
	views::{label, scroll, svg, v_stack_from_iter, Decorators},
	IntoView,
};

use crate::ui::colors::*;

pub fn select<
	T: std::fmt::Display + Clone + 'static + std::default::Default,
>(
	value: RwSignal<usize>,
	options: Vec<(usize, T)>,
	on_change: impl Fn(usize) + 'static,
) -> impl IntoView {
	let chevron_icon = include_str!("../icons/chevron.svg");

	let selected_text = options
		.clone()
		.into_iter()
		.find(|(id, _)| *id == value.get())
		.unwrap_or((0, Default::default()))
		.1;
	let select_text = create_rw_signal(selected_text);
	let is_open = create_rw_signal(false);

	let options_backup = options.clone();
	create_effect(move |_| {
		let new_value = options_backup
			.clone()
			.into_iter()
			.find(|(option_id, _)| *option_id == value.get())
			.unwrap_or((0, Default::default()))
			.1;
		select_text.set(new_value);
		is_open.set(false);
		on_change(value.get());
	});

	let height = 25;

	(
		(
			label(move || select_text.get()).style(|s| s.width(80).text_ellipsis()),
			svg(move || String::from(chevron_icon))
				.style(move |s| s.height(height - 5).width(height - 5)),
		)
			.keyboard_navigatable()
			.on_click_cont(move |_| {
				is_open.set(!is_open.get());
			})
			.on_event_cont(EventListener::KeyDown, move |event| {
				let key = match event {
					Event::KeyDown(k) => k.key.physical_key,
					_ => PhysicalKey::Code(KeyCode::F35),
				};

				if key == PhysicalKey::Code(KeyCode::Escape) {
					is_open.set(false);
				}
			})
			.style(move |s| {
				s.position(Position::Relative)
					.items_center()
					.flex_grow(1.0)
					.height(height)
					.padding(3)
					.padding_left(8)
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
					.hover(|s| {
						s.background(C_SIDE_BG_SELECTED.with_alpha_factor(0.6))
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
			}),
		scroll(
			(v_stack_from_iter(options.into_iter().map(|(id, option)| {
				label(move || option.clone())
					.keyboard_navigatable()
					.on_click_stop(move |_| {
						value.set(id);
					})
					.on_event_cont(EventListener::KeyDown, move |event| {
						let key = match event {
							Event::KeyDown(k) => k.key.physical_key,
							_ => PhysicalKey::Code(KeyCode::F35),
						};

						if key == PhysicalKey::Code(KeyCode::Escape) {
							is_open.set(false);
						}
					})
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
							.apply_if(id == value.get(), |s| s.background(C_SIDE_BG_SELECTED))
					})
			}))
			.style(|s| s.flex_grow(1.0).width_full()),)
				.style(|s| s.flex_col().flex_grow(1.0).width_full()),
		)
		.style(move |s| {
			s.position(Position::Absolute)
				.inset_top(0)
				.margin_top(height + 5)
				.flex_grow(1.0)
				.min_width_full()
				.max_height(100)
				.background(C_SIDE_BG.with_alpha_factor(0.6))
				.box_shadow_blur(4)
				.box_shadow_color(C_SHADOW_2)
				.box_shadow_spread(2)
				.box_shadow_h_offset(2)
				.box_shadow_v_offset(2)
				.border_radius(3)
				.border(1)
				.border_color(C_TOP_TEXT)
				.z_index(200)
				.display(Display::None)
				.apply_if(is_open.get(), |s| s.display(Display::Flex))
		}),
	)
		.style(|s| s.flex_col().position(Position::Relative))
}
