use crate::{
	app::{LoginAction, UserSignal},
	components::{
		avatar::Avatar,
		button::{Button, ButtonVariant},
		dropdown::{Dropdown, DropdownItem, DropdownPlacement, DropdownTrigger},
		file_input::FileInput,
		img_attachment::ImgAttachment,
		input::TextArea,
		multiline::MultiLine,
		pagination::Pagination,
	},
	equipment::{EquipmentNotesData, NotesForm, save_notes},
	error_template::ErrorTemplate,
	permission::Permissions,
};

use leptos::*;
use leptos_router::*;
use server_fn::codec::{MultipartData, MultipartFormData};
#[cfg(feature = "ssr")]
use sqlx::FromRow;
use web_sys::{FormData, SubmitEvent};

stylance::import_style!(css, "notes.module.css");

#[component]
pub fn Notes(
	id: RwSignal<String>,
	notes_query_page: RwSignal<u16>,
	notes_query_ipp: RwSignal<u8>,
	log_query_page: RwSignal<u16>,
	log_query_ipp: RwSignal<u8>,
	tab_query: RwSignal<String>,
) -> impl IntoView {
	let params = use_params_map();

	let login_action = use_context::<LoginAction>().expect("No login action found in context");

	let notes_upload_action = create_action(|data: &FormData| save_notes(data.clone().into()));
	let edit_note_action = create_action(|data: &FormData| edit_note(data.clone().into()));
	let delete_note_action = create_server_action::<DeleteNote>();

	let notes_data = create_resource(
		move || {
			(
				params.with(|p| p.get("id").cloned().unwrap_or_default()),
				login_action.version().get(),
				notes_upload_action.version().get(),
				edit_note_action.version().get(),
				delete_note_action.version().get(),
			)
		},
		move |(id, _, _, _, _)| get_notes_for_equipment(id, notes_query_page.get(), notes_query_ipp.get()),
	);

	view! {
		<Suspense fallback=move || view! { <p>Loading notes...</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors=errors /> }
			}>
				{move || {
					if notes_data.get().is_some() {
						match notes_data.get().unwrap() {
							Err(error) => {
								view! { <pre class="error">Notes Server Error: {error.to_string()}</pre> }.into_view()
							}
							Ok((notes, person_id, count)) => {
								let hidden_fields = vec![
									(String::from("log_page"), log_query_page.get().to_string()),
									(String::from("log_items_per_page"), log_query_ipp.get().to_string()),
									(String::from("tab"), tab_query.get()),
								];
								view! {
									<div class=css::notes_list id="equipment_notes">
										<NotesForm
											id=id.get()
											person_id=person_id
											notes_upload_action=notes_upload_action
										/>
										<Pagination
											action=format!("/equipment/{}#equipment_notes", id.get())
											page_key="notes_page"
											ipp_key="notes_items_per_page"
											query_page=notes_query_page
											query_ipp=notes_query_ipp
											row_count=count
											hidden_fields
										/>
										{notes
											.into_iter()
											.map(|note| {
												view! {
													<NotesItem
														note=note
														edit_note_action=edit_note_action
														delete_note_action=delete_note_action
													/>
												}
											})
											.collect_view()}
									</div>
								}
									.into_view()
							}
						}
					} else {
						view! { <div>No Notes found</div> }.into_view()
					}
				}}
			</ErrorBoundary>
		</Suspense>
	}
}

#[component]
pub fn NotesItem(
	note: EquipmentNotesData,
	edit_note_action: Action<FormData, Result<(), ServerFnError>>,
	delete_note_action: Action<DeleteNote, Result<(), ServerFnError>>,
) -> impl IntoView {
	let is_editing = create_rw_signal(false);

	let note_view = note.clone();
	let note_edit = note.clone();

	view! {
		<div class=css::notes_item>
			<Avatar data=note.person />
			<div>
				<Show
					when=move || !is_editing.get()
					fallback=move || {
						view! {
							<NoteEdit note=note_edit.clone() is_editing=is_editing edit_note_action=edit_note_action />
						}
					}
				>
					<Note note=note_view.clone() is_editing=is_editing delete_note_action=delete_note_action />
				</Show>
			</div>
		</div>
	}
}

