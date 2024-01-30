use floem::{
	reactive::create_rw_signal,
	style::{CursorStyle, Display, Foreground},
	views::{container, h_stack, label, v_stack, Container, Decorators},
	widgets::slider::{slider, AccentBarClass, BarClass, HandleRadius},
};

use crate::{
	config::Config,
	ui::{
		colors::*,
		primitives::{
			button::{icon_button, IconButton},
			styles,
			tooltip::TooltipSignals,
		},
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

pub fn database_view(
	tooltip_signals: TooltipSignals,
	config: Config,
) -> Container {
	let db_timeout = config.general.read().db_timeout;
	let timeout_backup = create_rw_signal(db_timeout);
	let timeout = create_rw_signal(convert_timeout_2_pct(db_timeout));

	let save_icon = include_str!("../icons/save.svg");
	let revert_icon = include_str!("../icons/revert.svg");
	let snap_icon = include_str!("../icons/snap.svg");

	container(
		v_stack((
			label(|| "Auto lock after"),
			v_stack((
				label(move || human_readable(convert_pct_2_timeout(timeout.get()))),
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
					.on_change_pct(move |val| timeout.set(val)),
				container(
					h_stack((
						icon_button(
							IconButton {
								icon: String::from(save_icon),
								tooltip: String::from("Save to database"),
								tooltip_signals,
								..IconButton::default()
							},
							move |_| {
								let seconds = convert_pct_2_timeout(timeout.get());
								config.general.write().db_timeout = seconds;
								timeout_backup.set(seconds);
								tooltip_signals.hide();
							},
						),
						icon_button(
							IconButton {
								icon: String::from(snap_icon),
								tooltip: String::from("Snap to minute"),
								tooltip_signals,
								..IconButton::default()
							},
							move |_| {
								let seconds = convert_pct_2_timeout(timeout.get());
								timeout
									.set(convert_timeout_2_pct((seconds / 60.0).floor() * 60.0));
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
								tooltip_signals.hide();
							},
						),
					))
					.style(move |s| {
						s.display(Display::Flex).apply_if(
							(convert_pct_2_timeout(timeout.get())
								- timeout_backup.get().abs())
							.abs() < f32::EPSILON,
							|s| s.display(Display::None),
						)
					}),
				)
				.style(|s| s.gap(5, 0).height(30)),
			)),
		))
		.style(styles::settings_line),
	)
}
