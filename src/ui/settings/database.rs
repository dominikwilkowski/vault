use std::fs;

use floem::{
	event::{Event, EventListener},
	file::{FileDialogOptions, FileInfo, FileSpec},
	file_action::save_as,
	keyboard::{KeyCode, PhysicalKey},
	kurbo::Size,
	reactive::{create_rw_signal, use_context, RwSignal},
	style::{CursorStyle, Display},
	view::View,
	views::{container, h_stack, label, svg, v_stack, Decorators},
	widgets::slider::slider,
};

use crate::{
	create_lock_timeout,
	db::Db,
	env::Environment,
	ui::{
		app_view::{
			QueSettings, SidebarList, ToastSignalsSettings, TooltipSignalsSettings,
		},
		colors::*,
		import::import_view::import_view,
		primitives::{
			button::{button, icon_button, IconButton},
			file_input::file_input,
			password_field::password_field,
			que::Que,
			select::select,
			styles,
			toast::ToastSignals,
		},
		window_management::{closing_window, opening_window, WindowSpec},
	},
};

const MIN: f32 = 60.0; // 1min
const MAX: f32 = (60.0 * 60.0 * 10.0) - 60.0; // 60s -> 60min -> 10h minus MIN

fn convert_pct_2_timeout(pct: f32) -> f32 {
	((MAX / 100.0) * pct) + MIN
}

fn convert_timeout_2_pct(timeout: f32) -> f32 {
	((timeout - MIN) / MAX) * 100.0
}

fn human_readable(seconds: f32) -> String {
	let hours = (seconds / 3600.0).floor() as usize;
	let minutes = ((seconds % 3600.0) / 60.0).floor() as usize;
	let remaining_seconds = (seconds % 60.0).floor() as usize;

	let mut result = String::new();

	if hours > 0 {
		result.push_str(&format!("{}h ", hours));
	}

	if minutes > 0 {
		result.push_str(&format!("{}min ", minutes));
	}

	if remaining_seconds > 0 || (hours == 0 && minutes == 0) {
		result.push_str(&format!("{}sec", remaining_seconds));
	}

	String::from(result.trim())
}

fn export(file: FileInfo, env: Environment) {
	match fs::write(file.path[0].clone(), env.db.export().unwrap()) {
		Ok(_) => {},
		Err(_) => panic!("Can't write export file"),
	};
}

pub fn import(
	import_list: im::Vector<(usize, bool)>,
	import_db: Db,
	env: Environment,
) {
	let list_sidebar_signal = use_context::<SidebarList>()
		.expect("No list_sidebar_signal context provider");

	for &(import_id, is_selected) in &import_list {
		if is_selected {
			let import_entry = import_db.get_by_id(&import_id);
			let new_id = env.db.add(import_entry.title);

			import_db.get_fields(&import_id).iter().for_each(
				|(import_field, is_visible)| {
					let name = import_db.get_name_of_field(&import_id, import_field);
					let kind = import_db.get_field_kind(&import_id, import_field);

					let mut history: Vec<(u64, String)> = import_db
						.get_history(&import_id, import_field)
						.unwrap_or_default()
						.into_iter()
						.collect();

					let last = history.len() - 1;
					let value = import_db.get_n_by_field(&import_id, import_field, last);

					let field = env.db.add_field(&new_id, kind, name, value);
					if !is_visible {
						env.db.edit_field_visbility(&new_id, &field, false);
					}

					history.reverse();
					history.iter().skip(1).for_each(|(_, value)| {
						env.db.edit_field(new_id, &field, value.clone());
					})
				},
			);
		}
	}

	let _ = env.save();
	list_sidebar_signal.set(env.db.get_sidebar_list());
	closing_window(String::from("import-window"), || ());
}

fn import_window(
	import_path: RwSignal<Vec<String>>,
	import_password: RwSignal<String>,
	toast_signals: ToastSignals,
	env: Environment,
) {
	if !import_path.get().is_empty() {
		let imported_db = Db::load(import_path.get()[0].clone());
		let decrypted = imported_db.decrypt_database(import_password.get());
		match decrypted {
			Ok(()) => {
				import_path.set(Vec::new());
				import_password.set(String::from(""));

				let que_import = Que::default();

				opening_window(
					move || import_view(imported_db.clone(), que_import, env.clone()),
					WindowSpec {
						id: String::from("import-window"),
						title: String::from("Import into Vault"),
					},
					Size::new(300.0, 350.0),
					move || {
						que_import.unque_all_tooltips();
					},
				);
			},
			Err(err) => {
				toast_signals.add(err.to_string());
			},
		};
	}
}

enum Snap {
	NoSnaping,
	ToMinute,
	ToTenMinutes,
	ToHalfHour,
	ToHour,
}

