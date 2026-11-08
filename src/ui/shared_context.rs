//! App-wide values (environment, queues, signals) that must be reachable from
//! every window, event handler and timer.
//!
//! Floem scopes its reactive context per window and per view tree, which does
//! not cover values that are looked up from event handlers or from other
//! windows, so these are kept in a plain, type-keyed registry instead.

use std::{
	any::{Any, TypeId},
	cell::RefCell,
	collections::HashMap,
};

thread_local! {
	static SHARED: RefCell<HashMap<TypeId, Box<dyn Any>>> = RefCell::new(HashMap::new());
}

/// Store (or replace) the shared value of type `T`.
pub fn provide<T: Clone + 'static>(value: T) {
	SHARED.with(|shared| {
		shared.borrow_mut().insert(TypeId::of::<T>(), Box::new(value));
	});
}

/// Fetch a clone of the shared value of type `T`.
pub fn get<T: Clone + 'static>() -> Option<T> {
	SHARED.with(|shared| {
		shared
			.borrow()
			.get(&TypeId::of::<T>())
			.and_then(|value| value.downcast_ref::<T>().cloned())
	})
}
