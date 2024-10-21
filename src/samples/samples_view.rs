use crate::error_template::ErrorTemplate;

use leptos::*;
use leptos_router::*;
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
	let delete_sample = create_server_action::<DeleteSample>();
	let edit_sample = create_server_multi_action::<EditSample>();
	let add_sample = create_server_action::<AddSample>();
	let samples = create_resource(
		move || (add_sample.version().get(), edit_sample.version().get(), delete_sample.version().get()),
		move |_| get_samples(),
	);

	let (sample_type_value, set_sample_type_value) = create_signal(String::new());
	let (analyst_value, set_analyst_value) = create_signal(String::new());

	create_effect(move |_| {
		match add_sample.value().get() {
			None => {},
			Some(Ok(())) => {
				set_sample_type_value.set(String::new());
				set_analyst_value.set(String::new());
			},
			Some(Err(_)) => {},
		};
	});

	view! {
		<div class=css::wrapper>
			<h1>Samples</h1>
			<ActionForm action=add_sample class=css::add_form>
				<input type="text" name="sample_type" placeholder="type" prop:value=sample_type_value />
				<input type="text" name="analyst" placeholder="analyst" prop:value=analyst_value />
				<button type="submit" prop:disabled=move || add_sample.pending().get()>
					Add
				</button>
			</ActionForm>

			<Transition fallback=move || view! { <p>"Loading samples..."</p> }>
				<ErrorBoundary fallback=|errors| {
					view! { <ErrorTemplate errors /> }
				}>
					{move || {
						let existing_todos = {
							move || {
								samples
									.get()
									.map(move |todos| match todos {
										Err(e) => {
											view! { <pre class="error">"Server Error: " {e.to_string()}</pre> }
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
																<SampleItem
																	id=sample.id
																	sample_type=sample.sample_type
																	analyst=sample.analyst
																	delete_sample
																	edit_sample
																/>
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

#[component]
pub fn SampleItem(
	id: i32,
	sample_type: String,
	analyst: String,
	delete_sample: Action<DeleteSample, Result<(), ServerFnError>>,
	edit_sample: MultiAction<EditSample, Result<(), ServerFnError>>,
) -> impl IntoView {
	let show_edit = create_rw_signal(false);
	let input_ref = create_node_ref();

	view! {
		{move || {
			let sample_type = sample_type.clone();
			let analyst = analyst.clone();
			let sample_type_edit = sample_type.clone();
			let analyst_edit = analyst.clone();
			if show_edit.get() {
				view! {
					<MultiActionForm
						action=edit_sample
						on:submit=move |_| {
							show_edit.set(false);
						}
					>
						<input type="hidden" name="id" value=id />
						<input type="text" name="sample_type" value=sample_type_edit node_ref=input_ref />
						<input type="text" name="analyst" value=analyst_edit />
						<button type="submit">Save</button>
						<button on:click=move |_| {
							show_edit.set(false);
						}>Cancel</button>
					</MultiActionForm>
				}
					.into_view()
			} else {
				view! {
					<ul class=css::inner_ul>
						<li>{id}</li>
						<li>{sample_type}</li>
						<li>{analyst}</li>
						<li>
							<button on:click=move |_| {
								show_edit.set(true);
								if let Some(input) = input_ref.get() {
									input.focus().unwrap();
								}
							}>Edit</button>
						</li>
						<li>
							<ActionForm action=delete_sample>
								<input type="hidden" name="id" value=id />
								<input type="submit" value="Delete" />
							</ActionForm>
						</li>
					</ul>
				}
					.into_view()
			}
		}}
	}
}

#[server(prefix = "/api")]
pub async fn get_samples() -> Result<Vec<SampleData>, ServerFnError> {
	use sqlx::PgPool;

	let pool = use_context::<PgPool>().expect("Database not initialized");

	sqlx::query!("SELECT * FROM samples ORDER BY id")
		.map(|data| SampleData {
			id: data.id,
			sample_type: data.sample_type.expect("No `sample_type` found in `samples` table"),
			analyst: data.analyst.expect("No `analyst` found in `samples` table"),
		})
		.fetch_all(&pool)
		.await
		.map_err(|error| ServerFnError::ServerError(error.to_string()))
}

#[server(prefix = "/api")]
pub async fn add_sample(sample_type: String, analyst: String) -> Result<(), ServerFnError> {
	use sqlx::PgPool;

	let pool = use_context::<PgPool>().expect("Database not initialized");

	// fake API delay
	std::thread::sleep(std::time::Duration::from_millis(1250));

	Ok(
		sqlx::query!("INSERT INTO samples (sample_type, analyst) VALUES ($1, $2)", sample_type, analyst)
			.execute(&pool)
			.await
			.map(|_| ())?,
	)
}

#[server(prefix = "/api")]
pub async fn edit_sample(id: i32, sample_type: String, analyst: String) -> Result<(), ServerFnError> {
	use sqlx::PgPool;

	let pool = use_context::<PgPool>().expect("Database not initialized");

	Ok(
		sqlx::query!("UPDATE samples SET sample_type = $1,analyst = $2 WHERE id = $3", sample_type, analyst, id)
			.execute(&pool)
			.await
			.map(|_| ())?,
	)
}

#[server(prefix = "/api")]
pub async fn delete_sample(id: i32) -> Result<(), ServerFnError> {
	use sqlx::PgPool;

	let pool = use_context::<PgPool>().expect("Database not initialized");

	Ok(sqlx::query!("DELETE FROM samples WHERE id = $1", id).execute(&pool).await.map(|_| ())?)
}
