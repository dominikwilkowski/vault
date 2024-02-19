use floem::{
	action::open_file,
	file::{FileDialogOptions, FileInfo},
	reactive::{create_effect, create_rw_signal, untrack, RwSignal},
	view::View,
	views::{h_stack, label, svg, Decorators},
};

use crate::ui::primitives::styles;

pub fn file_input<F>(
	value: RwSignal<Vec<String>>,
	options: FileDialogOptions,
	on_file: F,
) -> impl View
where
	F: Fn(FileInfo) + 'static + Copy,
{
	let title = create_rw_signal(String::from("Select file..."));

	let upload_icon = include_str!("../icons/upload.svg");

	create_effect(move |_| {
		value.track();
		if value.get().is_empty() {
			untrack(|| {
				title.set(String::from("Select file..."));
			});
		}
	});

	h_stack((
		label(move || title.get()).style(|s| s.text_ellipsis().width(173)),
		svg(move || String::from(upload_icon)).style(|s| s.width(16).height(16)),
	))
	.on_click_cont(move |_| {
		open_file(options.clone(), move |file_info: Option<FileInfo>| {
			if let Some(file) = file_info {
				let names = file.path.iter().fold(Vec::new(), |mut name, path| {
					name.push(
						path.file_name().and_then(|name| name.to_str()).unwrap_or_default(),
					);
					name
				});

				let paths =
					file.path.iter().fold(Vec::new(), |mut collection, path| {
						collection.push(path.to_string_lossy().to_string());
						collection
					});

				title.set(names.join(", "));
				value.set(paths);
				on_file(file);
			}
		});
	})
	.style(styles::button)
	.style(|s| s.width_full().items_center().padding_left(5))
}
