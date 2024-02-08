use floem::{
	view::View,
	// reactive::create_rw_signal,
	// style::{CursorStyle, Display, Foreground},
	views::{container, h_stack, label, Decorators},
};

use crate::env::Environment;
use crate::ui::{
	// colors::*,
	primitives::{styles, tooltip::TooltipSignals},
};

pub fn shortcut_view(
	_tooltip_signals: TooltipSignals,
	_env: Environment,
) -> impl View {
	// TODO: add shortcut settings
	container(
		h_stack((label(|| "Shortcut settings"), label(|| "TODO")))
			.style(|s| s.margin_bottom(120))
			.style(styles::settings_line),
	)
}
