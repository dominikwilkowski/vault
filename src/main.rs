// #![windows_subsystem = "windows"]

use std::time::Duration;
use zeroize::Zeroize;

use floem::{
	action::exec_after,
	event::EventListener,
	kurbo::Size,
	menu::{Menu, MenuItem},
	reactive::{
		create_effect, create_rw_signal, provide_context, untrack, use_context,
		RwSignal,
	},
	view::View,
	views::{container, dyn_container, Decorators},
	window::WindowConfig,
	Application, EventPropagation,
};

pub mod config;
pub mod db;
mod encryption;
mod env;
mod password_gen;

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
	pub mod import {
		pub mod import_detail_view;
		pub mod import_view;
	}
	pub mod onboard_view;
	pub mod password_view;
	pub mod settings {
		pub mod database;
		pub mod editing;
		pub mod general;
		pub mod settings_view;
		pub mod shortcut;
	}
	pub mod window_management;
	pub mod primitives {
		pub mod button;
		pub mod checkbox;
		pub mod debounce;
		pub mod file_input;
		pub mod input_button_field;
		pub mod input_field;
		pub mod logo;
		pub mod multiline_input_field;
		pub mod password_field;
		pub mod que;
		pub mod select;
		pub mod styles;
		pub mod toast;
		pub mod tooltip;
	}
}

use crate::{
	env::Environment,
	ui::{
		app_view::app_view,
		onboard_view::onboard_view,
		password_view::password_view,
		primitives::{
			debounce::Debounce, que::Que, toast::ToastSignals,
			tooltip::TooltipSignals,
		},
	},
};

pub const DEFAULT_DEBUG_PASSWORD: &str = "p";

pub type TimeoutQueId = RwSignal<u8>;

pub fn create_lock_timeout(que: Que, env: Environment) {
	let app_state: RwSignal<AppState> =
		use_context().expect("No context provider");
	let timeout_que_id: TimeoutQueId =
		use_context().expect("No context provider");

	let timeout = env.config.general.read().db_timeout;
	let db_timeout = env.db.clone();
	let mut id = *que.lock.get().last().unwrap_or(&timeout_que_id.get());
	if id == 255 {
		id = 0;
	} else {
		id += 1;
	}
	que.lock.update(|item| item.push(id));
	timeout_que_id.set(id);

	exec_after(Duration::from_secs_f32(timeout), move |_| {
		if que.lock.get().contains(&id) {
			que.lock.update(|item| item.retain(|ids| *ids != id));

			que.tooltip.set(Vec::new()); // reset all tooltips before locking
			db_timeout.clear_hash();
			*db_timeout.vault_unlocked.write() = false;
			app_state.set(AppState::PassPrompting);
		}
	});
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
	OnBoarding,
	PassPrompting,
	Ready,
}

fn main() {
	let app_state = create_rw_signal(AppState::OnBoarding);
	let timeout_que_id: TimeoutQueId = create_rw_signal(0);

	let has_config = Environment::has_config().is_ok();
	if has_config {
		app_state.set(AppState::PassPrompting);
	}

	let env = if has_config {
		Environment::load()
	} else {
		Environment::default()
	};
	let env_closure = env.clone();

	let que = Que::default();
	let tooltip_signals = TooltipSignals::new(que);
	let toast_signals = ToastSignals::new(que);

	provide_context(env.clone());
	provide_context(que);
	provide_context(tooltip_signals);
	provide_context(toast_signals);
	provide_context(app_state);
	provide_context(timeout_que_id);

	let password = create_rw_signal(if !env.db.config_db.read().encrypted {
		String::from(DEFAULT_DEBUG_PASSWORD)
	} else {
		String::from("")
	});

	let window_size = env.config.general.read().window_settings.window_size;

	create_effect(move |_| match app_state.get() {
		AppState::OnBoarding => {
			if !password.get().is_empty() {
				let _ = env_closure.db.set_password(password.get());
				let _ = env_closure.save();
				password.update(|pass| pass.zeroize());
				app_state.set(AppState::PassPrompting);
			}
		},
		AppState::PassPrompting => {
			if !password.get().is_empty() {
				let decrypted = env_closure.db.decrypt_database(password.get());
				match decrypted {
					Ok(()) => {
						untrack(|| {
							password.update(|pass| pass.zeroize());
							toast_signals.kill_all_toasts();
							app_state.set(AppState::Ready);
						});
					},
					Err(err) => {
						untrack(|| {
							toast_signals.add(err.to_string());
						});
					},
				};
			}
		},
		AppState::Ready => {},
	});

	let view = container(
		dyn_container(
			move || app_state.get(),
			move |state| {
				match state {
					AppState::OnBoarding => onboard_view(password).any(),
					AppState::PassPrompting => {
						env.db.clear_hash();
						password_view(password).any()
					},
					AppState::Ready => {
						let config_close = env.config.clone();
						let config_debounce = env.config.clone();
						let debounce = Debounce::default();

						create_lock_timeout(que, env.clone());

						app_view()
							.any()
							.window_title(|| String::from("Vault"))
							.window_menu(|| {
								Menu::new("").entry(MenuItem::new("Menu item")).entry(
									MenuItem::new("Menu item with something on the\tright"),
								)
								// menus are currently commented out in the floem codebase
							})
							.on_resize(move |rect| {
								tooltip_signals.window_size.set((rect.x1, rect.y1));
								let fn_config = config_debounce.clone();
								debounce.clone().add(move || {
									fn_config.general.write().window_settings.window_size =
										(rect.x1, rect.y1);
									let _ = fn_config.save();
								});
							})
							.on_event(EventListener::WindowClosed, move |_| {
								let _ = config_close.save();
								EventPropagation::Continue
							})
					},
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
					},
					Err(_) => view,
				}
			},
			Some(
				WindowConfig::default()
					.size(Size::new(window_size.0, window_size.1))
					.title("Vault"),
			),
		)
		.run();
}
