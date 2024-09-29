use crate::{
	components::{
		avatar::Avatar,
		button::Button,
		dropdown::{Dropdown, DropdownItem, DropdownPlacement, DropdownTrigger},
		file_input::FileInput,
		img_attachment::ImgAttachment,
		input::TextArea,
		multiline::MultiLine,
		pagination::Pagination,
	},
	equipment::{save_notes, NotesForm, NotesPerson},
	error_template::ErrorTemplate,
};

use leptos::*;
use leptos_router::*;
use server_fn::codec::{MultipartData, MultipartFormData};
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
	let notes_upload_action = create_action(|data: &FormData| save_notes(data.clone().into()));
	let edit_note_action = create_action(|data: &FormData| edit_note(data.clone().into()));
	let delete_note_action = create_server_action::<DeleteNote>();

	let notes_data = create_resource(
		move || {
			(
				notes_upload_action.version().get(),
				id.get(),
				edit_note_action.version().get(),
				delete_note_action.version().get(),
			)
		},
		move |_| get_notes_for_equipment(id.get(), notes_query_page.get(), notes_query_ipp.get()),
	);

	view! {
		<Transition fallback=move || view! { <p>Loading notes...</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors=errors /> }
			}>
				{move || {
					if notes_data.get().is_some() {
						match notes_data.get().unwrap() {
							Err(error) => {
								view! { <pre class="error">Notes Server Error: {error.to_string()}</pre> }.into_view()
							}
							Ok((notes, count)) => {
								let hidden_fields = vec![
									(String::from("log_page"), log_query_page.get().to_string()),
									(String::from("log_items_per_page"), log_query_ipp.get().to_string()),
									(String::from("tab"), tab_query.get()),
								];
								view! {
									<div class=css::notes_list id="equipment_notes">
										<NotesForm id=id.get() notes_upload_action=notes_upload_action />
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
		</Transition>
	}
}

#[component]
pub fn NotesItem(
	note: NotesPerson,
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
	note: NotesPerson,
	is_editing: RwSignal<bool>,
	delete_note_action: Action<DeleteNote, Result<(), ServerFnError>>,
) -> impl IntoView {
	view! {
		<small>
			{note.note.create_date.format("%d %b %Y %I:%M:%S %P").to_string()}
			<Dropdown
				placement=DropdownPlacement::BottomEnd
				on_select=move |link: String| if link.as_str() == "edit" {
					is_editing.set(true);
				}
			>
				<DropdownTrigger slot>
					<svg class=css::menu xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
						<path d="M12 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm6 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zM6 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z" />
					</svg>
				</DropdownTrigger>
				<DropdownItem key="edit" label="Edit" />
				<ActionForm class=css::dropdown_form action=delete_note_action>
					<input type="hidden" name="id" value=note.note.id />
					<button class=css::text_btn type="submit">
						Delete
					</button>
				</ActionForm>
			</Dropdown>
		</small>
		<MultiLine text=note.note.notes />
		<div class="codon_img_attachment">
			<ImgAttachment file_path=note.note.media1 />
			<ImgAttachment file_path=note.note.media2 />
			<ImgAttachment file_path=note.note.media3 />
			<ImgAttachment file_path=note.note.media4 />
			<ImgAttachment file_path=note.note.media5 />
			<ImgAttachment file_path=note.note.media6 />
			<ImgAttachment file_path=note.note.media7 />
			<ImgAttachment file_path=note.note.media8 />
			<ImgAttachment file_path=note.note.media9 />
			<ImgAttachment file_path=note.note.media10 />
		</div>
	}
}

#[component]
pub fn NoteEdit(
	note: NotesPerson,
	is_editing: RwSignal<bool>,
	edit_note_action: Action<FormData, Result<(), ServerFnError>>,
) -> impl IntoView {
	let form_ref = create_node_ref::<html::Form>();

	let media1 = create_rw_signal(note.note.media1.unwrap_or_default());
	let media2 = create_rw_signal(note.note.media2.unwrap_or_default());
	let media3 = create_rw_signal(note.note.media3.unwrap_or_default());
	let media4 = create_rw_signal(note.note.media4.unwrap_or_default());
	let media5 = create_rw_signal(note.note.media5.unwrap_or_default());
	let media6 = create_rw_signal(note.note.media6.unwrap_or_default());
	let media7 = create_rw_signal(note.note.media7.unwrap_or_default());
	let media8 = create_rw_signal(note.note.media8.unwrap_or_default());
	let media9 = create_rw_signal(note.note.media9.unwrap_or_default());
	let media10 = create_rw_signal(note.note.media10.unwrap_or_default());
	let loading = create_rw_signal(false);

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
			<TextArea value=create_rw_signal(note.note.notes) name="notes" placeholder="Your note" />
			<div class=css::file_inputs>
				<span>
					<FileInput name="media1" value=media1 />
				</span>
				<span class=move || { if media1.get().is_empty() { "is_hidden" } else { "" } }>
					<FileInput name="media2" value=media2 />
				</span>
				<span class=move || { if media2.get().is_empty() { "is_hidden" } else { "" } }>
					<FileInput name="media3" value=media3 />
				</span>
				<span class=move || { if media3.get().is_empty() { "is_hidden" } else { "" } }>
					<FileInput name="media4" value=media4 />
				</span>
				<span class=move || { if media4.get().is_empty() { "is_hidden" } else { "" } }>
					<FileInput name="media5" value=media5 />
				</span>
				<span class=move || { if media5.get().is_empty() { "is_hidden" } else { "" } }>
					<FileInput name="media6" value=media6 />
				</span>
				<span class=move || { if media6.get().is_empty() { "is_hidden" } else { "" } }>
					<FileInput name="media7" value=media7 />
				</span>
				<span class=move || { if media7.get().is_empty() { "is_hidden" } else { "" } }>
					<FileInput name="media8" value=media8 />
				</span>
				<span class=move || { if media8.get().is_empty() { "is_hidden" } else { "" } }>
					<FileInput name="media9" value=media9 />
				</span>
				<span class=move || { if media9.get().is_empty() { "is_hidden" } else { "" } }>
					<FileInput name="media10" value=media10 />
				</span>
			</div>
			<div class=css::btns>
				<Button loading>Save</Button>
				<Button on_click=move |_| is_editing.set(false) outlined=true>
					Cancel
				</Button>
				<span>
					{move || {
						if let Some(Ok(_)) = edit_note_action.value().get() {
							loading.set(false);
							is_editing.set(false);
							String::from("")
						} else {
							loading.set(false);
							format!("Error: {:?}", edit_note_action.value().get())
						}
					}}
				</span>
			</div>
		</form>
	}
}

#[server(input = MultipartFormData)]
pub async fn edit_note(data: MultipartData) -> Result<(), ServerFnError> {
	println!("submitted: {:?}", data);
	Ok(())
}

#[server]
pub async fn delete_note(id: i32) -> Result<(), ServerFnError> {
	println!("submitted: {id}");
	Ok(())
}

#[server]
pub async fn get_notes_for_equipment(
	id: String,
	page: u16,
	items_per_page: u8,
) -> Result<(Vec<NotesPerson>, i64), ServerFnError> {
	use crate::{db::ssr::get_db, equipment::NotesPersonSQL};

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let limit = items_per_page as i64;
	let offset = (page as i64 - 1) * items_per_page as i64;

	let notes_sql_data = sqlx::query_as::<_, NotesPersonSQL>(
		r#"SELECT
			equipment_notes.id AS note_id,
			equipment_notes.equipment AS note_equipment,
			equipment_notes.create_date AS note_create_date,
			equipment_notes.person AS note_person,
			equipment_notes.notes AS note_notes,
			equipment_notes.media1 AS note_media1,
			equipment_notes.media2 AS note_media2,
			equipment_notes.media3 AS note_media3,
			equipment_notes.media4 AS note_media4,
			equipment_notes.media5 AS note_media5,
			equipment_notes.media6 AS note_media6,
			equipment_notes.media7 AS note_media7,
			equipment_notes.media8 AS note_media8,
			equipment_notes.media9 AS note_media9,
			equipment_notes.media10 AS note_media10,

			people.id AS person_id,
			people.employee_id AS person_employee_id,
			people.status AS person_status,
			people.first_name AS person_first_name,
			people.last_name AS person_last_name,
			people.preferred_name AS person_preferred_name,
			people.email AS person_email,
			people.phone_number AS person_phone_number,
			people.department AS person_department,
			people.role AS person_role,
			people.hire_date AS person_hire_date,
			people.emergency_contact AS person_emergency_contact,
			people.certifications AS person_certifications,
			people.specializations AS person_specializations,
			people.picture AS person_picture,
			people.bio AS person_bio,
			people.create_date AS person_create_date
		FROM
			equipment_notes
		JOIN people ON equipment_notes.person = people.id
		WHERE
			equipment_notes.equipment = $1
		ORDER BY equipment_notes.id DESC
		LIMIT $2 OFFSET $3"#,
	)
	.bind(id)
	.bind(limit)
	.bind(offset)
	.fetch_all(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	let notes_data: Vec<NotesPerson> = notes_sql_data.into_iter().map(Into::into).collect();

	let row_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM equipment_notes WHERE equipment = $1")
		.bind(id)
		.fetch_one(get_db())
		.await?;

	Ok((notes_data, row_count))
}
