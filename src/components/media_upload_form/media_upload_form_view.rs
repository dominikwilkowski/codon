use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use wasm_bindgen::JsCast;
use web_sys::{console, FormData, HtmlFormElement, SubmitEvent};

stylance::import_style!(css, "media_upload_form.module.css");

#[component]
pub fn MediaUploadForm() -> impl IntoView {
	let upload_action = create_action(|data: &FormData| {
		let data = data.clone();
		save_file(data.into())
	});

	let form_ref = create_node_ref::<HtmlFormElement>();

	view! {
		<form method="post" action="#" enctype="multipart/form-data" ref=form_ref on:submit=move |event: SubmitEvent| {
			event.prevent_default();
			let form = match form_ref.get() {
				Some(form) => {
					console::log_1(&"Successfully obtained form via ref".into());
					form
				},
				None => {
					console::error_1(&"Failed to get form element from ref".into());
					return;
				}
			};

			// Create FormData from the form
			let form_data = match FormData::new_with_form(&form) {
				Ok(fd) => {
					console::log_1(&"FormData created successfully".into());
					fd
				},
				Err(err) => {
					console::error_1(&"Failed to create FormData".into());
					console::error_1(&format!("{:?}", err).into());
					return;
				}
			};

			// let target = event
			// 	.target()
			// 	.unwrap()
			// 	.unchecked_into::<HtmlFormElement>();
			// let form_data = FormData::new_with_form(&target).unwrap();
			upload_action.dispatch(form_data);
		}>
			<h3>File Upload</h3>
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
	use std::path::Path;
	use tokio::{fs::File, io::AsyncWriteExt};

	let mut data = data.into_inner().unwrap();
	let mut uploaded_files = Vec::new();

	while let Ok(Some(mut field)) = data.next_field().await {
		let first_chunk = field.chunk().await?;
		if let Some(chunk) = first_chunk {
			if !chunk.is_empty() {
				let original_name = field.file_name().unwrap_or("unknown").to_string();
				let new_name = format!("new_{}", original_name);
				let file_path = Path::new("public/upload_media/").join(&new_name);

				// Create the directory if it doesn't exist
				if let Some(parent) = file_path.parent() {
					tokio::fs::create_dir_all(parent).await?;
				}

				// Save the file
				let mut file = File::create(file_path).await?;
				file.write_all(&chunk).await?;
				while let Ok(Some(chunk)) = field.chunk().await {
					file.write_all(&chunk).await?;
				}

				uploaded_files.push(new_name);
			} else {
				// empty chunk means the input field had no data in it
				continue;
			}
		} else {
			// no chunk was returned
			continue;
		}
	}

	if uploaded_files.is_empty() {
		Err(ServerFnError::ServerError("Failed to save file".into()))
	} else {
		return Ok(uploaded_files);
	}
}
