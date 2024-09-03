use crate::{
	equipment::equipment::{Equipment, EquipmentDetail},
	error_template::{AppError, ErrorTemplate},
	home::home::HomePage,
	nav::nav::Nav,
	samples::samples::Samples,
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
		<Title text="Welcome to Codon" />

		<Router fallback=|| {
			let mut outside_errors = Errors::default();
			outside_errors.insert_with_default_key(AppError::NotFound);
			view! { <ErrorTemplate outside_errors /> }.into_view()
		}>
			<main class=css::main>
				<Nav />
				<Routes>
					<Route path="" view=HomePage />
					<Route path="/samples" view=Samples />
					<Route path="/equipment" view=Equipment />
					<Route path="/equipment/:id" view=EquipmentDetail />
				</Routes>
			</main>
		</Router>
	}
}