pub fn database_view() -> impl View {
	let que =
		use_context::<QueSettings>().expect("No que context provider").inner;
	let tooltip_signals = use_context::<TooltipSignalsSettings>()
		.expect("No tooltip_signals context provider")
		.inner;
	let toast_signals = use_context::<ToastSignalsSettings>()
		.expect("No toast_signals context provider")
		.inner;
	let env = use_context::<Environment>().expect("No env context provider");

	let db_timeout = env.config.general.read().db_timeout;
	let timeout_backup = create_rw_signal(db_timeout);
	let timeout = create_rw_signal(convert_timeout_2_pct(db_timeout));
	let snap = create_rw_signal(0);
	let import_path = create_rw_signal(Vec::new());
	let import_password = create_rw_signal(String::from(""));

	let env_export = env.clone();
	let env_import_enter = env.clone();
	let env_import_click = env.clone();

	let all_snaps = [
		Snap::NoSnaping,
		Snap::ToMinute,
		Snap::ToTenMinutes,
		Snap::ToHalfHour,
		Snap::ToHour,
	];

	let save_icon = include_str!("../icons/save.svg");
	let revert_icon = include_str!("../icons/revert.svg");
	let snap_icon = include_str!("../icons/snap.svg");
	let download_icon = include_str!("../icons/download.svg");

	container(
		v_stack((
			label(|| "Auto lock after"),
			v_stack((
				label(move || {
					human_readable(convert_pct_2_timeout(timeout.get()).round())
				}),
				slider(move || timeout.get())
					.slider_style(|s| {
						s.handle_color(C_FOCUS)
							.accent_bar_color(C_FOCUS.with_alpha_factor(0.5))
							.bar_height(5)
							.bar_color(C_FOCUS.with_alpha_factor(0.2))
							.handle_radius(6)
					})
					.style(|s| s.width(200).cursor(CursorStyle::Pointer))
					.on_change_pct(move |pct| {
						let snaping = &all_snaps[snap.get()];

						match snaping {
							Snap::NoSnaping => {
								let seconds = convert_pct_2_timeout(pct).round();
								timeout.set(convert_timeout_2_pct(seconds));
							},
							Snap::ToMinute => {
								let seconds =
									((convert_pct_2_timeout(pct) / 60.0).floor() * 60.0).round();
								timeout.set(convert_timeout_2_pct(seconds));
							},
							Snap::ToTenMinutes => {
								let seconds = ((convert_pct_2_timeout(pct) / (60.0 * 10.0))
									.ceil() * (60.0 * 10.0))
									.round();
								timeout.set(convert_timeout_2_pct(seconds));
							},
							Snap::ToHalfHour => {
								let seconds = ((convert_pct_2_timeout(pct) / (60.0 * 30.0))
									.ceil() * (60.0 * 30.0))
									.round();
								timeout.set(convert_timeout_2_pct(seconds));
							},
							Snap::ToHour => {
								let seconds = ((convert_pct_2_timeout(pct) / (60.0 * 60.0))
									.ceil() * (60.0 * 60.0))
									.round();
								timeout.set(convert_timeout_2_pct(seconds));
							},
						};
					}),
				h_stack((
					svg(move || String::from(snap_icon))
						.style(|s| s.width(16).height(16)),
					label(|| "Snap to:"),
					select(
						snap,
						vec![
							(0, "No snapping"),
							(1, "Minute"),
							(2, "10 minutes"),
							(3, "30 minutes"),
							(4, "Hour"),
						],
						move |_| {},
					)
					.style(|s| s.margin_right(1)),
					h_stack((
						icon_button(
							IconButton {
								icon: String::from(revert_icon),
								tooltip: String::from("Reset"),
								tooltip_signals,
								..IconButton::default()
							},
							move |_| {
								timeout.set(convert_timeout_2_pct(timeout_backup.get()));
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
								let seconds = convert_pct_2_timeout(timeout.get()).round();
								env.config.general.write().db_timeout = seconds;
								timeout_backup.set(seconds);
								tooltip_signals.hide();
								que.lock.set(Vec::new()); // invalidate the current timeout
								let _ = env.config.save();

								create_lock_timeout();
							},
						),
					))
					.style(move |s| {
						s.gap(5, 0).display(Display::Flex).apply_if(
							(convert_pct_2_timeout(timeout.get())
								- timeout_backup.get().abs())
							.abs() < f32::EPSILON,
							|s| s.display(Display::None),
						)
					}),
				))
				.style(|s| s.gap(5, 0).items_center()),
			)),
			label(|| "Backup data").style(|s| s.margin_top(20)),
			container(
				h_stack((
					label(|| "Export").style(|s| s.margin_left(5)),
					svg(move || String::from(download_icon))
						.style(|s| s.width(16).height(16).margin_left(5)),
				))
				.style(styles::button)
				.style(|s| s.items_center())
				.on_click_cont(move |_| {
					let env_export = env_export.clone();
					save_as(
						FileDialogOptions::new()
							.default_name("vault.backup")
							.title("Save backup file"),
						move |file_info| {
							if let Some(file) = file_info {
								export(file, env_export.clone());
							}
						},
					);
				}),
			)
			.style(|s| s.margin_top(20)),
			label(|| "Importing data").style(|s| s.margin_top(20)),
			v_stack((
				file_input(
					import_path,
					FileDialogOptions::new()
						.allowed_types(vec![FileSpec {
							name: "backup",
							extensions: &["backup", "vault", "toml"],
						}])
						.title("Select import file"),
					move |_| {},
				)
				.style(|s| s.width(200)),
				password_field(import_password, "Enter password for import file")
					.on_event_cont(EventListener::KeyDown, move |event| {
						let key = match event {
							Event::KeyDown(k) => k.key.physical_key,
							_ => PhysicalKey::Code(KeyCode::F35),
						};

						if key == PhysicalKey::Code(KeyCode::Enter) {
							import_window(
								import_path,
								import_password,
								toast_signals,
								env_import_enter.clone(),
							);
						}
					})
					.style(|s| s.width(200)),
				container(button("Import").on_click_cont(move |_| {
					import_window(
						import_path,
						import_password,
						toast_signals,
						env_import_click.clone(),
					);
				}))
				.style(|s| s.margin_top(5)),
			))
			.style(|s| s.margin_top(20).gap(0, 5)),
		))
		.style(styles::settings_line),
	)
}
