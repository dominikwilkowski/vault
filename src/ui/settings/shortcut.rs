use floem::{
	reactive::use_context,
	view::View,
	// style::{CursorStyle, Display, Foreground},
	views::{container, h_stack, label, Decorators},
};

use crate::{
	env::Environment,
	ui::{
		// colors::*,
		primitives::{styles, tooltip::TooltipSignals},
	},
};

pub fn shortcut_view() -> impl View {
	let _tooltip_signals: TooltipSignals =
		use_context().expect("No context provider");
	let _env: Environment = use_context().expect("No context provider");

	// TODO: add shortcut settings
	container(
		h_stack((label(|| "Shortcut settings"), label(|| "TODO")))
			.style(|s| s.margin_bottom(120))
			.style(styles::settings_line),
	)
}