#[component]
pub fn Note(
	note: EquipmentNotesData,
	is_editing: RwSignal<bool>,
	delete_note_action: Action<DeleteNote, Result<(), ServerFnError>>,
) -> impl IntoView {
	let user_signal = use_context::<UserSignal>().expect("No user signal found in context");

	view! {
		<small>
			{note.create_date.format("%d %b %Y %I:%M:%S %P").to_string()}
			<Suspense fallback=move || {
				view! { <span /> }
			}>
				{move || {
					match user_signal.get() {
						None => view! { <span /> }.into_view(),
						Some(user) => {
							let Permissions::All { read: _, write: perm, create: _ } = user.permission_equipment;
							view! {
								<Show when=move || perm.has_permission("write", -1, note.person.id)>
									<Dropdown
										placement=DropdownPlacement::BottomEnd
										on_select=move |link: String| {
											if link.as_str() == "edit" {
												is_editing.set(true);
											}
										}
									>
										<DropdownTrigger slot>
											<button class=css::dropdown_btn>
												<svg
													class=css::menu
													xmlns="http://www.w3.org/2000/svg"
													viewBox="0 0 24 24"
													fill="currentColor"
												>
													<path d="M12 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm6 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zM6 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z" />
												</svg>
											</button>
										</DropdownTrigger>
										<DropdownItem key="edit" label="Edit" />
										<button
											class=css::text_btn
											on:click=move |_| {
												if web_sys::window()
													.unwrap()
													.confirm_with_message("Are you sure you want to delete this Note?")
													.unwrap_or(false)
												{
													delete_note_action.dispatch(DeleteNote { id: note.id });
												}
											}
										>
											Delete
										</button>
									</Dropdown>
								</Show>
							}
						}
					}
				}}
			</Suspense>
		</small>
		<MultiLine text=note.notes />
		<div class="codon_img_attachment">
			<ImgAttachment file_path=note.media1 />
			<ImgAttachment file_path=note.media2 />
			<ImgAttachment file_path=note.media3 />
			<ImgAttachment file_path=note.media4 />
			<ImgAttachment file_path=note.media5 />
			<ImgAttachment file_path=note.media6 />
			<ImgAttachment file_path=note.media7 />
			<ImgAttachment file_path=note.media8 />
			<ImgAttachment file_path=note.media9 />
			<ImgAttachment file_path=note.media10 />
		</div>
	}
}

