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
				<Show when=move || !media9.get().is_empty()>
					<FileInput name="media10" value=media10 />
				</Show>
				<Show when=move || !media8.get().is_empty()>
					<FileInput name="media9" value=media9 />
				</Show>
				<Show when=move || !media7.get().is_empty()>
					<FileInput name="media8" value=media8 />
				</Show>
				<Show when=move || !media6.get().is_empty()>
					<FileInput name="media7" value=media7 />
				</Show>
				<Show when=move || !media5.get().is_empty()>
					<FileInput name="media6" value=media6 />
				</Show>
				<Show when=move || !media4.get().is_empty()>
					<FileInput name="media5" value=media5 />
				</Show>
				<Show when=move || !media3.get().is_empty()>
					<FileInput name="media4" value=media4 />
				</Show>
				<Show when=move || !media2.get().is_empty()>
					<FileInput name="media3" value=media3 />
				</Show>
				<Show when=move || !media1.get().is_empty()>
					<FileInput name="media2" value=media2 />
				</Show>
				<FileInput name="media1" value=media1 />
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
		auth::get_user,
		components::file_upload::file_upload,
		permission::{Permission, Permissions},
		utils::{get_equipment_base_folder, get_equipment_notes_folder, move_file},
	};

	use sqlx::PgPool;

	let pool = use_context::<PgPool>().expect("Database not initialized");
	let user = get_user().await?;

	match user {
		Some(user) => {
			let Permissions::All {
				read: _,
				write: _,
				create: perm,
			} = user.permission_equipment;
			if perm != Permission::Create(true) {
				return Err(ServerFnError::Request(String::from("User not authenticated")));
			}
		},
		None => return Err(ServerFnError::Request(String::from("User not authenticated"))),
	};

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

	let media1 = move_file(result.media1, &notes_folder).await?;
	let media2 = move_file(result.media2, &notes_folder).await?;
	let media3 = move_file(result.media3, &notes_folder).await?;
	let media4 = move_file(result.media4, &notes_folder).await?;
	let media5 = move_file(result.media5, &notes_folder).await?;
	let media6 = move_file(result.media6, &notes_folder).await?;
	let media7 = move_file(result.media7, &notes_folder).await?;
	let media8 = move_file(result.media8, &notes_folder).await?;
	let media9 = move_file(result.media9, &notes_folder).await?;
	let media10 = move_file(result.media10, &notes_folder).await?;

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
