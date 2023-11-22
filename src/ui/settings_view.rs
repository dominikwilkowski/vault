use floem::{
	event::EventListener,
	reactive::{create_signal, ReadSignal, WriteSignal},
	style::{AlignItems, CursorStyle, Position},
	view::View,
	views::{container, h_stack, label, scroll, svg, tab, v_stack, Decorators},
};
use std::fmt;

use crate::ui::colors::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Tabs {
	General,
	Editing,
	Database,
}

impl fmt::Display for Tabs {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Tabs::General => write!(f, "General"),
			Tabs::Editing => write!(f, "Editing"),
			Tabs::Database => write!(f, "Database"),
		}
	}
}

fn tab_button(
	icon: String,
	this_tab: Tabs,
	tabs: ReadSignal<im::Vector<Tabs>>,
	set_active_tab: WriteSignal<usize>,
	active_tab: ReadSignal<usize>,
) -> impl View {
	v_stack((
		svg(move || icon.clone()).style(|s| s.width(30).height(30)),
		label(move || this_tab).style(|s| s.justify_center().margin_top(2)),
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
			.hover(|s| s.background(C_BG_MAIN).cursor(CursorStyle::Pointer))
			.apply_if(active_tab.get() == tabs.get_untracked().iter().position(|it| *it == this_tab).unwrap(), |s| {
				s.background(C_BG_MAIN)
			})
	})
}

const TABBAR_HEIGHT: f64 = 63.0;

pub fn settings_view() -> impl View {
	let tabs = vec![Tabs::General, Tabs::Editing, Tabs::Database].into_iter().collect::<im::Vector<Tabs>>();
	let (tabs, _set_tabs) = create_signal(tabs);
	let (active_tab, set_active_tab) = create_signal(0);

	let settings_icon = include_str!("./icons/settings.svg");
	let editing_icon = include_str!("./icons/editing.svg");
	let database_icon = include_str!("./icons/database.svg");

	let tabs_bar = h_stack((
		tab_button(String::from(settings_icon), Tabs::General, tabs, set_active_tab, active_tab),
		tab_button(String::from(editing_icon), Tabs::Editing, tabs, set_active_tab, active_tab),
		tab_button(String::from(database_icon), Tabs::Database, tabs, set_active_tab, active_tab),
	))
	.style(|s| {
		s.flex_row()
			.width_full()
			.height(TABBAR_HEIGHT)
			.gap(5, 0)
			.padding(5)
			.border_bottom(1)
			.border_color(C_BG_TOP_BORDER)
			.background(C_BG_TOP)
	});

	let main_content = container(scroll(
		tab(
			move || active_tab.get(),
			move || tabs.get(),
			|it| *it,
			|it| match it {
				Tabs::General => container(label(move || String::from("General\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\n")).style(|s| s.padding(8.0))),
				Tabs::Editing => container(label(move || String::from("Editing")).style(|s| s.padding(8.0))),
				Tabs::Database => container(label(move || String::from("Database")).style(|s| s.padding(8.0))),
			},
		)
		.style(|s| s.flex_col().items_start().padding_bottom(10.0)),
	).style(|s| s.flex_col()
			.flex_basis(0)
			.min_width(0)
			.flex_grow(1.0)
			.background(C_BG_MAIN)
			.class(scroll::Handle, |s| s.set(scroll::Thickness, 5.0))))
	.style(|s| s.position(Position::Absolute).inset_top(TABBAR_HEIGHT).inset_bottom(0.0).width_full());

	let view =
		v_stack((tabs_bar, main_content)).style(|s| s.width_full().gap(0, 5)).style(|s| s.width_full().height_full());

	match std::env::var("DEBUG") {
		Ok(_) => {
			// for debugging the layout
			let id = view.id();
			view.on_event_stop(EventListener::KeyUp, move |e| {
				if let floem::event::Event::KeyUp(e) = e {
					if e.key.logical_key == floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11) {
						id.inspect();
					}
				}
			})
		}
		Err(_) => view,
	}
}
