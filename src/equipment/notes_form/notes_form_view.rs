use crate::components::{button::Button, file_input::FileInput, input::TextArea};

use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use web_sys::{FormData, SubmitEvent};

stylance::import_style!(css, "notes_form.module.css");

#[component]
pub fn NotesForm(id: String, notes_upload_action: Action<FormData, Result<(), ServerFnError>>) -> impl IntoView {
	let form_ref = create_node_ref::<html::Form>();

	let media1 = create_rw_signal(String::new());
	let media2 = create_rw_signal(String::new());
	let media3 = create_rw_signal(String::new());
	let media4 = create_rw_signal(String::new());
	let media5 = create_rw_signal(String::new());
	let media6 = create_rw_signal(String::new());
	let media7 = create_rw_signal(String::new());
	let media8 = create_rw_signal(String::new());
	let media9 = create_rw_signal(String::new());
	let media10 = create_rw_signal(String::new());
	let loading = create_rw_signal(false);

	view! {
		<form
			ref=form_ref
			class=css::form
			method="post"
			action="#"
			enctype="multipart/form-data"
			on:submit=move |event: SubmitEvent| {
				event.prevent_default();
				let form = form_ref.get().unwrap();
				let form_data = match FormData::new_with_form(&form) {
					Ok(fd) => fd,
					Err(error) => {
						logging::log!("Failed to create FormData");
						logging::log!("{error:?}");
						return;
					}
				};
				notes_upload_action.dispatch(form_data);
			}
		>
			<h3>Add a Note</h3>
			<input type="hidden" name="id" value=id />
			<input type="hidden" name="person" value=12 />
			<TextArea name="notes" value=create_rw_signal(String::from("")) placeholder="Your note" required=true />
			<div class=css::file_inputs>
				<FileInput name="media1" value=media1 />
				<Show when=move || !media1.get().is_empty()>
					<FileInput name="media2" value=media2 />
				</Show>
				<Show when=move || !media2.get().is_empty()>
					<FileInput name="media3" value=media3 />
				</Show>
				<Show when=move || !media3.get().is_empty()>
					<FileInput name="media4" value=media4 />
				</Show>
				<Show when=move || !media4.get().is_empty()>
					<FileInput name="media5" value=media5 />
				</Show>
				<Show when=move || !media5.get().is_empty()>
					<FileInput name="media6" value=media6 />
				</Show>
				<Show when=move || !media6.get().is_empty()>
					<FileInput name="media7" value=media7 />
				</Show>
				<Show when=move || !media7.get().is_empty()>
					<FileInput name="media8" value=media8 />
				</Show>
				<Show when=move || !media8.get().is_empty()>
					<FileInput name="media9" value=media9 />
				</Show>
				<Show when=move || !media9.get().is_empty()>
					<FileInput name="media10" value=media10 />
				</Show>
			</div>
			<div class=css::btn_line>
				<Button kind="submit" loading>
					Save
				</Button>
				<span>
					{move || {
						if notes_upload_action.input().get().is_none() && notes_upload_action.value().get().is_none() {
							view! {}.into_view()
						} else if notes_upload_action.pending().get() {
							loading.set(true);
							view! {}.into_view()
						} else if let Some(Ok(_)) = notes_upload_action.value().get() {
							loading.set(false);
							view! { <span class=css::success>Saved successfully</span> }.into_view()
						} else {
							loading.set(false);
							view! {
								<span class=css::error>
									{format!("Error: {:?}", notes_upload_action.value().get())}
								</span>
							}
								.into_view()
						}
					}}
				</span>
			</div>
		</form>
	}
}

