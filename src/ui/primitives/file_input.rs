use floem::{
	action::open_file,
	event::listener,
	file::{FileDialogOptions, FileInfo},
	reactive::{Effect, RwSignal, SignalGet, SignalTrack, SignalUpdate},
	views::{svg, Decorators, Label},
	IntoView,
};

use crate::ui::{
	app_view::TooltipSignalsSettings, primitives::styles, shared_context,
};

pub fn file_input<F>(
	value: RwSignal<Vec<String>>,
	input_label: String,
	options: FileDialogOptions,
	on_file: F,
) -> impl IntoView
where
	F: Fn(FileInfo) + 'static + Copy,
{
	let tooltip_signals = shared_context::get::<TooltipSignalsSettings>()
		.expect("No tooltip_signals context provider")
		.inner;

	let title = RwSignal::new(input_label.clone());
	let input_label_effect = input_label.clone();

	let upload_icon = include_str!("../icons/upload.svg");

	Effect::new(move |_| {
		let input_label_effect = input_label_effect.clone();
		value.track();
		if value.get().is_empty() {
			Effect::untrack(move || {
				title.set(input_label_effect);
			});
		}
	});

	(
		Label::derived(move || title.get())
			.style(|s| s.text_ellipsis().flex_grow(1.0_f32).selectable(false)),
		svg(move || String::from(upload_icon)).style(|s| s.width(16).height(16)),
	)
		.style(|s| s.keyboard_navigable())
		.on_event_cont(listener::PointerEnter, move |_, _| {
			if !value.get().is_empty() && value.get()[0] != input_label.clone() {
				tooltip_signals.show(value.get()[0].clone());
			}
		})
		.on_event_cont(listener::PointerLeave, move |_, _| {
			tooltip_signals.hide();
		})
		.on_event_cont(listener::Click, move |_, _| {
			open_file(options.clone(), move |file_info: Option<FileInfo>| {
				if let Some(file) = file_info {
					let names = file.path.iter().fold(Vec::new(), |mut name, path| {
						name.push(
							path
								.file_name()
								.and_then(|name| name.to_str())
								.unwrap_or_default(),
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
