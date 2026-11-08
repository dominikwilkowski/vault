use floem::reactive::{RwSignal, SignalUpdate};

#[derive(Debug, Copy, Clone)]
pub struct Que {
	pub tooltip: RwSignal<Vec<u8>>,
	pub toast: RwSignal<Vec<u8>>,
	pub lock: RwSignal<Vec<u8>>,
}

impl Default for Que {
	fn default() -> Self {
		Self {
			tooltip: RwSignal::new(Vec::new()),
			toast: RwSignal::new(Vec::new()),
			lock: RwSignal::new(Vec::new()),
		}
	}
}

impl Que {
	pub fn unque_all_tooltips(self) {
		self.tooltip.set(Vec::new());
	}

	pub fn unque_all_toasts(self) {
		self.toast.set(Vec::new());
	}
}
