// #![windows_subsystem = "windows"]

use floem::{
	kurbo::Size,
	menu::{Menu, MenuItem},
	reactive::create_rw_signal,
	views::{container, dyn_container, Decorators},
	window::WindowConfig,
	Application,
};

pub mod config;
pub mod db;
mod encryption;

mod ui {
	pub mod app_view;
	pub mod colors;
	pub mod detail_view;
	pub mod history_view;
	pub mod password_view;
	pub mod settings_view;
	pub mod window_management;
	pub mod primitives {
		pub mod button;
		pub mod input_field;
		pub mod styles;
		pub mod tooltip;
	}
}

use crate::ui::app_view::app_view;
use crate::ui::password_view::password_view;

fn main() {
	let password = create_rw_signal(String::from(""));

	Application::new()
		.window(
			move |_| {
				container(
					dyn_container(
						move || password.get(),
						move |pass_value| {
							if pass_value.is_empty() {
								Box::new(password_view(password))
							} else {
								Box::new(
									app_view(config::Config::new())
										.window_title(|| String::from("Vault"))
										.window_menu(|| {
											Menu::new("").entry(MenuItem::new("Menu item")).entry(
												MenuItem::new("Menu item with something on the\tright"),
											)
											// menus are currently commented out in the floem codebase
										}),
								)
							}
						},
					)
					.style(|s| s.width_full().height_full()),
				)
				.style(|s| s.width_full().height_full())
			},
			Some(
				WindowConfig::default().size(Size::new(800.0, 350.0)).title("Vault"),
			),
		)
		.run();
}
