use crate::components::{button::Button, file_input::FileInput, input::TextArea};
#[cfg(feature = "ssr")]
use crate::{db::ssr::get_db, equipment::EquipmentType};

use leptos::*;
#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};
use server_fn::codec::{MultipartData, MultipartFormData};
#[cfg(feature = "ssr")]
use sqlx::FromRow;
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

#[cfg(feature = "ssr")]
pub async fn get_folder(id: i32) -> Result<String, ServerFnError> {
	#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
	struct EquipmentSQLIDType {
		id: i32,
		equipment_type: String,
	}

	let equipment_sql_data =
		sqlx::query_as::<_, EquipmentSQLIDType>("SELECT id, equipment_type FROM equipment WHERE id = $1")
			.bind(id)
			.fetch_one(get_db())
			.await
			.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	Ok(create_folder_name(EquipmentType::parse(equipment_sql_data.equipment_type), equipment_sql_data.id))
}

#[cfg(feature = "ssr")]
pub fn create_folder_name(equipment_type: EquipmentType, id: i32) -> String {
	let category = match equipment_type {
		EquipmentType::Flask => "F",
		EquipmentType::Vessel => "V",
		EquipmentType::IncubationCabinet => "I",
	};

	format!("{category}-{}/", id)
}

#[server(input = MultipartFormData)]
pub async fn save_notes(data: MultipartData) -> Result<(), ServerFnError> {
	use crate::{components::file_upload::file_upload, db::ssr::get_db, utils::string_to_option};

	let result = file_upload(data, get_folder).await?;

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

	sqlx::query!(
		r#"INSERT INTO equipment_notes
		(equipment, person, notes, media1, media2, media3, media4, media5, media6, media7, media8, media9, media10)
		VALUES
		($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)"#,
		result.id,
		person,
		notes,
		string_to_option(result.media1.clone()),
		string_to_option(result.media2.clone()),
		string_to_option(result.media3.clone()),
		string_to_option(result.media4.clone()),
		string_to_option(result.media5.clone()),
		string_to_option(result.media6.clone()),
		string_to_option(result.media7.clone()),
		string_to_option(result.media8.clone()),
		string_to_option(result.media9.clone()),
		string_to_option(result.media10.clone()),
	)
	.execute(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	Ok(())
}
