use zeroize::Zeroize;

use floem::{
	event::{Event, EventListener},
	keyboard::{KeyCode, PhysicalKey},
	peniko::Brush,
	reactive::{
		create_rw_signal, use_context, RwSignal, SignalGet, SignalUpdate,
	},
	style::{CursorStyle, Display},
	views::{container, empty, label, slider::slider, toggle_button, Decorators},
	IntoView,
};

use crate::{
	env::Environment,
	ui::{
		app_view::{ToastSignalsSettings, TooltipSignalsSettings},
		colors::*,
		keyboard::is_submit,
		primitives::{
			button::{button, icon_button, IconButton},
			password_field::password_field,
			styles,
		},
	},
	DEFAULT_DEBUG_PASSWORD,
};

const MIN: f32 = 16.0;
const MAX: f32 = 2032.0;

fn change_password(
	old_password: RwSignal<String>,
	new_password: RwSignal<String>,
	new_password_check: RwSignal<String>,
	success: RwSignal<bool>,
) {
	let toast_signals = use_context::<ToastSignalsSettings>()
		.expect("No toast_signals context provider")
		.inner;
	let env = use_context::<Environment>().expect("No env context provider");

	success.set(false);
	if new_password.get() != new_password_check.get() {
		toast_signals.add(String::from("New passwords do not match"));
	} else if new_password.get().is_empty() {
		toast_signals.add(String::from("Empty passwords are not allowed"));
	} else {
		let result = env.db.change_password(old_password.get(), new_password.get());
		match result {
			Err(e) => {
				toast_signals.add(e.to_string());
			},
			Ok(()) => {
				old_password.update(|pass| pass.zeroize());
				new_password.update(|pass| pass.zeroize());
				new_password_check.update(|pass| pass.zeroize());
				toast_signals.kill_all_toasts();
				success.set(true);
			},
		}
	}
}

fn convert_pct_2_letter_count(pct: f32) -> usize {
	(((MAX / 100.0) * pct) + MIN).round() as usize
}

fn convert_letter_count_2_pct(timeout: f32) -> f32 {
	((timeout - MIN) / MAX) * 100.0
}

fn round_letter_count(letter_count: usize) -> usize {
	let mut letter_count = letter_count - 1;
	letter_count |= letter_count >> 1;
	letter_count |= letter_count >> 2;
	letter_count |= letter_count >> 4;
	letter_count |= letter_count >> 8;
	letter_count |= letter_count >> 16;
	letter_count |= letter_count >> 32;

	letter_count + 1
}

