use floem::views::{container, label, Container, Decorators};

use crate::config::Config;

pub fn general_view(_config: Config) -> Container {
	container(label(|| "General")).style(|s| s)
}
