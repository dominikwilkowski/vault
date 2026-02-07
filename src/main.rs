#![cfg_attr(
	all(target_os = "windows", not(debug_assertions)),
	windows_subsystem = "windows"
)]

use image::ImageReader;
use std::{io::Cursor, time::Duration};
use zeroize::Zeroize;

use floem::{
	action::exec_after,
	event::{Event, EventListener},
	kurbo::Size,
	menu::Menu,
	reactive::{
		Context, Effect, Trigger,
		RwSignal, SignalGet, SignalUpdate,
	},
	ui_events::keyboard::KeyState,
	views::{Container, dyn_container, Decorators},
	window::{Icon, RgbaIcon, WindowConfig},
	Application, IntoView, View,
};
pub mod config;
pub mod db;
mod encryption;
mod env;
mod password_gen;

mod ui {
	pub mod app_view;
	pub mod colors;
	pub mod keyboard;
	pub mod details {
		pub mod button_slots;
		pub mod detail_view;
		pub mod dyn_field_title_form;
		pub mod heading;
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
		keyboard::{
			code_to_key, modifiersstate_to_keymodifier, Key, KeyModifier,
		},
		onboard_view::onboard_view,
		password_view::password_view,
		primitives::{
			debounce::Debounce, que::Que, toast::ToastSignals,
			tooltip::TooltipSignals,
		},
		settings::settings_view::settings_view,
		window_management::{close_all_windows, opening_window, WindowSpec},
	},
};

pub const DEFAULT_DEBUG_PASSWORD: &str = "p";

pub type TimeoutQueId = RwSignal<u8>;

pub fn create_lock_timeout() {
	let env = Context::get::<Environment>().expect("No env context provider");
	let que = Context::get::<Que>().expect("No que context provider");
	let timeout_que_id =
		Context::get::<TimeoutQueId>().expect("No timeout_que_id context provider");

	let timeout = env.config.general.read().db_timeout;

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

			lock_app();
		}
	});
}

pub fn lock_app() {
	let env = Context::get::<Environment>().expect("No env context provider");
	let que = Context::get::<Que>().expect("No que context provider");
	let app_state =
		Context::get::<RwSignal<AppState>>().expect("No app_state context provider");

	close_all_windows();
	que.unque_all_tooltips();
	env.db.lock();
	*env.db.vault_unlocked.write() = false;
	app_state.set(AppState::PassPrompting);
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
	OnBoarding,
	PassPrompting,
	Ready,
}

fn window_icon() -> Icon {
	let image =
		ImageReader::new(Cursor::new(include_bytes!("../assets/64x64.png")))
			.with_guessed_format()
			.unwrap()
			.decode()
			.unwrap()
			.into_rgba8();
	let (icon_width, icon_height) = image.dimensions();
	let icon_rgba = image.into_raw();
	Icon::from(
		RgbaIcon::new(icon_rgba, icon_width, icon_height)
			.expect("Failed to open icon"),
	)
}

