use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<Stylesheet id="leptos" href="/pkg/codon.css"/>
		<Title text="Welcome to Codon"/>

		// content for this welcome page
		<Router fallback=|| {
				let mut outside_errors = Errors::default();
				outside_errors.insert_with_default_key(AppError::NotFound);
				view! { <ErrorTemplate outside_errors/> }.into_view()
		}>
			<main>
				<Routes>
					<Route path="" view=HomePage/>
				</Routes>
			</main>
		</Router>
	}
}

#[component]
fn HomePage() -> impl IntoView {
	let count = create_rw_signal(0);
	let on_click = move |_| count.update(|count| *count += 1);

	view! {
		<h1>"Welcome to Codon"</h1>
		<button on:click=on_click>"Click Me: " {count}</button>
	}
}
