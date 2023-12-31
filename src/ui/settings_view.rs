use floem::{
	event::EventListener,
	reactive::create_signal,
	style::Position,
	view::View,
	views::{container, h_stack, label, scroll, tab, v_stack, Decorators},
};

use crate::ui::{
	colors::*,
	primitives::{button::tab_button, styles},
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Tabs {
	General,
	Editing,
	Database,
}

impl std::fmt::Display for Tabs {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Tabs::General => write!(f, "General"),
			Tabs::Editing => write!(f, "Editing"),
			Tabs::Database => write!(f, "Database"),
		}
	}
}

pub const TABBAR_HEIGHT: f64 = 63.0;

pub fn settings_view() -> impl View {
	let tabs = vec![Tabs::General, Tabs::Editing, Tabs::Database]
		.into_iter()
		.collect::<im::Vector<Tabs>>();
	let (tabs, _set_tabs) = create_signal(tabs);
	let (active_tab, set_active_tab) = create_signal(0);

	let settings_icon = include_str!("./icons/settings.svg");
	let editing_icon = include_str!("./icons/editing.svg");
	let database_icon = include_str!("./icons/database.svg");

	let tabs_bar = h_stack((
		tab_button(
			String::from(settings_icon),
			Tabs::General,
			tabs,
			set_active_tab,
			active_tab,
		),
		tab_button(
			String::from(editing_icon),
			Tabs::Editing,
			tabs,
			set_active_tab,
			active_tab,
		),
		tab_button(
			String::from(database_icon),
			Tabs::Database,
			tabs,
			set_active_tab,
			active_tab,
		),
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

	let main_content = container(scroll(tab(
			move || active_tab.get(),
			move || tabs.get(),
			|it| *it,
			|it| {
				match it {
				Tabs::General => container(label(move || String::from("General\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\nGeneral\n")).style(|s| s.padding(8.0))),
				Tabs::Editing => container(label(move || String::from("Editing")).style(|s| s.padding(8.0))),
				Tabs::Database => container(label(move || String::from("Database")).style(|s| s.padding(8.0))),
			}
			},
		).style(|s| s.flex_col().items_start().padding_bottom(10.0))).style(|s| {
		s.flex_col()
			.flex_basis(0)
			.min_width(0)
			.flex_grow(1.0)
			.background(C_BG_MAIN)
			.class(scroll::Handle, styles::scrollbar_styles)
	}))
	.style(|s| s.position(Position::Absolute).inset_top(TABBAR_HEIGHT).inset_bottom(0.0).width_full());

	let settings_view = v_stack((tabs_bar, main_content))
		.style(|s| s.width_full().height_full().gap(0, 5));

	match std::env::var("DEBUG") {
		Ok(_) => {
			// for debugging the layout
			let id = settings_view.id();
			settings_view.on_event_stop(EventListener::KeyUp, move |e| {
				if let floem::event::Event::KeyUp(e) = e {
					if e.key.logical_key
						== floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11)
					{
						id.inspect();
					}
				}
			})
		}
		Err(_) => settings_view,
	}
}
