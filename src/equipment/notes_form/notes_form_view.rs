use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use web_sys::{FormData, SubmitEvent};

stylance::import_style!(css, "notes_form.module.css");

#[component]
pub fn NotesForm(id: String) -> impl IntoView {
	let upload_action =
		create_action(|data: &FormData| save_notes(data.clone().into()));

	let form_ref = create_node_ref::<html::Form>();

	view! {
		<form
			ref=form_ref
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
				upload_action.dispatch(form_data);
			}
		>
			<h3>File Upload</h3>
			<input type="hidden" name="id" value=id />
			<input type="text" name="name" />
			<textarea name="note"></textarea>
			<input
				type="file"
				name="media1"
				accept="image/*;video/*;capture=camera"
				capture="environment"
			/>
			<input
				type="file"
				name="media2"
				accept="image/*;video/*;capture=camera"
				capture="environment"
			/>
			<input
				type="file"
				name="media3"
				accept="image/*;video/*;capture=camera"
				capture="environment"
			/>
			<input
				type="file"
				name="media4"
				accept="image/*;video/*;capture=camera"
				capture="environment"
			/>
			<input
				type="file"
				name="media5"
				accept="image/*;video/*;capture=camera"
				capture="environment"
			/>
			<input
				type="file"
				name="media6"
				accept="image/*;video/*;capture=camera"
				capture="environment"
			/>
			<input
				type="file"
				name="media7"
				accept="image/*;video/*;capture=camera"
				capture="environment"
			/>
			<input
				type="file"
				name="media8"
				accept="image/*;video/*;capture=camera"
				capture="environment"
			/>
			<input
				type="file"
				name="media9"
				accept="image/*;video/*;capture=camera"
				capture="environment"
			/>
			<input
				type="file"
				name="media10"
				accept="image/*;video/*;capture=camera"
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
				} else if let Some(Ok(files)) = upload_action.value().get() {
					format!("Finished uploading: {:?}", files)
				} else {
					format!("Error: {:?}", upload_action.value().get())
				}
			}}
		</p>
	}
}

#[server(input = MultipartFormData)]
pub async fn save_notes(data: MultipartData) -> Result<String, ServerFnError> {
	use crate::{
		components::file_upload::file_upload, db::ssr::get_db,
		equipment::EquipmentType,
	};

	use serde::{Deserialize, Serialize};
	use sqlx::FromRow;

	let result = file_upload(data, |id| async move {
		let id = match id.parse::<i32>() {
			Ok(value) => value,
			Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
		};

		#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
		struct EquipmentSQLIDType {
			id: i32,
			equipment_type: String,
		}

		let equipment_sql_data = sqlx::query_as::<_, EquipmentSQLIDType>(
			"SELECT id, equipment_type FROM equipment WHERE id = $1",
		)
		.bind(id)
		.fetch_one(get_db())
		.await
		.map_err::<ServerFnError, _>(|error| {
			ServerFnError::ServerError(error.to_string())
		})?;

		let category = match EquipmentType::parse(equipment_sql_data.equipment_type)
		{
			EquipmentType::Flask => "F",
			EquipmentType::Vessel => "V",
			EquipmentType::IncubationCabinet => "I",
		};

		Ok(format!("{category}-{}/", equipment_sql_data.id))
	})
	.await?;

	Ok(format!("{result:?}"))
}
