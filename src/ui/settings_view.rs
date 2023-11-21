use floem::{
	event::EventListener,
	// reactive::{create_rw_signal, create_signal},
	// style::{AlignContent, AlignItems, CursorStyle, Display, Position},
	view::View,
	views::{label, Decorators},
};

// use crate::ui::colors::*;

pub fn settings_view() -> impl View {
	let view = label(|| "Settings").style(|s| s.width_full().height_full());

	match std::env::var("DEBUG") {
		Ok(_) => {
			// for debugging the layout
			let id = view.id();
			view.on_event_stop(EventListener::KeyUp, move |e| {
				if let floem::event::Event::KeyUp(e) = e {
					if e.key.logical_key == floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11) {
						id.inspect();
					}
				}
			})
		}
		Err(_) => view,
	}
}
