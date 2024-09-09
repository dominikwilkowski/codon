#[macro_use]
pub mod macros;

pub mod app;
pub mod components {
	pub mod button;
	pub mod datepicker;
	pub mod input;
	pub mod pagination;
	pub mod select;
	pub mod table;
}
pub mod db;
pub mod ds;
pub mod equipment;
pub mod error_template;
pub mod home;
pub mod icons;
pub mod nav;
pub mod qrcode;
pub mod samples;

#[cfg(feature = "ssr")]
pub mod fileserv;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	use crate::app::App;
	console_error_panic_hook::set_once();
	leptos::mount_to_body(App);
}
