use crate::{
	equipment::{Equipment, EquipmentAdd, EquipmentDetail, EquipmentEdit},
	error_template::{AppError, ErrorTemplate},
	home::Home,
	nav::Nav,
	samples::Samples,
};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

stylance::import_style!(css, "app.module.css");

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<Stylesheet id="leptos" href="/pkg/codon.css" />
		<Link
			rel="preload"
			as_="font"
			type_="font/woff2"
			href="/noto_sans_mono_latin.woff2"
			crossorigin=""
		/>
		<Link
			rel="preload"
			as_="font"
			type_="font/woff2"
			href="/noto_sans_mono_latin_ext.woff2"
			crossorigin=""
		/>
		<Title text="Welcome to Codon" />

		<Router
			trailing_slash=TrailingSlash::Redirect
			fallback=|| {
				let mut outside_errors = Errors::default();
				outside_errors.insert_with_default_key(AppError::NotFound);
				view! { <ErrorTemplate outside_errors /> }.into_view()
			}
		>
			<Nav />
			<main class=format!("{} frame", css::main)>
				<Routes>
					<Route path="" view=Home />
					<Route path="/samples" view=Samples />
					<Route path="/equipment" view=Equipment />
					<Route path="/equipment/add" view=EquipmentAdd />
					<Route path="/equipment/:id" view=EquipmentDetail />
					<Route path="/equipment/edit/:id" view=EquipmentEdit />
				</Routes>
			</main>
		</Router>
	}
}
