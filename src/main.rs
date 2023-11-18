use floem::{
	menu::{Menu, MenuItem},
	views::Decorators,
	// window::WindowConfig,
	// kurbo::Size,
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

	// Window size can't be set yet due to the WindowConfig fields being set to private right now
	// floem::new_window(move |_| app_view(), Some(WindowConfig {
	// 	size: Some(Size {
	// 		width: 750.0,
	// 		height: 350.0,
	// 	}),
	// 	..Default::default()
	// }));
}
