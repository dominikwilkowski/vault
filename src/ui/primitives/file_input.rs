use floem::{
	action::open_file,
	file::{FileDialogOptions, FileInfo},
	reactive::create_rw_signal,
	view::View,
	views::{h_stack, label, svg, Decorators},
};

use crate::ui::primitives::styles;

pub fn file_input(on_file: &'static dyn Fn(FileInfo)) -> impl View {
	let title = create_rw_signal(String::from("Select file..."));

	let file_icon = include_str!("../icons/file.svg");

	h_stack((
		label(move || title.get()).style(|s| s.text_ellipsis().width(165)),
		svg(move || String::from(file_icon)).style(|s| s.width(21).height(21)),
	))
	.on_click_cont(move |_| {
		open_file(
			FileDialogOptions::new()
				.show_hidden()
				.title("Select import file")
				.button_text("Import"),
			move |file_info| {
				if let Some(file) = file_info {
					let file_name = file
						.path
						.file_name()
						.and_then(|name| name.to_str())
						.unwrap_or_default();
					title.set(String::from(file_name));
					on_file(file);
				}
			},
		);
	})
	.style(styles::button)
	.style(|s| s.width(200).items_center().padding_left(5).padding_right(5))
}
