#![windows_subsystem = "windows"]

use floem::{
	kurbo::Size,
	menu::{Menu, MenuItem},
	views::Decorators,
	window::WindowConfig,
	Application,
};

pub mod config;

mod ui {
	pub mod app_view;
	pub mod colors;
	pub mod detail_view;
	pub mod settings_view;
	pub mod primitives {
		pub mod button;
		pub mod input_field;
		pub mod styles;
	}
}

mod db {
	pub mod db;
}

use crate::ui::app_view::app_view;

fn main() {
	let config = config::Config::new();
	println!("{:?}", config);
	Application::new()
		.window(
			|_| {
				app_view().window_title(|| String::from("Vault")).window_menu(|| {
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
