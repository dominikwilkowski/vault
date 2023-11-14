use floem::{
	menu::{Menu, MenuItem},
	views::Decorators,
};

mod ui {
	pub mod app_view;
	pub mod colors;
}

mod db {
	pub mod db;
}

use crate::ui::app_view::app_view;

fn main() {
	floem::launch(|| {
		app_view().window_menu(|| {
			Menu::new("").entry(MenuItem::new("Menu item")).entry(MenuItem::new("Menu item with something on the\tright"))
			// menus are currently commented out in the floem codebase
		})
	});
}
