use floem::{
	event::EventListener,
	file::{FileDialogOptions, FileInfo},
	file_action::open_file,
	reactive::{create_effect, create_rw_signal, untrack, use_context, RwSignal},
	view::View,
	views::{h_stack, label, svg, Decorators},
};

use crate::ui::{app_view::TooltipSignalsSettings, primitives::styles};

pub fn file_input<F>(
	value: RwSignal<Vec<String>>,
	input_label: String,
	options: FileDialogOptions,
	on_file: F,
) -> impl View
where
	F: Fn(FileInfo) + 'static + Copy,
{
	let tooltip_signals = use_context::<TooltipSignalsSettings>()
		.expect("No tooltip_signals context provider")
		.inner;

	let title = create_rw_signal(input_label.clone());
	let input_label_effect = input_label.clone();

	let upload_icon = include_str!("../icons/upload.svg");

	create_effect(move |_| {
		let input_label_effect = input_label_effect.clone();
		value.track();
		if value.get().is_empty() {
			untrack(move || {
				title.set(input_label_effect);
			});
		}
	});

	h_stack((
		label(move || title.get()).style(|s| s.text_ellipsis().flex_grow(1.0)),
		svg(move || String::from(upload_icon)).style(|s| s.width(16).height(16)),
	))
	.keyboard_navigatable()
	.on_event_cont(EventListener::PointerEnter, move |_| {
		if !value.get().is_empty() && value.get()[0] != input_label.clone() {
			tooltip_signals.show(value.get()[0].clone());
		}
	})
	.on_event_cont(EventListener::PointerLeave, move |_| {
		tooltip_signals.hide();
	})
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
