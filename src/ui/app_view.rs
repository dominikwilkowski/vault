use floem::{
	event::{Event, EventListener},
	keyboard::{Key, NamedKey},
	reactive::{create_rw_signal, create_signal},
	style::{Background, CursorStyle, Transition},
	unit::UnitExt,
	view::View,
	views::{
		container, container_box, h_stack, label, scroll, stack, tab, v_stack, virtual_list, Decorators,
		VirtualListDirection, VirtualListItemSize,
	},
	widgets::text_input,
	EventPropagation,
};

use crate::db::db::{get_by_id, get_list};
use crate::ui::colors::*;

pub fn app_view() -> impl View {
	let db = get_list();
	let (list, set_list) = create_signal(db.clone());
	let (active_tab, set_active_tab) = create_signal(0);

	let sidebar_list = scroll({
		virtual_list(
			VirtualListDirection::Vertical,
			VirtualListItemSize::Fixed(Box::new(|| 36.0)),
			move || list.get(),
			move |item| *item,
			move |item| {
				let index = list.get_untracked().iter().position(|it| *it == item).unwrap();
				stack((label(move || item.1).style(|s| s.font_size(12.0).padding(5.0)),))
					.on_click_stop(move |_| {
						set_active_tab.update(|v: &mut usize| {
							*v = list.get_untracked().iter().position(|it| *it == item).unwrap();
						});
					})
					.on_event(EventListener::KeyDown, move |e| {
						if let Event::KeyDown(key_event) = e {
							let active = active_tab.get();
							if key_event.modifiers.is_empty() {
								match key_event.key.logical_key {
									Key::Named(NamedKey::ArrowUp) => {
										if active > 0 {
											set_active_tab.update(|v| *v -= 1)
										}
										EventPropagation::Stop
									}
									Key::Named(NamedKey::ArrowDown) => {
										if active < list.get().len() - 1 {
											set_active_tab.update(|v| *v += 1)
										}
										EventPropagation::Stop
									}
									_ => EventPropagation::Continue,
								}
							} else {
								EventPropagation::Continue
							}
						} else {
							EventPropagation::Continue
						}
					})
					.keyboard_navigatable()
					.style(move |s| {
						s.flex_row()
							.padding(5.0)
							.width(100.pct())
							.height(36.0)
							.transition(Background, Transition::linear(0.4))
							.items_center()
							.border_bottom(1.0)
							.border_color(C_BG_SIDE_BORDER)
							.color(C_TEXT_SIDE)
							.apply_if(index == active_tab.get(), |s| s.background(C_BG_SIDE_SELECTED))
							.focus_visible(|s| s.border(2.).border_color(C_FOCUS))
							.hover(|s| {
								s.background(C_BG_SIDE_SELECTED.with_alpha_factor(0.6))
									.apply_if(index == active_tab.get(), |s| s.background(C_BG_SIDE_SELECTED))
									.cursor(CursorStyle::Pointer)
							})
					})
			},
		)
		.style(|s| s.flex_col().width(140.0))
	})
	.style(|s| s.flex_col().width(140.0).flex_grow(1.0).min_height(0).flex_basis(0));

	let sidebar_list = container(sidebar_list)
		.style(|s| s.border_right(1.0).border_color(C_BG_SIDE_BORDER).min_height(0).background(C_BG_SIDE).height_full());

	let search_text = create_rw_signal("".to_string());
	let search_bar = container_box(
		text_input(search_text)
			.keyboard_navigatable()
			.on_event(EventListener::KeyDown, move |_| {
				set_list.update(|list: &mut im::Vector<(usize, &'static str)>| {
					*list =
						db.iter().map(|item| *item).filter(|item| item.1.contains(&search_text.get())).collect::<im::Vector<_>>();
				});
				EventPropagation::Continue
			})
			.style(|s| {
				s.padding(5.0)
					.width(138.0)
					.margin_top(3.0)
					.margin_bottom(3.0)
					.margin_left(1.0)
					.margin_right(1.0)
					.border_radius(2)
			}),
	);

	let sidebar = v_stack((search_bar, sidebar_list));

	let main_window = tab(
		move || active_tab.get(),
		move || list.get(),
		|it| *it,
		|it| match it {
			thing => {
				let data = get_by_id(thing.0);
				container_box(
					label(move || format!("id:{} title:{} body:{}", data.0, data.1, data.2)).style(|s| s.padding(8.0)),
				)
			}
		},
	)
	.style(|s| s.flex_col().items_start());

	let main_window = scroll(main_window).style(|s| s.flex_basis(0).min_width(0).flex_grow(1.0));

	h_stack((sidebar, main_window)).style(|s| s.width_full().height_full()).window_title(|| String::from("Vault"))

	// let id = view.id();
	// view.on_event_stop(EventListener::KeyUp, move |e| {
	// 	if let Event::KeyUp(e) = e {
	// 		if e.key.logical_key == Key::Named(NamedKey::F11) {
	// 			id.inspect();
	// 		}
	// 	}
	// })
}
