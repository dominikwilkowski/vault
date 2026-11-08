//! Focus requests made from inside pointer handlers are overridden by floem's
//! default pointer-down behaviour, which focuses the element that was clicked.
//! Deferring the request by a tick lets it win, and also makes requests issued
//! while a view is still being built land once the view is in the window.

use std::time::Duration;

use floem::{
	action::exec_after,
	event::{Event, EventCx, RouteKind},
	ui_events::keyboard::{Code, Key, KeyboardEvent, NamedKey},
	ViewId,
};

/// Focus `id` once the current event has been handled and, for text inputs,
/// move the caret to the end of the content (floem starts at the beginning).
pub fn focus_later(id: ViewId) {
	exec_after(Duration::from_millis(1), move |_| {
		if !id.is_valid() {
			return;
		}
		id.request_focus();
		exec_after(Duration::from_millis(1), move |_| {
			if !id.is_valid() {
				return;
			}
			let end = KeyboardEvent::key_down(Key::Named(NamedKey::End), Code::End);
			id.route_event(Event::Key(end), RouteKind::target(id.get_element_id()));
		});
	});
}

/// floem re-dispatches shortcut-like keys (Escape, Tab, F-keys and modifier
/// combos) that nothing on the focus path consumed to every view with a key
/// listener, focused or not. Key handlers of fields use this to act only on
/// keys that were pressed while they had the focus.
pub fn has_focus(cx: &EventCx) -> bool {
	cx.window_state.is_focus_within(cx.target)
}

/// Pin where a view comes in Tab traversal. floem walks views with an explicit
/// order first and everything else in reading order (top to bottom, left to
/// right), which would put the top bar buttons before the search field.
pub fn tab_order(id: ViewId, order: i32) {
	let element = id.get_element_id();
	if let Some(mut meta) = id.focus_nav_meta_for_element(element) {
		meta.order = Some(order);
		id.set_focus_nav_meta_for_element(element, meta);
	}
}
