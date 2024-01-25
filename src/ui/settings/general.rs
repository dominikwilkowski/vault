use floem::{
	reactive::create_rw_signal,
	views::{container, h_stack, label, v_stack, Container, Decorators},
	widgets::toggle_button,
};

use crate::{
	config::Config,
	ui::{colors::*, primitives::styles},
};

pub fn general_view(config: Config) -> Container {
	let debug_settings_slot = if std::env::var("DEBUG").is_ok() {
		let is_encrypted = create_rw_signal(config.config_db.read().encrypted);

		container(v_stack((
			label(|| "Debug settings")
				.style(|s| s.inset_top(-5).margin_bottom(5).color(C_BG_MAIN_BORDER)),
			h_stack((
				label(move || {
					if is_encrypted.get() {
						"Disable encryption:"
					} else {
						"Enable encryption:"
					}
				}),
				container(
					toggle_button(move || is_encrypted.get())
						.on_toggle(move |_| {
							let new_state = !is_encrypted.get();
							config.config_db.write().encrypted = new_state;
							let _ = config.save();
							is_encrypted.set(new_state);
						})
						.style(styles::toggle_button),
				),
			))
			.style(styles::settings_line),
		)))
		.style(move |s| {
			s.border_top(1).border_color(C_BG_MAIN_BORDER).padding_top(5)
		})
		.style(|s| s.margin_top(20).width_full())
	} else {
		container(label(|| ""))
	};

	container(
		v_stack((
			v_stack((
				label(|| "Change password"),
				container(label(|| "old / new / new")),
			))
			.style(styles::settings_line),
			debug_settings_slot,
		))
		.style(|s| s.width_full()),
	)
	.style(|s| s.width_full())
}
