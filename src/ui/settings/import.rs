use floem::{
	// reactive::create_rw_signal,
	// style::{CursorStyle, Display, Foreground},
	views::{container, label, v_stack, Container, Decorators},
};

use crate::{
	config::Config,
	ui::{
		// colors::*,
		primitives::{file_input::file_input, styles, tooltip::TooltipSignals},
	},
};

pub fn import_view(
	_tooltip_signals: TooltipSignals,
	_config: Config,
) -> Container {
	container(
		v_stack((
			label(|| "Importing data"),
			v_stack((file_input(&|x| {
				println!("{:?}", x);
			}),)),
		))
		.style(|s| s.margin_bottom(120))
		.style(styles::settings_line),
	)
}
