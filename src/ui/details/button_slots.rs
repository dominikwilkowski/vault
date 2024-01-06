use floem::{
	event::EventListener,
	kurbo::Size,
	reactive::{create_rw_signal, RwSignal},
	view::View,
	views::{container, h_stack, label, Decorators},
	Clipboard, EventPropagation,
};

use crate::{
	config::Config,
	db::DbFields,
	ui::{
		details::detail_view::SECRET_PLACEHOLDER,
		history_view::history_view,
		primitives::{button::icon_button, tooltip::TooltipSignals},
		window_management::{
			closing_window, make_field_path, opening_window, WindowSpec,
		},
	},
};

pub fn view_button_slot(
	is_secret: bool,
	tooltip_signals: TooltipSignals,
	value: RwSignal<String>,
	getter: impl Fn() -> String + 'static,
) -> impl View {
	let see_btn_visible = create_rw_signal(true);
	let hide_btn_visible = create_rw_signal(false);

	let see_icon = include_str!("../icons/see.svg");
	let hide_icon = include_str!("../icons/hide.svg");

	if is_secret {
		h_stack((
			icon_button(String::from(see_icon), see_btn_visible, move |_| {
				let data = getter();
				value.set(data);
				see_btn_visible.set(false);
				hide_btn_visible.set(true);
				tooltip_signals.hide();
			})
			.on_event(EventListener::PointerEnter, move |_event| {
				if is_secret {
					tooltip_signals.show(String::from("See contents of field"));
				}
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				if is_secret {
					tooltip_signals.hide();
				}
				EventPropagation::Continue
			}),
			icon_button(String::from(hide_icon), hide_btn_visible, move |_| {
				value.set(String::from(SECRET_PLACEHOLDER));
				see_btn_visible.set(true);
				hide_btn_visible.set(false);
				tooltip_signals.hide();
			})
			.on_event(EventListener::PointerEnter, move |_event| {
				if is_secret {
					tooltip_signals.show(String::from("Hide contents of field"));
				}
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				if is_secret {
					tooltip_signals.hide();
				}
				EventPropagation::Continue
			}),
		))
	} else {
		h_stack((label(|| ""),))
	}
}

pub fn clipboard_button_slot(
	tooltip_signals: TooltipSignals,
	getter: impl Fn() -> String + 'static,
) -> impl View {
	let clipboard_icon = include_str!("../icons/clipboard.svg");

	container(icon_button(
		String::from(clipboard_icon),
		create_rw_signal(true),
		move |_| {
			let data = getter();
			let _ = Clipboard::set_contents(data);
		},
	))
	.on_event(EventListener::PointerEnter, move |_event| {
		tooltip_signals.show(String::from("Copy to clipboard"));
		EventPropagation::Continue
	})
	.on_event(EventListener::PointerLeave, move |_| {
		tooltip_signals.hide();
		EventPropagation::Continue
	})
}

pub fn history_button_slot(
	id: usize,
	field: DbFields,
	is_secret: bool,
	field_title: String,
	tooltip_signals: TooltipSignals,
	config: Config,
) -> impl View {
	let history_icon = include_str!("../icons/history.svg");
	let hide_history_icon = include_str!("../icons/hide_history.svg");

	let history_btn_visible = create_rw_signal(true);
	let hide_history_btn_visible = create_rw_signal(false);

	if is_secret {
		let config_history = config.clone();
		h_stack((
			icon_button(String::from(history_icon), history_btn_visible, move |_| {
				let config_history_inner = config_history.clone();
				tooltip_signals.hide();
				let window_title = format!("{} Field History", field_title);

				opening_window(
					move || {
						let dates = config_history_inner
							.db
							.read()
							.unwrap()
							.get_history_dates(&id, &field);
						history_btn_visible.set(false);
						hide_history_btn_visible.set(true);

						history_view(id, field, dates, config_history_inner.clone())
					},
					WindowSpec {
						id: make_field_path(id, &field),
						title: window_title,
					},
					Size::new(350.0, 300.0),
					move || {
						history_btn_visible.set(true);
						hide_history_btn_visible.set(false);
					},
				);
			})
			.on_event(EventListener::PointerEnter, move |_event| {
				if is_secret {
					tooltip_signals.show(String::from("See history of field"));
				}
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				if is_secret {
					tooltip_signals.hide();
				}
				EventPropagation::Continue
			}),
			icon_button(
				String::from(hide_history_icon),
				hide_history_btn_visible,
				move |_| {
					closing_window(make_field_path(id, &field), || {
						history_btn_visible.set(true);
						hide_history_btn_visible.set(false);
					});
				},
			)
			.on_event(EventListener::PointerEnter, move |_event| {
				if is_secret {
					tooltip_signals.show(String::from("Hide history of field"));
				}
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				if is_secret {
					tooltip_signals.hide();
				}
				EventPropagation::Continue
			}),
		))
	} else {
		h_stack((label(|| ""),))
	}
}

pub fn delete_button_slot(
	is_dyn_field: bool,
	tooltip_signals: TooltipSignals,
	_config: Config,
) -> impl View {
	let delete_icon = include_str!("../icons/delete.svg");

	if is_dyn_field {
		container(
			icon_button(
				String::from(delete_icon),
				create_rw_signal(true),
				move |_| {
					tooltip_signals.hide();
					// TODO: confirm and delete this field
				},
			)
			.on_event(EventListener::PointerEnter, move |_event| {
				tooltip_signals.show(String::from("Delete this field"));
				EventPropagation::Continue
			})
			.on_event(EventListener::PointerLeave, move |_| {
				tooltip_signals.hide();
				EventPropagation::Continue
			}),
		)
	} else {
		container(label(|| ""))
	}
}
