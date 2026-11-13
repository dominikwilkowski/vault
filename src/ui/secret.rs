//! Helpers for signals that hold secrets.
//!
//! A `String` that is dropped or replaced keeps its bytes in freed memory
//! (GHSA-vjvh-m6q3-jrgc), so secrets are kept in buffers that wipe themselves
//! and are wiped before they are replaced.

use floem::reactive::{RwSignal, SignalUpdate, SignalWith};
use zeroize::{Zeroize, Zeroizing};

/// Enough for any password without growing while typing; growing would leave
/// the old buffer behind unwiped.
const TYPED_SECRET_CAPACITY: usize = 256;

/// A buffer for a typed secret, wiped when it is dropped.
pub fn new_buffer() -> RwSignal<Zeroizing<String>> {
	RwSignal::new(Zeroizing::new(String::with_capacity(TYPED_SECRET_CAPACITY)))
}

/// Replace the value of a signal, wiping the old value first.
pub fn replace(signal: RwSignal<String>, new: String) {
	signal.update(|old| {
		old.zeroize();
		*old = new;
	});
}

/// Wipe the signal's value when the view that owns it is removed (`on_cleanup`).
pub fn wipe_later(signal: RwSignal<String>) -> impl Fn() + 'static {
	move || {
		if signal.try_with_untracked(|value| value.is_some()) {
			signal.update(|value| value.zeroize());
		}
	}
}
