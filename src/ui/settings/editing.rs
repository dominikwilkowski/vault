use floem::views::{container, label, v_stack, Container, Decorators};

use crate::{config::Config, ui::primitives::styles};

pub fn editing_view(_config: Config) -> Container {
	container(
		v_stack((label(|| "Default fields"), label(|| "Labels and add button")))
			.style(styles::settings_line),
	)
}