#[component]
pub fn NoteEdit(
	note: EquipmentNotesData,
	is_editing: RwSignal<bool>,
	edit_note_action: Action<FormData, Result<(), ServerFnError>>,
) -> impl IntoView {
	let form_ref = create_node_ref::<html::Form>();

	let media1 = create_rw_signal(String::from(""));
	let media2 = create_rw_signal(String::from(""));
	let media3 = create_rw_signal(String::from(""));
	let media4 = create_rw_signal(String::from(""));
	let media5 = create_rw_signal(String::from(""));
	let media6 = create_rw_signal(String::from(""));
	let media7 = create_rw_signal(String::from(""));
	let media8 = create_rw_signal(String::from(""));
	let media9 = create_rw_signal(String::from(""));
	let media10 = create_rw_signal(String::from(""));
	let loading = create_rw_signal(false);
	let empty_fields = create_rw_signal(
		(note.media1.is_none() as usize)
			+ (note.media2.is_none() as usize)
			+ (note.media3.is_none() as usize)
			+ (note.media4.is_none() as usize)
			+ (note.media5.is_none() as usize)
			+ (note.media6.is_none() as usize)
			+ (note.media7.is_none() as usize)
			+ (note.media8.is_none() as usize)
			+ (note.media9.is_none() as usize)
			+ (note.media10.is_none() as usize),
	);

	view! {
		<form
			ref=form_ref
			class=css::edit_form
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
				edit_note_action.dispatch(form_data);
			}
		>
			<input type="hidden" name="id" value=note.equipment />
			<input type="hidden" name="note_id" value=note.id />
			<TextArea value=create_rw_signal(note.notes) name="notes" placeholder="Your note" />
			<div class="codon_img_attachment">
				<MediaRemoveToggle media=note.media1 name="remove_media1" empty_fields=empty_fields />
				<MediaRemoveToggle media=note.media2 name="remove_media2" empty_fields=empty_fields />
				<MediaRemoveToggle media=note.media3 name="remove_media3" empty_fields=empty_fields />
				<MediaRemoveToggle media=note.media4 name="remove_media4" empty_fields=empty_fields />
				<MediaRemoveToggle media=note.media5 name="remove_media5" empty_fields=empty_fields />
				<MediaRemoveToggle media=note.media6 name="remove_media6" empty_fields=empty_fields />
				<MediaRemoveToggle media=note.media7 name="remove_media7" empty_fields=empty_fields />
				<MediaRemoveToggle media=note.media8 name="remove_media8" empty_fields=empty_fields />
				<MediaRemoveToggle media=note.media9 name="remove_media9" empty_fields=empty_fields />
				<MediaRemoveToggle media=note.media10 name="remove_media10" empty_fields=empty_fields />
			</div>
			<div class=css::file_inputs>
				<Show when=move || (empty_fields.get() >= 1)>
					<FileInput name="media1" value=media1 />
				</Show>
				<Show when=move || (!media1.get().is_empty() && empty_fields.get() >= 2)>
					<FileInput name="media2" value=media2 />
				</Show>
				<Show when=move || (!media2.get().is_empty() && empty_fields.get() >= 3)>
					<FileInput name="media3" value=media3 />
				</Show>
				<Show when=move || (!media3.get().is_empty() && empty_fields.get() >= 4)>
					<FileInput name="media4" value=media4 />
				</Show>
				<Show when=move || (!media4.get().is_empty() && empty_fields.get() >= 5)>
					<FileInput name="media5" value=media5 />
				</Show>
				<Show when=move || (!media5.get().is_empty() && empty_fields.get() >= 6)>
					<FileInput name="media6" value=media6 />
				</Show>
				<Show when=move || (!media6.get().is_empty() && empty_fields.get() >= 7)>
					<FileInput name="media7" value=media7 />
				</Show>
				<Show when=move || (!media7.get().is_empty() && empty_fields.get() >= 8)>
					<FileInput name="media8" value=media8 />
				</Show>
				<Show when=move || (!media8.get().is_empty() && empty_fields.get() >= 9)>
					<FileInput name="media9" value=media9 />
				</Show>
				<Show when=move || (!media9.get().is_empty() && empty_fields.get() >= 10)>
					<FileInput name="media10" value=media10 />
				</Show>
				<Show when=move || empty_fields.get() == 0>
					<em>Notes can have a maximum of 10 attachments. Remove other attachments to upload more.</em>
				</Show>
			</div>
			<div class=css::btns>
				<Button kind="submit" loading>
					Save
				</Button>
				<Button on_click=move |_| is_editing.set(false) variant=ButtonVariant::Outlined>
					Cancel
				</Button>
				<span>
					{move || {
						if let Some(Ok(_)) = edit_note_action.value().get() {
							loading.set(false);
							is_editing.set(false);
							String::from("")
						} else if edit_note_action.value().get().is_some() {
							loading.set(false);
							format!("Error: {:?}", edit_note_action.value().get())
						} else {
							String::from("")
						}
					}}
				</span>
			</div>
		</form>
	}
}

#[component]
pub fn MediaRemoveToggle(media: Option<String>, name: &'static str, empty_fields: RwSignal<usize>) -> impl IntoView {
	let is_checked = create_rw_signal(false);
	let input_ref = create_node_ref::<html::Input>();

	if media.is_some() && !media.clone().unwrap().is_empty() {
		view! {
			<label class=css::media_toggle title="Toggle to remove this attachment">
				<input
					ref=input_ref
					type="checkbox"
					name=name
					checked=is_checked
					on:change=move |_| {
						let input = input_ref.get_untracked().unwrap();
						is_checked.set(input.checked());
						empty_fields.update(move |i| { if is_checked.get() { *i += 1 } else { *i -= 1 } })
					}
				/>
				<img src=media.unwrap() />
				<div>
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
						<path d="m16.192 6.344-4.243 4.242-4.242-4.242-1.414 1.414L10.535 12l-4.242 4.242 1.414 1.414 4.242-4.242 4.243 4.242 1.414-1.414L13.364 12l4.242-4.242z" />
					</svg>
				</div>
			</label>
		}
		.into_view()
	} else {
		view! {}.into_view()
	}
}

#[cfg(feature = "ssr")]
#[derive(Debug, FromRow)]
struct MediasSQL {
	media1: Option<String>,
	media2: Option<String>,
	media3: Option<String>,
	media4: Option<String>,
	media5: Option<String>,
	media6: Option<String>,
	media7: Option<String>,
	media8: Option<String>,
	media9: Option<String>,
	media10: Option<String>,
}

