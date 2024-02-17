use floem::reactive::{create_rw_signal, RwSignal};

#[derive(Debug, Copy, Clone)]
pub struct Que {
	pub tooltip: RwSignal<Vec<u8>>,
	pub toast: RwSignal<Vec<u8>>,
	pub lock: RwSignal<Vec<u8>>,
}

impl Default for Que {
	fn default() -> Self {
		Self {
			tooltip: create_rw_signal(Vec::new()),
			toast: create_rw_signal(Vec::new()),
			lock: create_rw_signal(Vec::new()),
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
