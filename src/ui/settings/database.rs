use floem::views::{container, label, v_stack, Container, Decorators};

use crate::{
	config::Config,
	ui::primitives::{styles, tooltip::TooltipSignals},
};

pub fn database_view(
	_tooltip_signals: TooltipSignals,
	_config: Config,
) -> Container {
	container(
		v_stack((label(|| "Auto lock after"), label(|| "enter seconds")))
			.style(styles::settings_line),
	)
}