#[server(input = MultipartFormData, prefix = "/api")]
pub async fn edit_note(data: MultipartData) -> Result<(), ServerFnError> {
	use crate::{
		auth::get_user,
		components::file_upload::{file_upload, remove_temp_files},
		permission::{Permission, Permissions},
		utils::{get_equipment_base_folder, get_equipment_notes_folder},
	};

	use sqlx::PgPool;
	use std::{fs, path::PathBuf};
	use tokio::fs::rename;

	let pool = use_context::<PgPool>()
		.ok_or_else::<ServerFnError, _>(|| ServerFnError::ServerError(String::from("Database not initialized")))?;
	let user = get_user().await?;

	let result = file_upload(data, |id| format!("{}temp", get_equipment_base_folder(id))).await?;

	let mut note_id = None;
	let mut notes = None;
	let mut media_removal = Vec::new();

	for (name, value) in &result.additional_fields {
		match name.as_str() {
			f @ "remove_media1"
			| f @ "remove_media2"
			| f @ "remove_media3"
			| f @ "remove_media4"
			| f @ "remove_media5"
			| f @ "remove_media6"
			| f @ "remove_media7"
			| f @ "remove_media8"
			| f @ "remove_media9"
			| f @ "remove_media10" => {
				media_removal.push(f.replace("remove_", ""));
			},
			"notes" => notes = Some(value),
			"note_id" => {
				note_id = {
					let value = match value.parse::<i32>() {
						Ok(value) => value,
						Err(_) => return Err(ServerFnError::Request(String::from("Invalid note ID"))),
					};
					Some(value)
				}
			},
			_ => {},
		}
	}

	if note_id.is_none() {
		return Err(ServerFnError::Request(String::from("No note ID was passed")));
	}
	let note_id = note_id.unwrap();

	match user {
		Some(user) => {
			let Permissions::All {
				read: _,
				write: perm,
				create: _,
			} = user.permission_equipment;
			let person: i32 =
				sqlx::query_scalar("SELECT person FROM equipment_notes WHERE id = $1").bind(note_id).fetch_one(&pool).await?;
			if !perm.has_permission("write", -1, person) && perm != Permission::WriteAny {
				remove_temp_files(result).await?;
				return Err(ServerFnError::Request(String::from("User not authenticated")));
			}
		},
		None => {
			remove_temp_files(result).await?;
			return Err(ServerFnError::Request(String::from("User not authenticated")));
		},
	};

	let notes_folder = get_equipment_notes_folder(note_id);

	let medias = sqlx::query_as::<_, MediasSQL>("SELECT media1, media2, media3, media4, media5, media6, media7, media8, media9, media10 FROM equipment_notes WHERE id = $1")
		.bind(note_id)
		.fetch_one(&pool)
		.await
		.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	// Adding files to our que not marked for removal
	let mut new_medias = Vec::new();
	let media_fields = [
		("media1", medias.media1),
		("media2", medias.media2),
		("media3", medias.media3),
		("media4", medias.media4),
		("media5", medias.media5),
		("media6", medias.media6),
		("media7", medias.media7),
		("media8", medias.media8),
		("media9", medias.media9),
		("media10", medias.media10),
	];
	for (name, media_option) in media_fields {
		if let Some(media) = media_option {
			if !media_removal.contains(&name.to_string()) {
				new_medias.push(media);
			} else {
				// Removing files marked for removal
				let file_path = PathBuf::from(format!("{}/public/{media}", env!("UPLOAD_ROOT")));
				if file_path.exists() {
					match fs::remove_file(&file_path) {
						Ok(_) => {},
						Err(_) => return Err(ServerFnError::Request(format!("Could not delete {file_path:?}"))),
					}
				}
			}
		}
	}

	// Adding new files to the que
	let media_fields = [
		result.media1,
		result.media2,
		result.media3,
		result.media4,
		result.media5,
		result.media6,
		result.media7,
		result.media8,
		result.media9,
		result.media10,
	];
	for media in media_fields {
		if !media.is_empty() {
			let new_path = media.replace("temp/", &notes_folder);
			rename(format!("{}public{media}", env!("UPLOAD_ROOT")), format!("{}public{new_path}", env!("UPLOAD_ROOT")))
				.await?;
			new_medias.push(new_path);
		}
	}

	let new_media1 = new_medias.pop();
	let new_media2 = new_medias.pop();
	let new_media3 = new_medias.pop();
	let new_media4 = new_medias.pop();
	let new_media5 = new_medias.pop();
	let new_media6 = new_medias.pop();
	let new_media7 = new_medias.pop();
	let new_media8 = new_medias.pop();
	let new_media9 = new_medias.pop();
	let new_media10 = new_medias.pop();

	sqlx::query!(
		r#"UPDATE equipment_notes SET
			notes = $2,
			media1 = $3,
			media2 = $4,
			media3 = $5,
			media4 = $6,
			media5 = $7,
			media6 = $8,
			media7 = $9,
			media8 = $10,
			media9 = $11,
			media10 = $12
		WHERE id = $1"#,
		note_id,
		notes,
		new_media1,
		new_media2,
		new_media3,
		new_media4,
		new_media5,
		new_media6,
		new_media7,
		new_media8,
		new_media9,
		new_media10,
	)
	.execute(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	Ok(())
}

#[server(prefix = "/api")]
pub async fn delete_note(id: i32) -> Result<(), ServerFnError> {
	use crate::{
		auth::get_user,
		permission::{Permission, Permissions},
	};

	use sqlx::PgPool;
	use std::{fs, path::PathBuf};

	let pool = use_context::<PgPool>()
		.ok_or_else::<ServerFnError, _>(|| ServerFnError::ServerError(String::from("Database not initialized")))?;
	let user = get_user().await?;

	match user {
		Some(user) => {
			let Permissions::All {
				read: _,
				write: perm,
				create: _,
			} = user.permission_equipment;
			let person: i32 =
				sqlx::query_scalar("SELECT person FROM equipment_notes WHERE id = $1").bind(id).fetch_one(&pool).await?;
			if !perm.has_permission("write", -1, person) && perm != Permission::WriteAny {
				return Err(ServerFnError::Request(String::from("User not authenticated")));
			}
		},
		None => return Err(ServerFnError::Request(String::from("User not authenticated"))),
	};

	let medias = sqlx::query_as::<_, MediasSQL>("SELECT media1, media2, media3, media4, media5, media6, media7, media8, media9, media10 FROM equipment_notes WHERE id = $1")
		.bind(id)
		.fetch_one(&pool)
		.await
		.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	let media_fields = [
		medias.media1,
		medias.media2,
		medias.media3,
		medias.media4,
		medias.media5,
		medias.media6,
		medias.media7,
		medias.media8,
		medias.media9,
		medias.media10,
	];
	for media in media_fields.into_iter().flatten() {
		let file_path = PathBuf::from(format!("{}/public/{media}", env!("UPLOAD_ROOT")));
		if file_path.exists() {
			match fs::remove_file(&file_path) {
				Ok(_) => {},
				Err(_) => return Err(ServerFnError::Request(format!("Could not delete {file_path:?}"))),
			}
		}
	}

	Ok(sqlx::query!("DELETE FROM equipment_notes WHERE id = $1", id).execute(&pool).await.map(|_| ())?)
}

#[server(prefix = "/api")]
pub async fn get_notes_for_equipment(
	id: String,
	page: u16,
	items_per_page: u8,
) -> Result<(Vec<EquipmentNotesData>, i32, i64), ServerFnError> {
	use crate::{auth::get_user, equipment::EquipmentNotesSQLData, permission::Permissions};

	use sqlx::PgPool;

	let pool = use_context::<PgPool>()
		.ok_or_else::<ServerFnError, _>(|| ServerFnError::ServerError(String::from("Database not initialized")))?;
	let user = get_user().await?;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let auth_query = match user {
		Some(user) => {
			let Permissions::All {
				read: perm,
				write: _,
				create: _,
			} = user.permission_equipment;
			perm.get_query_select_without_where("equipment_notes.equipment")
		},
		None => return Err(ServerFnError::Request(String::from("User not authenticated"))),
	};

	let limit = items_per_page as i64;
	let offset = (page as i64 - 1) * items_per_page as i64;

	let notes_sql_data = sqlx::query_as::<_, EquipmentNotesSQLData>(&format!(
		r#"SELECT
		equipment_notes.*,
		people.id AS person_id,
		people.status AS person_status,
		people.preferred_name AS person_preferred_name,
		people.picture AS person_picture
		FROM
			equipment_notes
		JOIN people ON equipment_notes.person = people.id
		WHERE
			equipment_notes.equipment = $1
			{auth_query}
			AND equipment_notes.equipment = $1
		ORDER BY equipment_notes.id DESC
		LIMIT $2 OFFSET $3"#,
	))
	.bind(id)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	let notes_data: Vec<EquipmentNotesData> = notes_sql_data.into_iter().map(Into::into).collect();

	let person_id: i32 =
		sqlx::query_scalar("SELECT person FROM equipment WHERE id = $1").bind(id).fetch_one(&pool).await?;

	let row_count: i64 =
		sqlx::query_scalar(&format!("SELECT COUNT(*) FROM equipment_notes WHERE equipment = $1 {auth_query}"))
			.bind(id)
			.fetch_one(&pool)
			.await?;

	Ok((notes_data, person_id, row_count))
}
