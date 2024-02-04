use floem::action::exec_after;
use parking_lot::RwLock;
use std::{sync::Arc, time::Duration};

#[derive(Debug, Clone)]
pub struct Debounce<T> {
	pub duration: Duration,
	pub data: Arc<RwLock<Vec<T>>>,
}

impl<T> Default for Debounce<T> {
	fn default() -> Self {
		Debounce {
			duration: Duration::from_millis(100),
			data: Arc::new(RwLock::new(vec![])),
		}
	}
}

impl<T: std::fmt::Debug + PartialEq + Clone + 'static> Debounce<T> {
	pub fn add_value(self, value: T, action: impl Fn() + 'static) {
		let value_fn = value.clone();
		{
			self.data.write().push(value);
		}

		exec_after(self.duration, move |_| {
			if self.data.read().ends_with(&[value_fn.clone()]) {
				// If the value that was sent is not the same as the top of the vec then it is a
				// nop. This might "double save" on occasion, depending on the data.
				action();
			}
		});
	}
}