fn main() {
	let app_state = RwSignal::new(AppState::OnBoarding);
	let timeout_que_id: TimeoutQueId = RwSignal::new(0);

	let has_config = Environment::has_config().is_ok();
	let has_db = Environment::has_db();
	if has_config && has_db {
		app_state.set(AppState::PassPrompting);
	}

	let env = if has_config {
		Environment::load()
	} else {
		Environment::default()
	};
	let env_closure = env.clone();
	let env_shortcuts = env.clone();

	let que = Que::default();
	let tooltip_signals = TooltipSignals::new(que);
	let toast_signals = ToastSignals::new(que);

	Context::provide(env.clone());
	Context::provide(que);
	Context::provide(tooltip_signals);
	Context::provide(toast_signals);
	Context::provide(app_state);
	Context::provide(timeout_que_id);

	if has_config && !has_db {
		toast_signals.add(String::from(
			"Database not found.\nGenerated a new empty database.",
		));
	}

	let password = RwSignal::new(if !env.db.config_db.read().encrypted {
		String::from(DEFAULT_DEBUG_PASSWORD)
	} else {
		String::from("")
	});

	let search_trigger = Trigger::new();

	let window_size = env.config.general.read().window_settings.window_size;

	Effect::new(move |_| match app_state.get() {
		AppState::OnBoarding => {
			if !password.get().is_empty() {
				let _ = env_closure.db.set_password(password.get());
				let _ = env_closure.save();
				app_state.set(AppState::PassPrompting);
				password.update(|pass| pass.zeroize());
			}
		},
		AppState::PassPrompting => {
			if !password.get().is_empty() {
				let decrypted = env_closure.db.decrypt_database(password.get());
				match decrypted {
					Ok(()) => {
						Effect::untrack(|| {
							password.update(|pass| pass.zeroize());
							toast_signals.kill_all_toasts();
							app_state.set(AppState::Ready);
						});
					},
					Err(err) => {
						Effect::untrack(|| {
							toast_signals.add(err.to_string());
						});
					},
				};
			}
		},
		AppState::Ready => {},
	});

	let view = Container::new(
		dyn_container(
			move || app_state.get(),
			move |state| match state {
				AppState::OnBoarding => onboard_view(password).into_any(),
				AppState::PassPrompting => password_view(password).into_any(),
				AppState::Ready => {
					let config_close = env.config.clone();
					let config_debounce = env.config.clone();
					let debounce = Debounce::default();

					create_lock_timeout();

					app_view(search_trigger)
						.into_any()
						.window_title(|| String::from("Vault"))
						.window_menu(|| {
							Menu::new("")
								.entry(MenuItem::new("Menu item"))
								.entry(MenuItem::new("Menu item with something on the\tright"))
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
						.on_event_cont(EventListener::WindowClosed, move |_| {
							let _ = config_close.save();
						})
				},
			},
		)
		.style(|s| s.width_full().height_full()),
	)
	.style(|s| s.width_full().height_full())
	.into_view();

	let window_config = WindowConfig::default()
		.size(Size::new(window_size.0, window_size.1))
		.title("Vault")
		.with_mac_os_config(|settings| {
			settings
				.movable_by_window_background(false)
				.tabbing_identifier("Main Vault Window")
		})
		.window_icon(window_icon());

	Application::new()
		.window(
			move |_| {
				let id = view.id();
				view.on_event_cont(EventListener::KeyDown, move |event| {
					let (key, modifier) = match event {
						Event::Key(k) if k.state == KeyState::Down => (
							code_to_key(k.code),
							modifiersstate_to_keymodifier(k.modifiers),
						),
						_ => (Key::F35, KeyModifier::None),
					};

					if key == env_shortcuts.config.general.read().shortcuts.lock.0
						&& modifier == env_shortcuts.config.general.read().shortcuts.lock.1
					{
						que.unque_all_tooltips();
						env_shortcuts.db.lock();
						*env_shortcuts.db.vault_unlocked.write() = false;
						app_state.set(AppState::PassPrompting);
					}

					if key == env_shortcuts.config.general.read().shortcuts.search.0
						&& modifier
							== env_shortcuts.config.general.read().shortcuts.search.1
					{
						search_trigger.notify();
					}

					if key == env_shortcuts.config.general.read().shortcuts.settings.0
						&& modifier
							== env_shortcuts.config.general.read().shortcuts.settings.1
					{
						opening_window(
							settings_view,
							WindowSpec {
								id: String::from("settings-window"),
								title: String::from("Vault Settings"),
							},
							Size::new(500.0, 400.0),
							false,
							move || {
								que.unque_all_tooltips();
							},
						);
					}

					if std::env::var("DEBUG").is_ok() && key == Key::F11 {
						id.inspect();
					}
				})
			},
			Some(window_config),
		)
		.run();
}
