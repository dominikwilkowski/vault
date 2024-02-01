// #![windows_subsystem = "windows"]

use parking_lot::RwLock;
use std::{sync::Arc, time::Duration};

use floem::{
	action::exec_after,
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
		pub mod dyn_field_title_form;
		pub mod hidden_fields;
		pub mod list_item;
		pub mod new_field;
	}
	pub mod history_view;
	pub mod password_view;
	pub mod settings {
		pub mod database;
		pub mod editing;
		pub mod general;
		pub mod settings_view;
	}
	pub mod window_management;
	pub mod primitives {
		pub mod button;
		pub mod input_button_field;
		pub mod input_field;
		pub mod password_field;
		pub mod select;
		pub mod styles;
		pub mod tooltip;
	}
}

use crate::ui::app_view::app_view;
use crate::ui::password_view::password_view;

pub const DEFAULT_DEBUG_PASSWORD: &str = "p";

fn main() {
	let password = create_rw_signal(String::from(""));
	let error = create_rw_signal(String::from(""));
	let config = Arc::new(RwLock::new(config::Config::new()));

	let view = container(
		dyn_container(
			move || password.get(),
			move |pass_value| {
				if !pass_value.is_empty() {
					let decrypted = config.write().decrypt_database(pass_value);
					match decrypted {
						Ok(()) => (),
						Err(e) => error.set(e.to_string()),
					}
				}

				let is_encrypted = config.read().config_db.read().encrypted;
				let is_unlocked = *config.read().vault_unlocked.read();

				if !is_unlocked && is_encrypted {
					config.write().clear_hash(); // TODO: Need a signal maybe for clearing it
					password_view(password, error).any()
				} else {
					if password.get().is_empty() && !is_encrypted {
						password.set(String::from(DEFAULT_DEBUG_PASSWORD)); // in debug mode - not encrypted and for debug only
					} else {
						let timeout = config.read().general.read().db_timeout;
						let timeout_config = config.clone();
						exec_after(Duration::from_secs_f32(timeout), move |_| {
							timeout_config.write().clear_hash();
							*timeout_config.write().vault_unlocked.write() = false;
							password.set(String::from(""));
							error.set(String::from(""));
						});
					}

					app_view(password, config.write().clone())
						.any()
						.window_title(|| String::from("Vault"))
						.window_menu(|| {
							Menu::new("")
								.entry(MenuItem::new("Menu item"))
								.entry(MenuItem::new("Menu item with something on the\tright"))
							// menus are currently commented out in the floem codebase
						})
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
