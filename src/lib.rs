#[macro_use]
pub mod macros;

pub mod app;
pub mod components {
	pub mod avatar;
	pub mod button;
	pub mod checkbox;
	pub mod datepicker;
	pub mod dropdown;
	pub mod file_input;
	pub mod file_upload;
	pub mod img_attachment;
	pub mod input;
	pub mod multiline;
	pub mod pagination;
	pub mod qr_scanner;
	pub mod radio;
	pub mod select;
	pub mod switch;
	pub mod timezone_offset;
}
pub mod db;
pub mod ds;
pub mod equipment;
pub mod error_template;
pub mod footer;
pub mod header;
pub mod home;
pub mod icons;
pub mod nav;
pub mod qrcode;
pub mod samples;
pub mod utils;

#[cfg(feature = "ssr")]
pub mod fileserv;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
	use crate::app::App;
	console_error_panic_hook::set_once();
	leptos::mount_to_body(App);
}
