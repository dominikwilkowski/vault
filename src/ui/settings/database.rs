use floem::views::{container, label, v_stack, Container, Decorators};

use crate::{config::Config, ui::primitives::styles};

pub fn database_view(_config: Config) -> Container {
	container(
		v_stack((label(|| "Auto lock after"), label(|| "enter seconds")))
			.style(styles::settings_line),
	)
}
