// #![windows_subsystem = "windows"]

use parking_lot::RwLock;
use std::{sync::Arc, time::Duration};

use floem::{
	action::exec_after,
	event::EventListener,
	kurbo::Size,
	menu::{Menu, MenuItem},
	reactive::{create_rw_signal, RwSignal},
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

use crate::{
	config::Config,
	ui::{app_view::app_view, password_view::password_view},
};

pub const DEFAULT_DEBUG_PASSWORD: &str = "p";

#[derive(Debug, Copy, Clone)]
pub struct Que {
	tooltip: RwSignal<Vec<u8>>,
	lock: RwSignal<Vec<u8>>,
}

impl Default for Que {
	fn default() -> Self {
		Self {
			tooltip: create_rw_signal(Vec::new()),
			lock: create_rw_signal(Vec::new()),
		}
	}
}

pub fn create_lock_timeout(
	timeout_que_id: RwSignal<u8>,
	password: RwSignal<String>,
	que: Que,
	config: Config,
) {
	let timeout = config.general.read().db_timeout;
	let timeout_config = config.clone();
	let mut id = que.lock.get().last().unwrap_or(&timeout_que_id.get()) + 1;
	if id > 254 {
		id = 1;
	}
	que.lock.update(|item| item.push(id));
	timeout_que_id.set(id);

	exec_after(Duration::from_secs_f32(timeout), move |_| {
		if que.lock.get().contains(&id) {
			que.lock.update(|item| item.retain(|ids| *ids != id));

			que.tooltip.set(Vec::new()); // reset all tooltips before locking
			timeout_config.clear_hash();
			*timeout_config.vault_unlocked.write() = false;
			password.set(String::from(""));
		}
	});
}

fn main() {
	let password = create_rw_signal(String::from(""));
	let error = create_rw_signal(String::from(""));
	let timeout_que_id = create_rw_signal(0);
	let config = Arc::new(RwLock::new(Config::new()));
	let que = Que::default();

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
						create_lock_timeout(
							timeout_que_id,
							password,
							que,
							config.write().clone(),
						);
						error.set(String::from(""));
					}

					app_view(password, timeout_que_id, que, config.write().clone())
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
