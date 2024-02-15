use floem::reactive::{create_rw_signal, RwSignal};

#[derive(Debug, Copy, Clone)]
pub struct WindowMetrics {
	pub mouse_pos: RwSignal<(f64, f64)>,
	pub window_size: RwSignal<(f64, f64)>,
}

impl Default for WindowMetrics {
	fn default() -> Self {
		Self {
			mouse_pos: create_rw_signal((0.0, 0.0)),
			window_size: create_rw_signal((0.0, 0.0)),
		}
	}
}
