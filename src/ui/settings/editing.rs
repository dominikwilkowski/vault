use floem::views::{container, label, Container, Decorators};

use crate::config::Config;

pub fn editing_view(_config: Config) -> Container {
	container(label(|| "Editing")).style(|s| s)
}
