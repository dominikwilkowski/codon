use crate::error_template::ErrorTemplate;

use leptos::*;
use serde::{Deserialize, Serialize};

stylance::import_style!(css, "samples.module.css");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct SampleData {
	pub id: i32,
	pub sample_type: String,
	pub analyst: String,
}

#[component]
pub fn Samples() -> impl IntoView {
	let samples = create_resource(move || (), move |_| get_samples());

	view! {
		<div class=css::wrapper>
			<h1>Samples</h1>
			<Transition fallback=move || view! { <p>"Loading..."</p> }>
				<ErrorBoundary fallback=|errors| {
						view! { <ErrorTemplate errors=errors /> }
				}>
					{move || {
							let existing_todos = {
									move || {
											samples
													.get()
													.map(move |todos| match todos {
															Err(e) => {
																	view! {
																		<pre class="error">"Server Error: " {e.to_string()}</pre>
																	}
																			.into_view()
															}
															Ok(samples) => {
																	if samples.is_empty() {
																			view! { <p>"No samples were found."</p> }.into_view()
																	} else {
																			samples
																					.into_iter()
																					.map(move |sample| {
																							view! {
																								<li>
																									<ul class=css::inner_ul>
																										<li>{sample.id}</li>
																										<li>{sample.sample_type}</li>
																										<li>{sample.analyst}</li>
																									</ul>
																								</li>
																							}
																					})
																					.collect_view()
																	}
															}
													})
													.unwrap_or_default()
									}
							};
							view! { <ul class=css::outer_ul>{existing_todos}</ul> }
					}}
				</ErrorBoundary>
			</Transition>
		</div>
	}
}

#[server]
pub async fn get_samples() -> Result<Vec<SampleData>, ServerFnError> {
	use crate::db::ssr::get_db;

	sqlx::query!("SELECT * FROM samples")
		.map(|data| SampleData {
			id: data.id,
			sample_type: data
				.sample_type
				.expect("No `sample_type` found in `samples` table"),
			analyst: data.analyst.expect("No `analyst` found in `samples` table"),
		})
		.fetch_all(get_db())
		.await
		.map_err(|error| ServerFnError::ServerError(error.to_string()))
}
