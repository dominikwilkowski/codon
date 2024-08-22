use crate::error_template::ErrorTemplate;

use leptos::*;
use serde::{Deserialize, Serialize};

stylance::import_style!(css, "samples.module.css");

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct SampleData {
	id: i32,
	sample_type: String,
	analyst: String,
}

#[component]
pub fn Samples() -> impl IntoView {
	let samples = create_resource(move || (), move |_| get_samples());

	view! {
		<div class=css::wrapper>
			<h1>Samples</h1>
			<Transition fallback=move || view! {<p>"Loading..."</p> }>
				<ErrorBoundary fallback=|errors| view!{<ErrorTemplate errors=errors/>}>
					{move || {
						let existing_todos = {
							move || {
								samples.get()
									.map(move |todos| match todos {
										Err(e) => {
											view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
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

						view! {
							<ul class=css::outer_ul>
								{existing_todos}
							</ul>
						}
					}
				}
				</ErrorBoundary>
			</Transition>
		</div>
	}
}

#[server]
pub async fn get_samples() -> Result<Vec<SampleData>, ServerFnError> {
	use crate::db::ssr::db;

	use futures::TryStreamExt;
	// use http::request::Parts;
	//
	// this is just an example of how to access server context injected in the handlers
	// let req_parts = use_context::<Parts>();
	//
	// if let Some(req_parts) = req_parts {
	//   println!("Uri = {:?}", req_parts.uri);
	// }

	let conn = db().await?;

	let mut samples = Vec::new();
	let mut rows =
		sqlx::query_as::<_, SampleData>("SELECT * FROM samples").fetch(&conn);
	while let Some(row) = rows.try_next().await? {
		samples.push(row);
	}

	// Lines below show how to set status code and headers on the response
	// let resp = expect_context::<ResponseOptions>();
	// resp.set_status(StatusCode::IM_A_TEAPOT);
	// resp.insert_header(SET_COOKIE, HeaderValue::from_str("fizz=buzz").unwrap());

	Ok(samples)
}
