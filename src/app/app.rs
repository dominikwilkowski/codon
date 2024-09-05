use crate::{
	equipment::equipment::Equipment,
	equipment::equipment_detail::EquipmentDetail,
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
		<link rel="preconnect" href="https://fonts.googleapis.com" />
		<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
		<link
			href="https://fonts.googleapis.com/css2?family=Noto+Sans+Mono:wght@100..900&display=swap"
			rel="stylesheet"
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
