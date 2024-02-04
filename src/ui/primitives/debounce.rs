use floem::action::exec_after;
use parking_lot::RwLock;
use std::{sync::Arc, time::Duration};

#[derive(Debug, Clone)]
pub struct Debounce {
	duration: Duration,
	counter: Arc<RwLock<u64>>,
}

impl Default for Debounce {
	fn default() -> Self {
		Debounce {
			duration: Duration::from_millis(100),
			counter: Arc::new(RwLock::new(0)),
		}
	}
}

impl Debounce {
	pub fn add(self, action: impl Fn() + 'static) {
		let call_counter: u64;
		{
			let mut counter = self.counter.write();
			*counter += 1;
			call_counter = *counter;
		}
		exec_after(self.duration, move |_| {
			// If the counter set in this call is the same as in Debounce then it is the latest within duration.
			if *self.counter.read() == call_counter {
				// Reset the counter
				*self.counter.write() = 0;
				action();
			}
		});
	}
}

// use floem::action::exec_after;
// use parking_lot::RwLock;
// use std::{sync::Arc, time::Duration};
//
// #[derive(Debug, Clone)]
// pub struct Debounce<T> {
// 	pub duration: Duration,
// 	pub data: Arc<RwLock<Vec<T>>>,
// }
//
// impl<T> Default for crate::ui::primitives::debounce::Debounce<T> {
// 	fn default() -> Self {
// 		crate::ui::primitives::debounce::Debounce {
// 			duration: Duration::from_millis(100),
// 			data: Arc::new(RwLock::new(vec![])),
// 		}
// 	}
// }
//
// impl<T: std::fmt::Debug + PartialEq + Clone + 'static> crate::ui::primitives::debounce::Debounce<T> {
// 	pub fn add_value(self, value: T, action: impl Fn() + 'static) {
// 		let value_fn = value.clone();
// 		{
// 			self.data.write().push(value);
// 		}
//
// 		exec_after(self.duration, move |_| {
// 			if self.data.read().ends_with(&[value_fn.clone()]) {
// 				// If the value that was sent is not the same as the top of the vec then it is a
// 				// nop. This might "double save" on occasion, depending on the data.
// 				action();
// 			}
// 		});
// 	}
// }