#[server(input = MultipartFormData, prefix = "/api")]
pub async fn save_notes(data: MultipartData) -> Result<(), ServerFnError> {
	use crate::{
		components::file_upload::file_upload,
		utils::{get_equipment_base_folder, get_equipment_notes_folder},
	};

	use sqlx::PgPool;
	use tokio::fs::rename;

	let pool = use_context::<PgPool>().expect("Database not initialized");

	let result = file_upload(data, |id| format!("{}temp/", get_equipment_base_folder(id))).await?;

	let mut person = None;
	let mut notes = None;

	for (name, value) in &result.additional_fields {
		match name.as_str() {
			"person" => {
				person = {
					let value = match value.parse::<i32>() {
						Ok(value) => value,
						Err(_) => return Err(ServerFnError::Request(String::from("Invalid person ID"))),
					};
					Some(value)
				}
			},
			"notes" => notes = Some(value),
			_ => {},
		}
	}

	let note = sqlx::query!(
		r#"INSERT INTO equipment_notes (equipment, person, notes) VALUES ($1, $2, $3) RETURNING id"#,
		result.id,
		person,
		notes,
	)
	.fetch_one(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	let notes_folder = get_equipment_notes_folder(note.id);

	let media1 = if result.media1.is_empty() {
		None
	} else {
		let new_path = result.media1.replace("temp/", &notes_folder);
		rename(format!("public{}", result.media1), format!("public{new_path}")).await?;
		Some(new_path)
	};

	let media2 = if result.media2.is_empty() {
		None
	} else {
		let new_path = result.media2.replace("temp/", &notes_folder);
		rename(format!("public{}", result.media2), format!("public{new_path}")).await?;
		Some(new_path)
	};

	let media3 = if result.media3.is_empty() {
		None
	} else {
		let new_path = result.media3.replace("temp/", &notes_folder);
		rename(format!("public{}", result.media3), format!("public{new_path}")).await?;
		Some(new_path)
	};

	let media4 = if result.media4.is_empty() {
		None
	} else {
		let new_path = result.media4.replace("temp/", &notes_folder);
		rename(format!("public{}", result.media4), format!("public{new_path}")).await?;
		Some(new_path)
	};

	let media5 = if result.media5.is_empty() {
		None
	} else {
		let new_path = result.media5.replace("temp/", &notes_folder);
		rename(format!("public{}", result.media5), format!("public{new_path}")).await?;
		Some(new_path)
	};

	let media6 = if result.media6.is_empty() {
		None
	} else {
		let new_path = result.media6.replace("temp/", &notes_folder);
		rename(format!("public{}", result.media6), format!("public{new_path}")).await?;
		Some(new_path)
	};

	let media7 = if result.media7.is_empty() {
		None
	} else {
		let new_path = result.media7.replace("temp/", &notes_folder);
		rename(format!("public{}", result.media7), format!("public{new_path}")).await?;
		Some(new_path)
	};

	let media8 = if result.media8.is_empty() {
		None
	} else {
		let new_path = result.media8.replace("temp/", &notes_folder);
		rename(format!("public{}", result.media8), format!("public{new_path}")).await?;
		Some(new_path)
	};

	let media9 = if result.media9.is_empty() {
		None
	} else {
		let new_path = result.media9.replace("temp/", &notes_folder);
		rename(format!("public{}", result.media9), format!("public{new_path}")).await?;
		Some(new_path)
	};

	let media10 = if result.media10.is_empty() {
		None
	} else {
		let new_path = result.media10.replace("temp/", &notes_folder);
		rename(format!("public{}", result.media10), format!("public{new_path}")).await?;
		Some(new_path)
	};

	sqlx::query!(
		r#"UPDATE equipment_notes set
			media1 = $1,
			media2 = $2,
			media3 = $3,
			media4 = $4,
			media5 = $5,
			media6 = $6,
			media7 = $7,
			media8 = $8,
			media9 = $9,
			media10 = $10
		WHERE id = $11"#,
		media1,
		media2,
		media3,
		media4,
		media5,
		media6,
		media7,
		media8,
		media9,
		media10,
		note.id,
	)
	.execute(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	Ok(())
}
