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

	let upload_icon = include_str!("../icons/upload.svg");

	h_stack((
		label(move || title.get()).style(|s| s.text_ellipsis().width(173)),
		svg(move || String::from(upload_icon)).style(|s| s.width(16).height(16)),
	))
	.on_click_cont(move |_| {
		open_file(
			FileDialogOptions::new().show_hidden().title("Select import file"),
			move |file_info| {
				if let Some(file) = file_info {
					let file_name = file.path[0]
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
	.style(|s| s.width(200).items_center().padding_left(5))
}
