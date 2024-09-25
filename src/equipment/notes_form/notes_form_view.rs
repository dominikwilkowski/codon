use crate::components::{
	button::Button, file_input::FileInput, input::TextArea,
};

use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use web_sys::{FormData, SubmitEvent};

stylance::import_style!(css, "notes_form.module.css");

#[component]
pub fn NotesForm(id: String) -> impl IntoView {
	let upload_action =
		create_action(|data: &FormData| save_notes(data.clone().into()));

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
				upload_action.dispatch(form_data);
			}
		>
			<h3>New Note</h3>
			<input type="hidden" name="id" value=id />
			<TextArea
				name="note"
				value=create_rw_signal(String::from(""))
				placeholder="Your note"
			/>
			<div class=css::file_inputs>
				<span>
					<FileInput name="media1" value=media1 />
				</span>
				<span class=move || {
					if media1.get().is_empty() { "is_hidden" } else { "" }
				}>
					<FileInput name="media2" value=media2 />
				</span>
				<span class=move || {
					if media2.get().is_empty() { "is_hidden" } else { "" }
				}>
					<FileInput name="media3" value=media3 />
				</span>
				<span class=move || {
					if media3.get().is_empty() { "is_hidden" } else { "" }
				}>
					<FileInput name="media4" value=media4 />
				</span>
				<span class=move || {
					if media4.get().is_empty() { "is_hidden" } else { "" }
				}>
					<FileInput name="media5" value=media5 />
				</span>
				<span class=move || {
					if media5.get().is_empty() { "is_hidden" } else { "" }
				}>
					<FileInput name="media6" value=media6 />
				</span>
				<span class=move || {
					if media6.get().is_empty() { "is_hidden" } else { "" }
				}>
					<FileInput name="media7" value=media7 />
				</span>
				<span class=move || {
					if media7.get().is_empty() { "is_hidden" } else { "" }
				}>
					<FileInput name="media8" value=media8 />
				</span>
				<span class=move || {
					if media8.get().is_empty() { "is_hidden" } else { "" }
				}>
					<FileInput name="media9" value=media9 />
				</span>
				<span class=move || {
					if media9.get().is_empty() { "is_hidden" } else { "" }
				}>
					<FileInput name="media10" value=media10 />
				</span>
			</div>
			<div class=css::btn_line>
				<Button loading>Upload</Button>
				<span>
					{move || {
						if upload_action.input().get().is_none()
							&& upload_action.value().get().is_none()
						{
							String::from("")
						} else if upload_action.pending().get() {
							loading.set(true);
							String::from("")
						} else if let Some(Ok(files)) = upload_action.value().get()
						{
							loading.set(false);
							format!("Finished uploading: {:?}", files)
						} else {
							loading.set(false);
							format!("Error: {:?}", upload_action.value().get())
						}
					}}
				</span>
			</div>
		</form>
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

	std::thread::sleep(std::time::Duration::from_millis(3000));

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
