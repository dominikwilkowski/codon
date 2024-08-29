use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;

#[wasm_bindgen]
extern "C" {
	fn start_qr_scanner(video_element: HtmlVideoElement);
}

#[component]
pub fn QrScanner() -> impl IntoView {
	view! {
		<script src="js/qr-scanner.umd.min.js"></script>
		<script src="js/qr_scanner_wrapper.js"></script>
		<div id="qr-video-container">
			<video id="qr-video"></video>
		</div>
		<button on:click=move |_| {
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
		}>{"Start QR Scanner"}</button>
	}
}
