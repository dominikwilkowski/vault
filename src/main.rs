// #![windows_subsystem = "windows"]

use floem::{
	event::EventListener,
	kurbo::Size,
	menu::{Menu, MenuItem},
	reactive::create_rw_signal,
	view::View,
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
	pub mod details {
		pub mod button_slots;
		pub mod detail_view;
		pub mod list_item;
		pub mod new_field;
	}
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
	let view = container(
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
	.style(|s| s.width_full().height_full());

	Application::new()
		.window(
			move |_| {
				match std::env::var("DEBUG") {
					Ok(_) => {
						// for debugging the layout
						let id = view.id();
						view.on_event_stop(EventListener::KeyUp, move |e| {
							if let floem::event::Event::KeyUp(e) = e {
								if e.key.logical_key
									== floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11)
								{
									id.inspect();
								}
							}
						})
					}
					Err(_) => view,
				}
			},
			Some(
				WindowConfig::default().size(Size::new(800.0, 350.0)).title("Vault"),
			),
		)
		.run();
}
