#[cfg(feature = "ssr")]
use leptos::*;
#[cfg(feature = "ssr")]
use server_fn::codec::MultipartData;
#[cfg(feature = "ssr")]
use std::{
	fs,
	path::{Path, PathBuf},
};
#[cfg(feature = "ssr")]
use tokio::fs::rename;

#[cfg(feature = "ssr")]
async fn move_temp_files(
	temp_files: &mut Vec<(PathBuf, String)>,
	uploaded_files: &mut Vec<String>,
	base_path: &String,
) -> Result<(), ServerFnError> {
	let upload_root = std::env::var("UPLOAD_ROOT").unwrap();
	while let Some((temp_path, file_name)) = temp_files.pop() {
		let new_path = PathBuf::from(format!("{upload_root}/public{base_path}{file_name}"));
		rename(temp_path, new_path.clone()).await?;
		uploaded_files.push(
			new_path
				.to_string_lossy()
				.strip_prefix(&format!("{upload_root}/public"))
				.unwrap_or(new_path.to_string_lossy().as_ref())
				.to_string(),
		);
	}
	Ok(())
}

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub struct FileUploadResult {
	pub id: i32,
	pub media1: String,
	pub media2: String,
	pub media3: String,
	pub media4: String,
	pub media5: String,
	pub media6: String,
	pub media7: String,
	pub media8: String,
	pub media9: String,
	pub media10: String,
	pub additional_fields: Vec<(String, String)>,
}

#[cfg(feature = "ssr")]
pub async fn file_upload(
	data: MultipartData,
	get_folder: impl Fn(i32) -> String,
) -> Result<FileUploadResult, ServerFnError> {
	use tokio::{fs::File, io::AsyncWriteExt};
	use uuid::Uuid;

	let mut folder_name: Option<String> = None;
	let mut equipment_id: Option<i32> = None;
	let mut data = data.into_inner().unwrap();
	let mut uploaded_files = Vec::new();
	let mut files_to_upload = 0;
	let mut temp_files: Vec<(PathBuf, String)> = Vec::new();
	let mut additional_fields: Vec<(String, String)> = Vec::new();
	let upload_root = std::env::var("UPLOAD_ROOT").unwrap();

	while let Ok(Some(mut field)) = data.next_field().await {
		if let Some(name) = field.name() {
			match name {
				"id" => {
					let id = field.text().await?;
					let id = match id.parse::<i32>() {
						Ok(value) => value,
						Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
					};
					equipment_id = Some(id);
					folder_name = Some(get_folder(id));

					tokio::fs::create_dir_all(format!("{upload_root}public{}", folder_name.clone().unwrap())).await?;
					move_temp_files(&mut temp_files, &mut uploaded_files, &folder_name.clone().unwrap()).await?;
				},
				"media1" | "media2" | "media3" | "media4" | "media5" | "media6" | "media7" | "media8" | "media9"
				| "media10" => {
					let first_chunk = field.chunk().await?;
					if let Some(chunk) = first_chunk {
						if !chunk.is_empty() {
							files_to_upload += 1;
							let og_file_name = field.file_name().unwrap_or_default().to_string();
							let extension = Path::new(&og_file_name).extension().and_then(|ext| ext.to_str()).unwrap_or_default();
							let name = format!("{}.{extension}", &Uuid::new_v4().to_string());

							let file_path = if let Some(ref folder_name) = folder_name {
								move_temp_files(&mut temp_files, &mut uploaded_files, folder_name).await?;

								let final_path = PathBuf::from(format!("{upload_root}public{folder_name}")).join(&name);
								uploaded_files.push(format!("{}", PathBuf::from(folder_name).join(&name).to_string_lossy()));
								final_path
							} else {
								// ID has not been processed yet so we store the files in a temp folder until it is
								let temp_path = PathBuf::from("{upload_root}public/upload_media/temp/").join(&name);
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
				other => {
					let name = String::from(other);
					let value = field.text().await?;
					additional_fields.push((name, value));
				},
			}
		}
	}

	if folder_name.is_none() || equipment_id.is_none() {
		Err(ServerFnError::ServerError(String::from("ID not provided")))
	} else if uploaded_files.len() != files_to_upload {
		Err(ServerFnError::ServerError(String::from("Failed to save file")))
	} else {
		let mut iter = uploaded_files.into_iter();

		Ok(FileUploadResult {
			id: equipment_id.unwrap(),
			media1: iter.next().unwrap_or_default(),
			media2: iter.next().unwrap_or_default(),
			media3: iter.next().unwrap_or_default(),
			media4: iter.next().unwrap_or_default(),
			media5: iter.next().unwrap_or_default(),
			media6: iter.next().unwrap_or_default(),
			media7: iter.next().unwrap_or_default(),
			media8: iter.next().unwrap_or_default(),
			media9: iter.next().unwrap_or_default(),
			media10: iter.next().unwrap_or_default(),
			additional_fields,
		})
	}
}

#[cfg(feature = "ssr")]
pub async fn remove_temp_files(result: FileUploadResult) -> Result<(), ServerFnError> {
	let upload_root = std::env::var("UPLOAD_ROOT").unwrap();
	let media_fields = [
		if result.media1.is_empty() {
			None
		} else {
			Some(result.media1)
		},
		if result.media2.is_empty() {
			None
		} else {
			Some(result.media2)
		},
		if result.media3.is_empty() {
			None
		} else {
			Some(result.media3)
		},
		if result.media4.is_empty() {
			None
		} else {
			Some(result.media4)
		},
		if result.media5.is_empty() {
			None
		} else {
			Some(result.media5)
		},
		if result.media6.is_empty() {
			None
		} else {
			Some(result.media6)
		},
		if result.media7.is_empty() {
			None
		} else {
			Some(result.media7)
		},
		if result.media8.is_empty() {
			None
		} else {
			Some(result.media8)
		},
		if result.media9.is_empty() {
			None
		} else {
			Some(result.media9)
		},
		if result.media10.is_empty() {
			None
		} else {
			Some(result.media10)
		},
	];

	for media in media_fields.into_iter().flatten() {
		let file_path = PathBuf::from(format!("{upload_root}/public{media}"));
		if file_path.exists() {
			match fs::remove_file(&file_path) {
				Ok(_) => {},
				Err(_) => return Err(ServerFnError::Request(format!("Could not delete temp file {file_path:?}"))),
			}
		}
	}

	Ok(())
}
