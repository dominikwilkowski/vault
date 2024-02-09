use floem::{
	action::open_file,
	file::FileDialogOptions,
	reactive::{create_rw_signal, RwSignal},
	style::{CursorStyle, Display, Foreground},
	view::View,
	views::{container, h_stack, label, svg, v_stack, Decorators},
	widgets::slider::{slider, AccentBarClass, BarClass, HandleRadius},
};

use crate::{
	create_lock_timeout,
	env::Environment,
	ui::{
		colors::*,
		primitives::{
			button::{icon_button, IconButton},
			file_input::file_input,
			password_field::password_field,
			select::select,
			styles,
			tooltip::TooltipSignals,
		},
	},
	Que,
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

enum Snap {
	NoSnaping,
	ToMinute,
	ToTenMinutes,
	ToHalfHour,
	ToHour,
}

pub fn database_view(
	password: RwSignal<String>,
	timeout_que_id: RwSignal<u8>,
	que: Que,
	tooltip_signals: TooltipSignals,
	env: Environment,
) -> impl View {
	let db_timeout = env.config.general.read().db_timeout;
	let timeout_backup = create_rw_signal(db_timeout);
	let timeout_sec = create_rw_signal(db_timeout);
	let timeout = create_rw_signal(convert_timeout_2_pct(db_timeout));
	let snap = create_rw_signal(0);
	let import_password = create_rw_signal(String::from(""));

	let all_snaps = vec![
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
				label(move || human_readable(timeout_sec.get())),
				slider(move || timeout.get())
					.style(|s| {
						s.width(200)
							.hover(|s| s.cursor(CursorStyle::Pointer))
							.class(AccentBarClass, |s| {
								s.background(C_FOCUS.with_alpha_factor(0.5))
							})
							.class(BarClass, |s| {
								s.height(5)
									.background(C_FOCUS.with_alpha_factor(0.2))
									.border_radius(0)
							})
							.set(Foreground, C_FOCUS)
							.set(HandleRadius, 6)
					})
					.on_change_pct(move |pct| {
						let snaping = &all_snaps[snap.get()];

						match snaping {
							Snap::NoSnaping => {
								let seconds = convert_pct_2_timeout(pct).round();
								timeout_sec.set(seconds);
								timeout.set(convert_timeout_2_pct(seconds));
							},
							Snap::ToMinute => {
								let seconds =
									((convert_pct_2_timeout(pct) / 60.0).floor() * 60.0).round();
								timeout_sec.set(seconds);
								timeout.set(convert_timeout_2_pct(seconds));
							},
							Snap::ToTenMinutes => {
								let seconds = ((convert_pct_2_timeout(pct) / (60.0 * 10.0))
									.floor() * (60.0 * 10.0))
									.round();
								timeout_sec.set(seconds);
								timeout.set(convert_timeout_2_pct(seconds));
							},
							Snap::ToHalfHour => {
								let seconds = ((convert_pct_2_timeout(pct) / (60.0 * 30.0))
									.floor() * (60.0 * 30.0))
									.round();
								timeout_sec.set(seconds);
								timeout.set(convert_timeout_2_pct(seconds));
							},
							Snap::ToHour => {
								let seconds = ((convert_pct_2_timeout(pct) / (60.0 * 60.0))
									.floor() * (60.0 * 60.0))
									.round();
								timeout_sec.set(seconds);
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
					),
					h_stack((
						icon_button(
							IconButton {
								icon: String::from(save_icon),
								tooltip: String::from("Save to database"),
								tooltip_signals,
								..IconButton::default()
							},
							move |_| {
								let seconds = timeout_sec.get();
								env.config.general.write().db_timeout = seconds;
								timeout_backup.set(seconds);
								tooltip_signals.hide();
								que.lock.set(Vec::new()); // invalidate the current timeout
								let _ = env.save();

								create_lock_timeout(timeout_que_id, password, que, env.clone());
							},
						),
						icon_button(
							IconButton {
								icon: String::from(revert_icon),
								tooltip: String::from("Reset"),
								tooltip_signals,
								..IconButton::default()
							},
							move |_| {
								timeout.set(convert_timeout_2_pct(timeout_backup.get()));
								timeout_sec.set(timeout_backup.get());
								tooltip_signals.hide();
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
					label(|| "Download").style(|s| s.margin_left(5)),
					svg(move || String::from(download_icon))
						.style(|s| s.width(16).height(16).margin_left(5)),
				))
				.style(styles::button)
				.style(|s| s.items_center())
				.on_click_cont(|_| {
					open_file(
						FileDialogOptions::new()
							.select_directories()
							.title("Save backup file")
							.button_text("Save"),
						move |file_info| {
							if let Some(mut file) = file_info {
								file.path.push("backup.vault");
								println!("{:?}", file.path);
								// TODO: save file to location
							}
						},
					);
				}),
			)
			.style(|s| s.margin_top(20)),
			label(|| "Importing data").style(|s| s.margin_top(20)),
			v_stack((
				file_input(&|x| {
					println!("{:?}", x);
					// TODO: import data, check ids, ask for password etc
				}),
				password_field(import_password, "Enter password for import file")
					.style(|s| s.margin(3)),
			))
			.style(|s| s.margin_top(20)),
		))
		.style(styles::settings_line),
	)
}
