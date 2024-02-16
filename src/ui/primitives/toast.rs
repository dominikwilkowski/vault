use std::{panic::catch_unwind, time::Duration};

use floem::{
	action::exec_after,
	reactive::{create_rw_signal, RwSignal},
	style::{FlexDirection, Position},
	view::View,
	views::{dyn_stack, label, Decorators},
};

use crate::{
	ui::{colors::*, primitives::window_metrics::WindowMetrics},
	Que,
};

#[derive(Debug, Copy, Clone)]
pub struct ToastSignals {
	pub toasts: RwSignal<Vec<String>>,
	pub window_metrics: WindowMetrics,
	pub que: Que,
}

impl ToastSignals {
	pub fn new(que: Que, window_metrics: WindowMetrics) -> Self {
		Self {
			toasts: create_rw_signal(Vec::new()),
			window_metrics,
			que,
		}
	}

	pub fn unque_toast(self, id: u8) {
		self.que.toast.update(|item| item.retain(|ids| *ids != id));
	}

	pub fn unque_all_toasts(self) {
		self.que.toast.set(Vec::new());
	}

	pub fn add(self, text: String) -> u8 {
		self.toasts.update(|item| item.push(text.clone()));
		let index = self.toasts.get().len() - 1;
		let id = self.que.toast.get().last().unwrap_or(&0) + 1;
		self.que.toast.update(|item| item.push(id));

		exec_after(Duration::from_secs_f64(10.0), move |_| {
			if self.que.toast.get().contains(&id) {
				self.unque_toast(id);
				// make sure we don't execute toasts after a view has been destroyed (window closed)
				let _ = catch_unwind(|| {
					println!("removal");
					// self.toasts.update(|item| { item.remove(index);});
				});
			}
		});

		id
	}
}

pub fn toast_view(toast_signals: ToastSignals) -> impl View {
	dyn_stack(
		move || toast_signals.toasts.get(),
		move |toasts| toasts.clone(),
		move |toast| {
			label(move || toast.clone()).style(|s| {
				s.width(180)
					.background(C_BG_TOOLTIP)
					.color(C_TEXT_TOOLTIP)
					.padding(5)
					.box_shadow_blur(8)
					.box_shadow_color(C_SHADOW_2)
					.box_shadow_spread(-3)
					.border_color(C_BG_MAIN_BORDER)
					.border(1)
			})
		},
	)
	.style(move |s| {
		s.position(Position::Absolute)
			.flex_direction(FlexDirection::Column)
			.gap(0, 5)
			.z_index(11)
			.inset_right(0)
			.inset_bottom(0)
			.margin(10)
	})
}
