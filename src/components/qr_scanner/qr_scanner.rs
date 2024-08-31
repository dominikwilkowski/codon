use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;

stylance::import_style!(css, "qr_scanner.module.css");

#[wasm_bindgen]
extern "C" {
	fn start_qr_scanner(video_element: HtmlVideoElement);
}

#[component]
pub fn QrScanner() -> impl IntoView {
	let show_video = create_rw_signal(false);

	view! {
		<script src="js/qr-scanner.umd.min.js"></script>
		<script src="js/qr_scanner_wrapper.js"></script>
		<div
			id="qr-video-container"
			class=move || {
				let mut classes = css::video.to_string();
				if show_video.get() {
					classes
						.push_str(&format!(" {}", css::video_visible.to_string()));
				}
				classes
			}
		>
			<video id="qr-video"></video>
		</div>
		<button on:click=move |_| {
			show_video.set(true);
			wasm_bindgen_futures::spawn_local(async {
				let video_element = web_sys::window()
					.unwrap()
					.document()
					.unwrap()
					.get_element_by_id("qr-video")
					.unwrap()
					.dyn_into::<HtmlVideoElement>()
					.unwrap();
				start_qr_scanner(video_element);
			});
		}>Start QR Scanner</button>
	}
}