pub fn general_view() -> impl IntoView {
	let tooltip_signals = use_context::<TooltipSignalsSettings>()
		.expect("No tooltip_signals context provider")
		.inner;
	let env = use_context::<Environment>().expect("No env context provider");

	let save_icon = include_str!("../icons/save.svg");
	let revert_icon = include_str!("../icons/revert.svg");

	let env_salt = env.clone();

	let old_password = create_rw_signal(String::from(""));
	let new_password = create_rw_signal(String::from(""));
	let new_password_check = create_rw_signal(String::from(""));
	let success = create_rw_signal(false);

	let db_salt_letter_count_pct = convert_letter_count_2_pct(
		env.db.config_db.read().salt_letter_count as f32,
	);
	let salt_letter_count_pct = create_rw_signal(db_salt_letter_count_pct);
	let salt_letter_count_pct_backup = create_rw_signal(db_salt_letter_count_pct);

	let debug_settings_slot = if std::env::var("DEBUG").is_ok() {
		let is_encrypted = create_rw_signal(env.db.config_db.read().encrypted);

		(
			"Debug settings"
				.style(|s| s.inset_top(-5).margin_bottom(5).color(C_MAIN_BG_BORDER)),
			(
				label(move || {
					if is_encrypted.get() {
						"Disable encryption:"
					} else {
						"Enable encryption:"
					}
				}),
				toggle_button(move || is_encrypted.get())
					.on_toggle(move |_| {
						let new_state = !is_encrypted.get();
						env.db.config_db.write().encrypted = new_state;
						let _ = env.db.save();
						is_encrypted.set(new_state);
						if new_state {
							tooltip_signals
								.show(String::from("Decrypt with password in memory"));
						} else {
							tooltip_signals.show(format!(
								"Encrypt with default debug password \"{}\"",
								DEFAULT_DEBUG_PASSWORD
							));
						}
					})
					.style(styles::toggle_button)
					.on_event_cont(EventListener::PointerEnter, move |_| {
						if is_encrypted.get() {
							tooltip_signals
								.show(String::from("Decrypt with password in memory"));
						} else {
							tooltip_signals.show(format!(
								"Encrypt with default debug password \"{}\"",
								DEFAULT_DEBUG_PASSWORD
							));
						}
					})
					.on_event_cont(EventListener::PointerLeave, move |_| {
						tooltip_signals.hide();
					}),
			)
				.style(styles::settings_line),
		)
			.into_any()
			.style(move |s| {
				s.flex_col().border_top(1).border_color(C_MAIN_BG_BORDER).padding_top(5)
			})
			.style(|s| s.margin_top(20).width_full())
	} else {
		empty().into_any()
	};

	let change_password_slot = ((
		"Change Password",
		(
			password_field(old_password, "Old Password")
				.on_event_cont(EventListener::KeyDown, move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};
					if is_submit(key) {
						change_password(
							old_password,
							new_password,
							new_password_check,
							success,
						);
					}
				})
				.style(|s| s.width(250)),
			password_field(new_password, "New Password")
				.on_event_cont(EventListener::KeyDown, move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};
					if is_submit(key) {
						change_password(
							old_password,
							new_password,
							new_password_check,
							success,
						);
					}
				})
				.style(|s| s.width(250)),
			password_field(new_password_check, "New Password Again")
				.on_event_cont(EventListener::KeyDown, move |event| {
					let key = match event {
						Event::KeyDown(k) => k.key.physical_key,
						_ => PhysicalKey::Code(KeyCode::F35),
					};
					if is_submit(key) {
						change_password(
							old_password,
							new_password,
							new_password_check,
							success,
						);
					}
				})
				.style(|s| s.width(250)),
		)
			.style(|s| s.flex_col().column_gap(5)),
		empty(),
		(
			container("Password updated successfully".style(move |s| {
				s.color(C_SUCCESS)
					.display(Display::None)
					.apply_if(success.get(), |s| s.display(Display::Flex))
			}))
			.style(|s| s.height(17)),
			container(button("Change password").on_click_cont(move |_| {
				change_password(old_password, new_password, new_password_check, success)
			})),
		)
			.style(|s| s.flex_col().margin_bottom(20)),
		"Password salt",
		(
			label(move || {
				format!(
					"{} size",
					convert_pct_2_letter_count(salt_letter_count_pct.get())
				)
			}),
			(
				slider(move || salt_letter_count_pct.get())
					.slider_style(|s| {
						s.handle_color(Brush::Solid(C_FOCUS))
							.accent_bar_color(C_FOCUS.with_alpha_factor(0.5))
							.bar_height(5)
							.bar_color(C_FOCUS.with_alpha_factor(0.2))
							.handle_radius(6)
					})
					.style(|s| s.width(241).cursor(CursorStyle::Pointer))
					.on_change_pct(move |pct| {
						salt_letter_count_pct.set(convert_letter_count_2_pct(
							round_letter_count(convert_pct_2_letter_count(pct)) as f32,
						));
					}),
				container(
					(
						icon_button(
							IconButton {
								icon: String::from(revert_icon),
								tooltip: String::from("Reset"),
								tooltip_signals,
								..IconButton::default()
							},
							move |_| {
								salt_letter_count_pct.set(salt_letter_count_pct_backup.get());
								tooltip_signals.hide();
							},
						),
						icon_button(
							IconButton {
								icon: String::from(save_icon),
								tooltip: String::from("Save to database"),
								tooltip_signals,
								..IconButton::default()
							},
							move |_| {
								env_salt.db.config_db.write().salt_letter_count =
									convert_pct_2_letter_count(salt_letter_count_pct.get());
								env_salt.db.change_salt();
								let _ = env_salt.db.save();

								salt_letter_count_pct_backup.set(salt_letter_count_pct.get());
								tooltip_signals.hide();
							},
						),
					)
						.style(move |s| {
							s.row_gap(5).display(Display::Flex).apply_if(
								convert_pct_2_letter_count(salt_letter_count_pct.get())
									== convert_pct_2_letter_count(
										salt_letter_count_pct_backup.get(),
									),
								|s| s.display(Display::None),
							)
						}),
				)
				.style(|s| s.height(25)),
			)
				.style(|s| s.items_center().row_gap(5)),
		)
			.style(|s| s.flex_col()),
	)
		.style(styles::settings_line),)
		.style(|s| s.flex_col().width_full());

	(change_password_slot, debug_settings_slot)
		.style(|s| s.flex_col().margin_bottom(15))
}
