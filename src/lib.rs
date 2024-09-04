pub mod db;
pub mod error_template;
pub mod home {
	pub mod home;
}
pub mod samples {
	pub mod samples;
}
pub mod nav {
	pub mod nav;
}
pub mod app {
	pub mod app;
}
pub mod equipment {
	pub mod equipment;
	pub mod schema;
}
pub mod icons {
	pub mod culture;
	pub mod equipment;
	pub mod experiment;
	pub mod flask;
	pub mod incubation_cabine;
	pub mod people;
	pub mod vessel;
}

#[cfg(feature = "ssr")]
pub mod fileserv;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	use crate::app::app::*;
	console_error_panic_hook::set_once();
	leptos::mount_to_body(App);
}
