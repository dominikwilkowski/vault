use std::{panic::catch_unwind, time::Duration};

use floem::{
	action::exec_after,
	animate::animation,
	reactive::{create_rw_signal, RwSignal},
	style::{FlexDirection, Position},
	views::{container, dyn_stack, empty, scroll, svg, Decorators},
	IntoView,
};

use crate::ui::{colors::*, primitives::que::Que};

const DISMISS_TIMEOUT: u64 = 10;

#[derive(Debug, Copy, Clone)]
pub struct ToastSignals {
	pub toasts: RwSignal<Vec<(u8, String)>>,
	pub que: Que,
}

impl ToastSignals {
	pub fn new(que: Que) -> Self {
		Self {
			toasts: create_rw_signal(Vec::new()),
			que,
		}
	}

	pub fn unque_toast(self, id: u8) {
		self.que.toast.update(|item| item.retain(|ids| *ids != id));
	}

	pub fn kill_all_toasts(self) {
		self.toasts.set(Vec::new());
		self.que.unque_all_toasts();
	}

	pub fn add(self, text: String) -> u8 {
		let id = self.que.toast.get().last().unwrap_or(&0) + 1;
		self.toasts.update(|item| item.push((id, text.clone())));
		self.que.toast.update(|item| item.push(id));

		exec_after(Duration::from_secs(DISMISS_TIMEOUT), move |_| {
			if self.que.toast.get().contains(&id) {
				self.unque_toast(id);
				// make sure we don't execute toasts after a view has been destroyed (window closed)
				let _ = catch_unwind(|| {
					self.toasts.update(|item| item.retain(|(ids, _)| *ids != id));
				});
			}
		});

		id
	}
}

pub fn toast_view(toast_signals: ToastSignals) -> impl IntoView {
	let alert_icon = include_str!("../icons/alert.svg");

	scroll(
		dyn_stack(
			move || toast_signals.toasts.get(),
			move |toasts| toasts.clone(),
			move |toast| {
				(
					container(
						svg(move || String::from(alert_icon))
							.style(|s| s.width(16).height(16)),
					)
					.style(|s| {
						s.background(C_ERROR.with_alpha_factor(0.2))
							.border_radius(3)
							.width(35)
							.height_full()
							.items_center()
							.justify_center()
					}),
					(
						toast.1.clone().style(|s| s.width_full().height_full().padding(5)),
						empty()
							.style(|s| {
								s.width(0).height(2).background(C_ERROR.with_alpha_factor(0.7))
							})
							.animation(
								animation()
									.width(|| 200.0 - 35.0 + 3.0)
									.ease_in_out()
									.duration(Duration::from_secs(DISMISS_TIMEOUT)),
							),
					)
						.style(|s| {
							s.flex_col()
								.color(C_TOOLTIP_TEXT)
								.background(C_MAIN_BG)
								.width(200 - 35)
								.position(Position::Relative)
								.inset_left(-3)
						}),
				)
					.style(|s| {
						s.width(200)
							.border_radius(3)
							.background(C_MAIN_BG)
							.box_shadow_blur(4)
							.box_shadow_color(C_SHADOW_1)
							.box_shadow_spread(0)
					})
			},
		)
		.style(move |s| {
			s.flex_direction(FlexDirection::Column).column_gap(5).margin(10)
		}),
	)
	.style(|s| {
		s.position(Position::Absolute)
			.inset_right(7.5)
			.inset_bottom(0)
			.z_index(11)
			.max_height_pct(85.0)
	})
}
