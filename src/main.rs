#![windows_subsystem = "windows"]

use floem::{
	kurbo::Size,
	menu::{Menu, MenuItem},
	views::Decorators,
	window::WindowConfig,
	Application,
};

pub mod config;
pub mod db;

mod ui {
	pub mod app_view;
	pub mod colors;
	pub mod detail_view;
	pub mod settings_view;
	pub mod primitives {
		pub mod button;
		pub mod input_field;
		pub mod styles;
		pub mod tooltip;
	}
}

use crate::ui::app_view::app_view;

fn main() {
	let mut config = config::SharedConfig::default();

	Application::new()
		.window(
			move |_| {
				app_view(&mut config)
					.window_title(|| String::from("Vault"))
					.window_menu(|| {
						Menu::new("")
							.entry(MenuItem::new("Menu item"))
							.entry(MenuItem::new("Menu item with something on the\tright"))
						// menus are currently commented out in the floem codebase
					})
			},
			Some(
				WindowConfig::default().size(Size::new(800.0, 350.0)).title("Vault"),
			),
		)
		.run();
}
