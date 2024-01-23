use floem::views::{container, label, Container, Decorators};

use crate::config::Config;

pub fn database_view(_config: Config) -> Container {
	container(label(|| "Database")).style(|s| s)
}
