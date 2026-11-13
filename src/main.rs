#![cfg_attr(
	all(target_os = "windows", not(debug_assertions)),
	windows_subsystem = "windows"
)]

use image::ImageReader;
use std::{io::Cursor, time::Duration};
use zeroize::Zeroize;

use crate::ui::secret;

use floem::{
	action::exec_after,
	event::{listener, EventPropagation},
	kurbo::Size,
	reactive::{Effect, RwSignal, SignalGet, SignalUpdate, SignalWith, Trigger},
	views::{dyn_container, Container, Decorators},
	window::{Icon, RgbaIcon, WindowConfig},
	Application, IntoView, View,
};
pub mod config;
pub mod db;
mod encryption;
mod env;
#[cfg(any(test, feature = "leakcheck"))]
mod leakcheck;
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
	pub mod focus;
	pub mod onboard_view;
	pub mod password_view;
	pub mod secret;
	pub mod shared_context;
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
		keyboard::{keycode_to_key, modifiersstate_to_keymodifier, Key},
		onboard_view::onboard_view,
		password_view::password_view,
		primitives::{
			debounce::Debounce, que::Que, styles, toast::ToastSignals,
			tooltip::TooltipSignals,
		},
		settings::settings_view::settings_view,
		shared_context,
		window_management::{
			close_all_windows, opening_window, provide_shared_context, WindowSpec,
		},
	},
};

// Finds secrets that are freed without being wiped (see leakcheck.rs).
#[cfg(any(test, feature = "leakcheck"))]
#[global_allocator]
static GLOBAL: leakcheck::LeakCheck = leakcheck::LeakCheck;

pub const DEFAULT_DEBUG_PASSWORD: &str = "p";

pub type TimeoutQueId = RwSignal<u8>;

pub fn create_lock_timeout() {
	let env =
		shared_context::get::<Environment>().expect("No env context provider");
	let que = shared_context::get::<Que>().expect("No que context provider");
	let timeout_que_id = shared_context::get::<TimeoutQueId>()
		.expect("No timeout_que_id context provider");

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
	let env =
		shared_context::get::<Environment>().expect("No env context provider");
	let que = shared_context::get::<Que>().expect("No que context provider");
	let app_state = shared_context::get::<RwSignal<AppState>>()
		.expect("No app_state context provider");

	close_all_windows();
	que.unque_all_tooltips();
	env.db.lock();
	*env.db.vault_unlocked.write() = false;
	app_state.set(AppState::PassPrompting);
	report_leaks_after_lock();
}

/// With the `leakcheck` feature on, print what was found in freed memory a few
/// seconds after locking, once the unlocked views have been dropped as well.
fn report_leaks_after_lock() {
	#[cfg(feature = "leakcheck")]
	exec_after(Duration::from_secs(5), |_| leakcheck::report_summary());
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
	#[cfg(feature = "leakcheck")]
	leakcheck::watch_from_env();

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

	// the master password only ever lives in this buffer, which wipes itself
	let password = secret::new_buffer();
	if !env.db.config_db.read().encrypted {
		password.update(|pass| pass.push_str(DEFAULT_DEBUG_PASSWORD));
	}

	let search_trigger = Trigger::new();

	let window_size = env.config.general.read().window_settings.window_size;

	Effect::new(move |_| {
		let state = app_state.get();
		if state == AppState::Ready || password.with(|pass| pass.is_empty()) {
			return;
		}

		if state == AppState::OnBoarding {
			// first run: create the vault, then unlock it below like any other, so that the state changes only once
			let _ = password.with_untracked(|pass| env_closure.db.set_password(pass));
			let _ = env_closure.save();
		}

		let decrypted =
			password.with_untracked(|pass| env_closure.db.decrypt_database(pass));
		Effect::untrack(|| {
			// right or wrong, the typed password is not needed any more
			password.update(|pass| pass.zeroize());
			match decrypted {
				Ok(()) => {
					toast_signals.kill_all_toasts();
					app_state.set(AppState::Ready);
				},
				Err(err) => {
					toast_signals.add(err.to_string());
				},
			}
		});
	});

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
				// context is scoped per window in floem, so provide it from inside the
				// window and remember it for the windows opened later on
				provide_shared_context(env.clone());
				provide_shared_context(que);
				provide_shared_context(tooltip_signals);
				provide_shared_context(toast_signals);
				provide_shared_context(app_state);
				provide_shared_context(timeout_que_id);
				// views and timers must be created once the application exists, otherwise
				// floem's update wake-up is marked as posted before it can be delivered
				if has_config && !has_db {
					toast_signals.add(String::from(
						"Database not found.\nGenerated a new empty database.",
					));
				}

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
									.on_event_cont(listener::WindowResized, move |_, size| {
										let size = (size.width, size.height);
										tooltip_signals.window_size.set(size);
										let fn_config = config_debounce.clone();
										debounce.clone().add(move || {
											fn_config.general.write().window_settings.window_size =
												size;
											let _ = fn_config.save();
										});
									})
									.on_event_cont(listener::WindowClosed, move |_, _| {
										let _ = config_close.save();
									})
							},
						},
					)
					.style(|s| s.width_full().height_full()),
				)
				.style(styles::default_window_styles)
				.style(|s| s.width_full().height_full())
				.into_view();

				let id = view.id();
				// floem re-dispatches shortcut keys that nobody consumed to every key
				// listener, so handled shortcuts stop propagation to run only once.
				view.on_event(listener::KeyDown, move |_, event| {
					let key = keycode_to_key(event.code);
					let modifier = modifiersstate_to_keymodifier(event.modifiers);
					let shortcuts = env_shortcuts.config.general.read().shortcuts.clone();
					let mut handled = false;

					if key == shortcuts.lock.0 && modifier == shortcuts.lock.1 {
						// the same path as the timer lock: it also closes the settings and
						// other windows, so that nothing can save over the locked vault
						lock_app();
						handled = true;
					}

					if key == shortcuts.search.0 && modifier == shortcuts.search.1 {
						search_trigger.notify();
						handled = true;
					}

					if key == shortcuts.settings.0 && modifier == shortcuts.settings.1 {
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
						handled = true;
					}

					if std::env::var("DEBUG").is_ok() && key == Key::F11 {
						id.inspect();
						handled = true;
					}

					if handled {
						EventPropagation::Stop
					} else {
						EventPropagation::Continue
					}
				})
			},
			Some(window_config),
		)
		.run();
}
