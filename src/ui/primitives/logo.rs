use floem::views::{Decorators, Label};

use crate::ui::colors::FONT_MONOSPACE;

const BANNER: &str = r"
_    _     _     _   _   _      _____  •
\ \ / /   /_\   | | | | | |    |_   _| •
 \ V /   / _ \  | |_| | | |__    | |   •
  \_/   /_/ \_\  \___/  |____|   |_|";

pub fn logo() -> Label {
	Label::derived(|| BANNER.replace('•', ""))
		.style(|s| s.font_family(String::from(FONT_MONOSPACE)).font_size(8.0))
}
