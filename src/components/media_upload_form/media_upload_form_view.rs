use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use web_sys::{FormData, SubmitEvent};

stylance::import_style!(css, "media_upload_form.module.css");

#[component]
pub fn MediaUploadForm(id: String) -> impl IntoView {
	let upload_action =
		create_action(|data: &FormData| save_file(data.clone().into()));

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
pub async fn save_file(
	data: MultipartData,
) -> Result<Vec<String>, ServerFnError> {
	use crate::{
		db::ssr::get_db,
		equipment::{EquipmentData, EquipmentSQLData, EquipmentType},
	};

	use std::path::{Path, PathBuf};
	use tokio::{
		fs::{rename, File},
		io::AsyncWriteExt,
	};
	use uuid::Uuid;

	async fn move_temp_files(
		temp_files: &mut Vec<(PathBuf, String)>,
		uploaded_files: &mut Vec<String>,
		base_path: &String,
	) -> Result<(), ServerFnError> {
		while temp_files.len() > 0 {
			let (temp_path, file_name) = temp_files.pop().unwrap();
			let new_path = Path::new(base_path).join(file_name.clone());
			rename(temp_path, new_path).await?;
			uploaded_files.push(file_name);
		}
		Ok(())
	}

	let mut equipment_path: Option<String> = None;
	let mut data = data.into_inner().unwrap();
	let mut uploaded_files = Vec::new();
	let mut temp_files: Vec<(PathBuf, String)> = Vec::new();

	while let Ok(Some(mut field)) = data.next_field().await {
		if let Some(name) = field.name() {
			match name {
				"id" => {
					let id = field.text().await?;
					let id = match id.parse::<i32>() {
						Ok(value) => value,
						Err(_) => {
							return Err(ServerFnError::Request(String::from("Invalid ID")))
						},
					};

					let equipment_sql_data = sqlx::query_as::<_, EquipmentSQLData>(
						"SELECT * FROM equipment WHERE id = $1",
					)
					.bind(id)
					.fetch_one(get_db())
					.await
					.map_err::<ServerFnError, _>(|error| {
						ServerFnError::ServerError(error.to_string())
					})?;
					let equipment_data: EquipmentData = equipment_sql_data.into();

					let category = match equipment_data.equipment_type {
						EquipmentType::Flask => "F",
						EquipmentType::Vessel => "V",
						EquipmentType::IncubationCabinet => "I",
					};
					equipment_path = Some(format!("{category}-{}/", equipment_data.id));
					tokio::fs::create_dir_all(format!(
						"public/upload_media/{}",
						equipment_path.clone().unwrap()
					))
					.await?;

					move_temp_files(
						&mut temp_files,
						&mut uploaded_files,
						&equipment_path.clone().unwrap(),
					)
					.await?;
				},
				"media1" | "media2" | "media3" => {
					let first_chunk = field.chunk().await?;
					if let Some(chunk) = first_chunk {
						if !chunk.is_empty() {
							let og_file_name =
								field.file_name().unwrap_or_default().to_string();
							let extension = Path::new(&og_file_name)
								.extension()
								.and_then(|ext| ext.to_str())
								.unwrap_or_default();
							let name = format!("{}.{extension}", &Uuid::new_v4().to_string());

							let file_path = if let Some(ref equipment_path) = equipment_path {
								move_temp_files(
									&mut temp_files,
									&mut uploaded_files,
									&equipment_path,
								)
								.await?;

								uploaded_files.push(name.clone());
								PathBuf::from("public/upload_media/")
									.join(&equipment_path)
									.join(&name)
							} else {
								// ID has not been processed yet so we store the files in a temp folder until it is
								let temp_path =
									PathBuf::from("public/upload_media/temp/").join(&name);
								temp_files.push((temp_path.clone(), name));
								temp_path
							};

							if let Some(parent) = file_path.parent() {
								tokio::fs::create_dir_all(parent).await?;
							}

							let mut file = File::create(file_path.clone()).await?;
							file.write_all(&chunk).await?;
							while let Ok(Some(chunk)) = field.chunk().await {
								file.write_all(&chunk).await?;
							}
						} else {
							// empty chunk means the input field had no data in it
							continue;
						}
					} else {
						// no chunks were returned
						continue;
					}
				},
				_ => {
					// fields not accounted for
					continue;
				},
			}
		}
	}

	if equipment_path.is_none() {
		return Err(ServerFnError::ServerError(String::from(
			"Equipment ID not provided",
		)));
	} else if uploaded_files.is_empty() {
		Err(ServerFnError::ServerError(String::from("Failed to save file")))
	} else {
		return Ok(uploaded_files);
	}
}
