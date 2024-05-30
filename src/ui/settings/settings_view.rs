use floem::{
	event::{Event, EventListener},
	reactive::{create_rw_signal, provide_context, use_context},
	style::Position,
	views::{container, scroll, tab, Decorators},
	IntoView, View,
};

use crate::ui::{
	app_view::{QueSettings, ToastSignalsSettings, TooltipSignalsSettings},
	colors::*,
	primitives::{
		button::tab_button,
		styles,
		toast::{toast_view, ToastSignals},
		tooltip::tooltip_view,
	},
	settings::{
		database::database_view, editing::editing_view, general::general_view,
		shortcut::shortcut_view,
	},
};

pub const TABBAR_HEIGHT: f64 = 63.0;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Tabs {
	General,
	Editing,
	Database,
	Shortcuts,
}

impl std::fmt::Display for Tabs {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Tabs::General => write!(f, "General"),
			Tabs::Editing => write!(f, "Editing"),
			Tabs::Database => write!(f, "Database"),
			Tabs::Shortcuts => write!(f, "Shortcuts"),
		}
	}
}

pub fn settings_view() -> impl IntoView {
	let que =
		use_context::<QueSettings>().expect("No que context provider").inner;
	let tooltip_signals = use_context::<TooltipSignalsSettings>()
		.expect("No tooltip_signals context provider")
		.inner;

	let tabs = vec![
		Tabs::General,
		Tabs::Editing,
		Tabs::Database,
		Tabs::Shortcuts,
	]
	.into_iter()
	.collect::<im::Vector<Tabs>>();
	let tabs = create_rw_signal(tabs);
	let active_tab = create_rw_signal(0);

	let settings_icon = include_str!("../icons/settings.svg");
	let editing_icon = include_str!("../icons/editing.svg");
	let database_icon = include_str!("../icons/database.svg");
	let shortcut_icon = include_str!("../icons/shortcut.svg");

	let toast_signals = ToastSignalsSettings {
		inner: ToastSignals::new(que),
	};
	provide_context(toast_signals);
	let toast_signals = toast_signals.inner;

	let tabs_bar = (
		tab_button(String::from(settings_icon), Tabs::General, tabs, active_tab),
		tab_button(String::from(editing_icon), Tabs::Editing, tabs, active_tab),
		tab_button(String::from(database_icon), Tabs::Database, tabs, active_tab),
		tab_button(String::from(shortcut_icon), Tabs::Shortcuts, tabs, active_tab),
	)
		.style(|s| {
			s.flex_row()
				.width_full()
				.height(TABBAR_HEIGHT)
				.row_gap(5)
				.padding(5)
				.border_bottom(1)
				.border_color(C_TOP_BG_BORDER)
				.background(C_TOP_BG)
		});

	let main_content = container(
		scroll(
			tab(
				move || active_tab.get(),
				move || tabs.get(),
				|it| *it,
				move |it| match it {
					Tabs::General => general_view()
						.into_any()
						.style(|s| s.padding(8.0).padding_bottom(10.0)),
					Tabs::Editing => editing_view()
						.into_any()
						.style(|s| s.padding(8.0).padding_bottom(10.0)),
					Tabs::Database => database_view()
						.into_any()
						.style(|s| s.padding(8.0).padding_bottom(10.0)),
					Tabs::Shortcuts => shortcut_view()
						.into_any()
						.style(|s| s.padding(8.0).padding_bottom(10.0)),
				},
			)
			.style(|s| s.flex_col().items_start().margin_top(10)),
		)
		.on_scroll(move |_| {
			tooltip_signals.hide();
		})
		.style(|s| {
			s.flex_col()
				.flex_basis(0)
				.min_width(0)
				.flex_grow(1.0)
				.background(C_MAIN_BG)
				.class(scroll::Handle, styles::scrollbar_styles)
		}),
	)
	.style(|s| {
		s.position(Position::Absolute)
			.inset_top(TABBAR_HEIGHT)
			.inset_bottom(0.0)
			.width_full()
	});

	let settings_view = (
		tooltip_view(tooltip_signals),
		toast_view(toast_signals),
		tabs_bar,
		main_content,
	)
		.style(|s| s.flex_col().width_full().height_full().column_gap(5))
		.on_event_cont(EventListener::PointerMove, move |event| {
			let pos = match event {
				Event::PointerMove(p) => p.pos,
				_ => (0.0, 0.0).into(),
			};
			tooltip_signals.mouse_pos.set((pos.x, pos.y));
		})
		.on_resize(move |event| {
			tooltip_signals.window_size.set((event.x1, event.y1));
		});

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
		},
		Err(_) => settings_view,
	}
}
