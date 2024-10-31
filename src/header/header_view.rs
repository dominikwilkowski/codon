use crate::{icons::Logo, nav::Nav};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "header.module.css");

#[component]
pub fn Header() -> impl IntoView {
	view! {
		<header class=css::header>
			<A href="/" class=css::logo_wrapper>
				<Logo/>
				<span class=css::logo_word>Codon</span>
			</A>
			<Nav/>
		</header>
	}
}
