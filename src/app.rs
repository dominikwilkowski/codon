use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

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

#[cfg(feature = "ssr")]
pub mod ssr {
	// use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
	use leptos::server_fn::ServerFnError;
	use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

	pub async fn db() -> Result<Pool<Postgres>, ServerFnError> {
		Ok(
			PgPoolOptions::new()
				.connect("postgres://admin:codon-admin@127.0.0.1:5432/codon")
				.await?,
		)
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Sample {
	id: i16,
	sample_type: String,
	analyst: String,
}

#[server]
pub async fn get_samples() -> Result<Vec<Sample>, ServerFnError> {
	use self::ssr::*;
	use http::request::Parts;

	// this is just an example of how to access server context injected in the handlers
	let req_parts = use_context::<Parts>();

	if let Some(req_parts) = req_parts {
		println!("Uri = {:?}", req_parts.uri);
	}

	use futures::TryStreamExt;

	let conn = db().await?;

	let mut samples = Vec::new();
	let mut rows =
		sqlx::query_as::<_, Sample>("SELECT * FROM samples").fetch(&conn);
	while let Some(row) = rows.try_next().await? {
		samples.push(row);
	}

	// Lines below show how to set status code and headers on the response
	// let resp = expect_context::<ResponseOptions>();
	// resp.set_status(StatusCode::IM_A_TEAPOT);
	// resp.insert_header(SET_COOKIE, HeaderValue::from_str("fizz=buzz").unwrap());

	Ok(samples)
}
