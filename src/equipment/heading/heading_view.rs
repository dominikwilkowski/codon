use crate::components::qr_scanner::QRScanner;

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "heading.module.css");

#[component]
pub fn Heading(children: Children, #[prop(optional)] hide_new: bool) -> impl IntoView {
	view! {
		<div class=css::heading_wrapper>
			<h1 class=css::heading>{children()}</h1>
			<div class=css::heading_btns>
				<Show when=move || !hide_new>
					<A href="/equipment/add/">
						<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
							<path d="M13 7h-2v4H7v2h4v4h2v-4h4v-2h-4z" />
							<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm0 18a8 8 0 1 1 0-16 8 8 0 0 1 0 16z" />
						</svg>
						Add new
					</A>
				</Show>
				<QRScanner />
			</div>
		</div>
	}
}
