//! Allocator hook that reports secrets left behind in freed memory.
//!
//! GHSA-vjvh-m6q3-jrgc: after locking, the master password and vault entries
//! were still found in a core dump. `Db::lock` wipes the copies it owns, but a
//! copy that is dropped without being wiped keeps its bytes in the allocator's
//! free lists, which is what `strings` on a core dump finds.
//!
//! This allocator wraps the system allocator and, for every block that is
//! freed or reallocated, checks whether one of the watched secrets is still
//! inside it. The tests use it to assert that unlocking, using and locking the
//! database never frees a block that still holds a secret. With the
//! `leakcheck` feature the app does the same for the secrets listed in the
//! environment variable `VAULT_LEAKCHECK_NEEDLES` (comma separated) and prints
//! a summary a few seconds after locking.
//!
//! What this cannot see, and what vault therefore cannot promise to wipe: the
//! OS clipboard after "copy", key events and IME buffers owned by the OS and
//! winit, the GPU glyph cache of text that was shown revealed, the copies the
//! `toml` parser owns while unescaping string values, buffers that `String`
//! growth left behind while typing (the password input pre-reserves to avoid
//! most of it) and values on the stack.

use std::{
	alloc::{GlobalAlloc, Layout, System},
	backtrace::Backtrace,
	cell::Cell,
	io::Write,
	ptr, slice,
	sync::{
		atomic::{AtomicUsize, Ordering},
		RwLock,
	},
};

/// Backtraces are printed for the first hits of each needle only.
const MAX_REPORTS_PER_NEEDLE: usize = 5;

struct Needle {
	bytes: Vec<u8>,
	hits: AtomicUsize,
}

static NEEDLES: RwLock<Vec<Needle>> = RwLock::new(Vec::new());
static HITS: AtomicUsize = AtomicUsize::new(0);

thread_local! {
	// Reporting allocates; nothing allocated while reporting is scanned.
	static IN_HOOK: Cell<bool> = const { Cell::new(false) };
}

/// The allocator, registered with `#[global_allocator]` in `main.rs`.
pub struct LeakCheck;

/// Watch `secret`; returns the index used by [`hits_for`].
pub fn watch(secret: &[u8]) -> usize {
	let mut needles = NEEDLES.write().unwrap_or_else(|e| e.into_inner());
	needles.push(Needle {
		bytes: secret.to_vec(),
		hits: AtomicUsize::new(0),
	});
	needles.len() - 1
}

/// Watch every secret listed in `VAULT_LEAKCHECK_NEEDLES`.
#[cfg(feature = "leakcheck")]
pub fn watch_from_env() {
	if let Ok(list) = std::env::var("VAULT_LEAKCHECK_NEEDLES") {
		// the list itself contains the secrets, so it is wiped like one
		let list = zeroize::Zeroizing::new(list);
		for needle in list.split(',').filter(|needle| !needle.is_empty()) {
			watch(needle.as_bytes());
		}
	}
}

/// Number of freed blocks that still contained any watched secret.
#[cfg_attr(not(test), allow(dead_code))]
pub fn hits() -> usize {
	HITS.load(Ordering::Relaxed)
}

/// Number of freed blocks that still contained the secret with this index.
#[cfg_attr(not(test), allow(dead_code))]
pub fn hits_for(index: usize) -> usize {
	NEEDLES
		.read()
		.map(|needles| {
			needles.get(index).map_or(0, |needle| needle.hits.load(Ordering::Relaxed))
		})
		.unwrap_or(0)
}

/// Print one line per watched secret to stderr.
pub fn report_summary() {
	let Ok(needles) = NEEDLES.read() else {
		return;
	};
	let mut err = std::io::stderr().lock();
	for (index, needle) in needles.iter().enumerate() {
		let _ = writeln!(
			err,
			"leakcheck: needle #{index} ({} bytes) was found in {} freed block(s)",
			needle.bytes.len(),
			needle.hits.load(Ordering::Relaxed)
		);
	}
}

fn report(index: usize, size: usize) {
	let backtrace = Backtrace::force_capture();
	let mut err = std::io::stderr().lock();
	let _ = writeln!(
		err,
		"leakcheck: needle #{index} found in a freed block of {size} bytes at\n{backtrace}"
	);
}

/// Look for every watched secret in the block that is about to be freed.
///
/// # Safety
/// `ptr` must point to `size` readable bytes.
unsafe fn scan(ptr: *const u8, size: usize) {
	// A `true` from `replace` means an outer scan on this thread is reporting.
	if size == 0 || IN_HOOK.try_with(|flag| flag.replace(true)).unwrap_or(true) {
		return;
	}
	if let Ok(needles) = NEEDLES.try_read() {
		let block = unsafe { slice::from_raw_parts(ptr, size) };
		for (index, needle) in needles.iter().enumerate() {
			let found = needle.bytes.len() <= size
				&& block
					.windows(needle.bytes.len())
					.any(|window| window == needle.bytes);
			if found {
				HITS.fetch_add(1, Ordering::Relaxed);
				if needle.hits.fetch_add(1, Ordering::Relaxed) < MAX_REPORTS_PER_NEEDLE
				{
					report(index, size);
				}
			}
		}
	}
	let _ = IN_HOOK.try_with(|flag| flag.set(false));
}

unsafe impl GlobalAlloc for LeakCheck {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		unsafe { System.alloc(layout) }
	}

	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
		unsafe { System.alloc_zeroed(layout) }
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		unsafe {
			scan(ptr, layout.size());
			System.dealloc(ptr, layout);
		}
	}

	unsafe fn realloc(
		&self,
		ptr: *mut u8,
		layout: Layout,
		new_size: usize,
	) -> *mut u8 {
		// Always move so the old block is inspected before it is given back.
		let new_layout =
			unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };
		let new_ptr = unsafe { System.alloc(new_layout) };
		if !new_ptr.is_null() {
			unsafe {
				ptr::copy_nonoverlapping(ptr, new_ptr, layout.size().min(new_size));
				scan(ptr, layout.size());
				System.dealloc(ptr, layout);
			}
		}
		new_ptr
	}
}
