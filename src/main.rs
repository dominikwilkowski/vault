use floem::{
	event::{Event, EventListener},
	keyboard::{Key, NamedKey},
	menu::{Menu, MenuItem},
	reactive::create_signal,
	style::{Background, CursorStyle, Transition},
	unit::UnitExt,
	view::View,
	views::{
		container, container_box, h_stack, label, scroll, stack, tab, virtual_list, Decorators, VirtualListDirection,
		VirtualListItemSize,
	},
	EventPropagation,
};

pub mod colors;

use crate::colors::*;

fn app_view() -> impl View {
	let tabs: im::Vector<&str> = vec![
		"password 1",
		"password 2",
		"password 3",
		"password 4",
		"password 5 with really long text and stuff",
		"password 6",
		"password 7",
		"password 8",
		"password 9",
		"password 10",
		"password 11",
	]
	.into_iter()
	.collect();

	let (tabs, _set_tabs) = create_signal(tabs);
	let (active_tab, set_active_tab) = create_signal(0);

	let sidebar_list = scroll({
		virtual_list(
			VirtualListDirection::Vertical,
			VirtualListItemSize::Fixed(Box::new(|| 36.0)),
			move || tabs.get(),
			move |item| *item,
			move |item| {
				let index = tabs.get_untracked().iter().position(|it| *it == item).unwrap();
				stack((label(move || item).style(|s| s.font_size(12.0)),))
					.on_click_stop(move |_| {
						set_active_tab.update(|v: &mut usize| {
							*v = tabs.get_untracked().iter().position(|it| *it == item).unwrap();
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
										if active < tabs.get().len() - 1 {
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
		.style(|s| s.border_right(1.0).border_color(C_BG_SIDE_BORDER).min_height(0).background(C_BG_SIDE));

	let main_window = tab(
		move || active_tab.get(),
		move || tabs.get(),
		|it| *it,
		|it| match it {
			thing => container_box(label(move || String::from(thing))),
		},
	)
	.style(|s| s.flex_col().items_start());

	let main_window = scroll(main_window).style(|s| s.flex_basis(0).min_width(0).flex_grow(1.0));

	h_stack((sidebar_list, main_window)).style(|s| s.width_full().height_full()).window_title(|| String::from("Vault"))

	// let id = view.id();
	// view.on_event_stop(EventListener::KeyUp, move |e| {
	// 	if let Event::KeyUp(e) = e {
	// 		if e.key.logical_key == Key::Named(NamedKey::F11) {
	// 			id.inspect();
	// 		}
	// 	}
	// })
}

fn main() {
	floem::launch(|| {
		app_view().window_menu(|| {
			Menu::new("").entry(MenuItem::new("Menu item")).entry(MenuItem::new("Menu item with something on the\tright"))
		})
	});
}
