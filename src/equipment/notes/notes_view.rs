use crate::{
	app::ScrollableBody,
	components::{avatar::Avatar, multiline::MultiLine, pagination::Pagination},
	equipment::{save_notes, NotesForm, NotesPerson},
	error_template::ErrorTemplate,
};

use leptos::*;
use web_sys::FormData;

stylance::import_style!(css, "notes.module.css");

#[component]
pub fn Notes(
	id: RwSignal<String>,
	notes_query_page: RwSignal<u16>,
	notes_query_ipp: RwSignal<u8>,
	actions_query_page: RwSignal<u16>,
	actions_query_ipp: RwSignal<u8>,
) -> impl IntoView {
	let notes_upload_action =
		create_action(|data: &FormData| save_notes(data.clone().into()));

	let notes_data = create_resource(
		move || (notes_upload_action.version().get(), id.get()),
		move |_| {
			get_notes_for_equipment(
				id.get(),
				notes_query_page.get(),
				notes_query_ipp.get(),
			)
		},
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
								view! {
									<pre class="error">
										Notes Server Error: {error.to_string()}
									</pre>
								}
									.into_view()
							}
							Ok((notes, count)) => {
								let hidden_fields = vec![
									(
										String::from("actions_page"),
										actions_query_page.get().to_string(),
									),
									(
										String::from("actions_items_per_page"),
										actions_query_ipp.get().to_string(),
									),
								];
								view! {
									<div class=css::notes_list>
										<h2>Notes</h2>
										<Pagination
											action=format!("/equipment/{}", id.get())
											page_key="notes_page"
											ipp_key="notes_items_per_page"
											query_page=notes_query_page
											query_ipp=notes_query_ipp
											row_count=count
											hidden_fields
										/>
										<NotesForm
											id=id.get()
											notes_upload_action=notes_upload_action
										/>
										{notes
											.into_iter()
											.map(|note| {
												view! { <NotesItem note=note /> }
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
pub fn NotesItem(note: NotesPerson) -> impl IntoView {
	view! {
		<div class=css::notes_item>
			<Avatar data=note.person />
			<div>
				<small>
					{note
						.note
						.create_date
						.format("%d %b %Y %I:%M:%S %P")
						.to_string()}
				</small>
				<MultiLine text=note.note.notes />
				<div class=css::imgs>
					<NotesImg img=note.note.media1 />
					<NotesImg img=note.note.media2 />
					<NotesImg img=note.note.media3 />
					<NotesImg img=note.note.media4 />
					<NotesImg img=note.note.media5 />
					<NotesImg img=note.note.media6 />
					<NotesImg img=note.note.media7 />
					<NotesImg img=note.note.media8 />
					<NotesImg img=note.note.media9 />
					<NotesImg img=note.note.media10 />
				</div>
			</div>
		</div>
	}
}

#[component]
pub fn NotesImg(img: Option<String>) -> impl IntoView {
	let is_open = create_rw_signal(false);

	let is_body_scrollable = use_context::<ScrollableBody>()
		.expect("No ScrollableBody context provider");

	view! {
		{if img.is_some() {
			let img = img.unwrap();
			if !img.is_empty() {
				view! {
					<form
						class=move || {
							if is_open.get() {
								format!("{} form-isopen", css::form)
							} else {
								css::form.to_string()
							}
						}
						action=img.clone()
						method="GET"
						on:submit=move |event| {
							event.prevent_default();
							is_body_scrollable.set(is_open.get());
							is_open.update(|open| *open = !*open);
						}
					>
						<button type="submit" class=css::btn>
							<img class=css::img src=img />
						</button>
					</form>
				}
					.into_view()
			} else {
				view! {}.into_view()
			}
		} else {
			view! {}.into_view()
		}}
	}
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
	.map_err::<ServerFnError, _>(|error| {
		ServerFnError::ServerError(error.to_string())
	})?;

	let notes_data: Vec<NotesPerson> =
		notes_sql_data.into_iter().map(Into::into).collect();

	let row_count: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM equipment_notes WHERE equipment = $1",
	)
	.bind(id)
	.fetch_one(get_db())
	.await?;

	Ok((notes_data, row_count))
}
