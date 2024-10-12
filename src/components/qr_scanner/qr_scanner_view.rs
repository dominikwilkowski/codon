use crate::components::button::{Button, ButtonVariant};

use leptos::*;
use leptos_qr_scanner::Scan;
use leptos_router::*;

stylance::import_style!(css, "qr_scanner.module.css");

#[component]
pub fn QRScanner() -> impl IntoView {
	let navigate = use_navigate();

	let show_scanner = create_rw_signal(false);

	view! {
		<Button variant=ButtonVariant::Text on_click=move |_| show_scanner.update(|x| *x = !*x)>
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
				<path d="M4 4h4V2H2v6h2V4zm0 12H2v6h6v-2H4v-4zm16 4h-4v2h6v-6h-2v4zM16 4h4v4h2V2h-6v2z" />
				<path d="M5 11h6V5H5zm2-4h2v2H7zM5 19h6v-6H5zm2-4h2v2H7zM19 5h-6v6h6zm-2 4h-2V7h2zm-4 4h2v2h-2zm2 2h2v2h-2zm2 2h2v2h-2zm0-4h2v2h-2z" />
			</svg>
			Scan QR-Code
		</Button>
		<button
			class=move || {
				if show_scanner.get() {
					format!("{} {}", css::btn_backdrop, css::is_open)
				} else {
					css::btn_backdrop.to_string()
				}
			}
			on:click=move |_| show_scanner.set(false)
		/>
		<div class=move || {
			if show_scanner.get() {
				format!("{} {}", css::qr_scanner, css::is_open)
			} else {
				css::qr_scanner.to_string()
			}
		}>
			<Button variant=ButtonVariant::Outlined on:click=move |_| show_scanner.set(false)>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
					<path d="m16.192 6.344-4.243 4.242-4.242-4.242-1.414 1.414L10.535 12l-4.242 4.242 1.414 1.414 4.242-4.242 4.243 4.242 1.414-1.414L13.364 12l4.242-4.242z" />
				</svg>
			</Button>
			<Scan
				active=show_scanner
				on_scan=move |scanned_url| {
					if scanned_url.contains("/equipment/") {
						if let Some(id_bit) = scanned_url.rsplit("/equipment/").next() {
							show_scanner.set(false);
							navigate(&format!("/equipment/{id_bit}"), Default::default());
						}
					}
				}
				class=css::scan
				video_class=css::video
			/>
		</div>
	}
}
