#[cfg(windows)]
extern crate embed_resource;

#[cfg(windows)]
fn main() {
	println!("cargo:rerun-if-changed=./assets/assets.rc");
	println!("cargo:rerun-if-changed=./assets/favicon.ico");
	embed_resource::compile("./assets/assets.rc", embed_resource::NONE);
}

#[cfg(not(windows))]
fn main() {}
