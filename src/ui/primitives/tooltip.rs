use floem::{
	action::exec_after,
	reactive::{create_rw_signal, RwSignal},
	style::Position,
	view::View,
	views::{label, Decorators},
};

use std::time::Duration;

use crate::ui::colors::*;

#[derive(Debug, Copy, Clone)]
pub struct TooltipSignals {
	pub tooltip_text: RwSignal<String>,
	pub tooltip_visible: RwSignal<bool>,
	pub tooltip_pos: RwSignal<(f64, f64)>,
	pub tooltip_size: RwSignal<(f64, f64)>,
	pub mouse_pos: RwSignal<(f64, f64)>,
	pub window_size: RwSignal<(f64, f64)>,
}

impl TooltipSignals {
	pub fn new() -> Self {
		Self {
			tooltip_text: create_rw_signal(String::from("")),
			tooltip_visible: create_rw_signal(false),
			tooltip_pos: create_rw_signal((0.0, 0.0)),
			tooltip_size: create_rw_signal((0.0, 0.0)),
			mouse_pos: create_rw_signal((0.0, 0.0)),
			window_size: create_rw_signal((0.0, 0.0)),
		}
	}

	pub fn show(self, text: String) {
		self.tooltip_text.set(text.clone());
		exec_after(Duration::from_secs_f64(0.6), move |_| {
			if self.tooltip_text.get() == text {
				let pos = self.mouse_pos.get();
				let x = if (pos.0 + 13.0 + self.tooltip_size.get().0)
					> self.window_size.get().0
				{
					self.window_size.get().0 - self.tooltip_size.get().0 - 5.0
				} else {
					pos.0 + 13.0
				};

				let y = if self.window_size.get().1 > pos.1 + 33.0 {
					pos.1 + 13.0
				} else {
					pos.1 - 23.0
				};
				self.tooltip_pos.set((x, y));
				self.tooltip_visible.set(true);
			}
		});
	}

	pub fn hide(&self) {
		self.tooltip_text.set(String::from(""));
		self.tooltip_visible.set(false);
	}
}

pub fn tooltip_view(tooltip_signals: TooltipSignals) -> impl View {
	label(move || tooltip_signals.tooltip_text.get())
		.style(move |s| {
			s.position(Position::Absolute)
				.z_index(11)
				.inset_left(tooltip_signals.tooltip_pos.get().0)
				.inset_top(tooltip_signals.tooltip_pos.get().1)
				.apply_if(!tooltip_signals.tooltip_visible.get(), |s| {
					s.inset_left(-50).inset_top(-50)
				})
				.background(C_BG_TOOLTIP)
				.color(C_TEXT_TOOLTIP)
				.padding(3.0)
				.padding_bottom(4.0)
				.padding_left(4.0)
				.padding_right(4.0)
				.border_radius(3)
				.box_shadow_blur(8)
				.box_shadow_color(C_SHADOW_2)
				.box_shadow_spread(-3)
				.border_color(C_BORDER_TOOLTIP)
				.border(1)
		})
		.on_resize(move |rect| {
			let width = rect.x1 - rect.x0;
			let height = rect.y1 - rect.y0;
			tooltip_signals.tooltip_size.set((width, height));
		})
}
