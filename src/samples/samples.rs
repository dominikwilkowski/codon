use crate::{
	components::qr_scanner::qr_scanner::QrScanner, error_template::ErrorTemplate,
};

use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::{MultipartData, MultipartFormData};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

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
		move || {
			(
				add_sample.version().get(),
				edit_sample.version().get(),
				delete_sample.version().get(),
			)
		},
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
				<input
					type="text"
					name="sample_type"
					prop:value=sample_type_value
				/>
				<input type="text" name="analyst" prop:value=analyst_value />
				<button
					type="submit"
					prop:disabled=move || add_sample.pending().get()
				>
					Add
				</button>
			</ActionForm>
			<hr />
			<FileUpload />
			<hr />
			<QrScanner />
			<hr />
			<Transition fallback=move || view! { <p>"Loading samples..."</p> }>
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
																<SampleItem
																	id=sample.id
																	sample_type=sample.sample_type
																	analyst=sample.analyst
																	delete_sample=delete_sample.clone()
																	edit_sample=edit_sample.clone()
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
						<input
							type="text"
							name="sample_type"
							value=sample_type_edit
							node_ref=input_ref
						/>
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

#[component]
pub fn FileUpload() -> impl IntoView {
	#[server(input = MultipartFormData)]
	pub async fn save_file(data: MultipartData) -> Result<String, ServerFnError> {
		use std::path::Path;
		use tokio::{fs::File, io::AsyncWriteExt};
		let mut data = data.into_inner().unwrap();

		while let Ok(Some(mut field)) = data.next_field().await {
			let original_name = field.file_name().unwrap_or("unknown").to_string();
			let new_name = format!("new_{}", original_name);
			let file_path = Path::new("upload_media").join(&new_name);

			// Create the directory if it doesn't exist
			if let Some(parent) = file_path.parent() {
				tokio::fs::create_dir_all(parent).await?;
			}

			// Save the file
			let mut file = File::create(file_path).await?;
			while let Ok(Some(chunk)) = field.chunk().await {
				file.write_all(&chunk).await?;
			}

			return Ok(new_name);
		}

		Err(ServerFnError::ServerError("Failed to save file".into()))
	}

	let upload_action = create_action(|data: &FormData| {
		let data = data.clone();
		// `MultipartData` implements `From<FormData>`
		save_file(data.into())
	});

	view! {
		<form on:submit=move |ev: SubmitEvent| {
			ev.prevent_default();
			let target = ev
				.target()
				.unwrap()
				.unchecked_into::<HtmlFormElement>();
			let form_data = FormData::new_with_form(&target).unwrap();
			upload_action.dispatch(form_data);
		}>
			<h3>File Upload</h3>
			<input
				type="file"
				name="file_to_upload"
				accept="image/*"
				capture="environment"
			/>
			<button type="submit">Upload</button>
		</form>
		<p>
			{move || {
				if upload_action.input().get().is_none()
					&& upload_action.value().get().is_none()
				{
					"Upload a file.".to_string()
				} else if upload_action.pending().get() {
					"Uploading...".to_string()
				} else if let Some(Ok(value)) = upload_action.value().get() {
					format!("Finished uploading \"{}\"", value)
				} else {
					format!("Error: {:?}", upload_action.value().get())
				}
			}}
		</p>
	}
}

#[server]
pub async fn get_samples() -> Result<Vec<SampleData>, ServerFnError> {
	use crate::db::ssr::get_db;

	sqlx::query!("SELECT * FROM samples ORDER BY id")
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

#[server]
pub async fn add_sample(
	sample_type: String,
	analyst: String,
) -> Result<(), ServerFnError> {
	use crate::db::ssr::get_db;

	// fake API delay
	std::thread::sleep(std::time::Duration::from_millis(1250));

	Ok(
		sqlx::query!(
			"INSERT INTO samples (sample_type, analyst) VALUES ($1, $2)",
			sample_type,
			analyst
		)
		.execute(get_db())
		.await
		.map(|_| ())?,
	)
}

#[server]
pub async fn edit_sample(
	id: i32,
	sample_type: String,
	analyst: String,
) -> Result<(), ServerFnError> {
	use crate::db::ssr::get_db;

	Ok(
		sqlx::query!(
			"UPDATE samples SET sample_type = $1,analyst = $2 WHERE id = $3",
			sample_type,
			analyst,
			id
		)
		.execute(get_db())
		.await
		.map(|_| ())?,
	)
}

#[server]
pub async fn delete_sample(id: i32) -> Result<(), ServerFnError> {
	use crate::db::ssr::get_db;

	Ok(
		sqlx::query!("DELETE FROM samples WHERE id = $1", id)
			.execute(get_db())
			.await
			.map(|_| ())?,
	)
}
