use crate::{
	components::{
		avatar::Avatar, button::Button, multiline::MultiLine,
		pagination::Pagination,
	},
	equipment::{
		save_notes, ActionsPerson, EquipmentCell, EquipmentData, EquipmentStatus,
		NotesForm, NotesItem, NotesPerson,
	},
	error_template::ErrorTemplate,
	icons::EquipmentLogo,
};

use leptos::*;
use leptos_router::*;
use web_sys::FormData;

stylance::import_style!(css, "equipment_details.module.css");

#[component]
pub fn EquipmentDetail() -> impl IntoView {
	let params = use_params_map();
	let query = use_query_map();
	let navigate = use_navigate();

	let notes_query_page = create_rw_signal({
		let page = query
			.with(|p| p.get("notes_page").cloned().unwrap_or(String::from("1")))
			.parse::<u16>()
			.unwrap_or(1);
		if page > 0 {
			page
		} else {
			1
		}
	});

	let notes_query_ipp = create_rw_signal({
		let ipp = query
			.with(|p| {
				p.get("notes_items_per_page").cloned().unwrap_or(String::from("25"))
			})
			.parse::<u8>()
			.unwrap_or(25);
		if ipp > 0 {
			ipp
		} else {
			1
		}
	});

	let actions_query_page = create_rw_signal({
		let page = query
			.with(|p| p.get("actions_page").cloned().unwrap_or(String::from("1")))
			.parse::<u16>()
			.unwrap_or(1);
		if page > 0 {
			page
		} else {
			1
		}
	});

	let actions_query_ipp = create_rw_signal({
		let ipp = query
			.with(|p| {
				p.get("actions_items_per_page").cloned().unwrap_or(String::from("25"))
			})
			.parse::<u8>()
			.unwrap_or(25);
		if ipp > 0 {
			ipp
		} else {
			1
		}
	});

	let go_to_listing = create_rw_signal(false);
	let id =
		create_rw_signal(params.with(|p| p.get("id").cloned().unwrap_or_default()));

	create_effect(move |_| {
		if id.get().is_empty() || go_to_listing.get() {
			navigate("/equipment", Default::default());
		}
	});

	let notes_upload_action =
		create_action(|data: &FormData| save_notes(data.clone().into()));

	#[expect(clippy::redundant_closure)]
	let equipment_data =
		create_resource(move || id.get(), move |id| get_equipment_data_by_id(id));

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

	let actions_data = create_resource(
		move || id.get(),
		move |id| {
			get_actions_for_equipment(
				id,
				actions_query_page.get(),
				actions_query_ipp.get(),
			)
		},
	);

	view! {
		<Transition fallback=move || view! { <p>Loading equipment...</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors=errors /> }
			}>
				{move || {
					let equipment = {
						move || {
							if equipment_data.get().is_some() {
								match equipment_data.get().unwrap() {
									Err(error) => {
										go_to_listing.set(true);
										view! {
											<pre class="error">Server Error: {error.to_string()}</pre>
										}
											.into_view()
									}
									Ok(equipment) => {
										let is_archived = equipment.status.clone()
											== EquipmentStatus::Archived;
										view! {
											<div class=css::details>
												<h1 class=css::heading>
													<EquipmentLogo />
													" "
													{equipment.name.clone()}
													<A href=format!("/equipment/edit/{}", equipment.id)>Edit</A>
												</h1>

												<dl class=css::list>
													<dt>ID</dt>
													<dd>
														<EquipmentCell cell=equipment.id />
													</dd>

													<dt>Name</dt>
													<dd>
														<EquipmentCell cell=equipment.name />
													</dd>

													<dt>Equipment Type</dt>
													<dd>
														<EquipmentCell cell=equipment.equipment_type.clone() />
													</dd>

													<dt>Qrcode</dt>
													<dd>
														<EquipmentCell cell=equipment.qrcode />
													</dd>

													<dt>Create Date</dt>
													<dd>
														<EquipmentCell cell=equipment.create_date />
													</dd>

													<dt>Status</dt>
													<dd>
														<EquipmentCell cell=equipment.status.clone() />

														<Button>
															"Mark as \""
															{EquipmentStatus::get_next_status(
																	equipment.status.clone(),
																	equipment.equipment_type,
																)
																.to_string()}"\""
														</Button>
														<Show when=move || !is_archived>
															<Button>"Archive"</Button>
														</Show>
													</dd>

													<dt>Manufacturer</dt>
													<dd>
														<EquipmentCell cell=equipment.manufacturer />
													</dd>

													<dt>Purchase Date</dt>
													<dd>
														<EquipmentCell cell=equipment.purchase_date />
													</dd>

													<dt>Vendor</dt>
													<dd>
														<EquipmentCell cell=equipment.vendor />
													</dd>

													<dt>Cost</dt>
													<dd>
														<EquipmentCell cell=equipment.cost_in_cent />
													</dd>

													<dt>Warranty Expiration Date</dt>
													<dd>
														<EquipmentCell cell=equipment.warranty_expiration_date />
													</dd>

													<dt>Location</dt>
													<dd>
														<EquipmentCell cell=equipment.location />
													</dd>

													<dt>Notes</dt>
													<dd>
														<EquipmentCell cell=equipment.notes />
													</dd>
												</dl>
											</div>
										}
											.into_view()
									}
								}
							} else {
								view! { <div>Nothing found</div> }.into_view()
							}
						}
					};
					let notes = {
						move || {
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
										let params = use_params_map();
										let id = params
											.with(|p| p.get("id").cloned().unwrap_or_default());
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
											<div class=css::attachment_list>
												<h2>Notes</h2>
												<Pagination
													action=format!("/equipment/{id}")
													page_key="notes_page"
													ipp_key="notes_items_per_page"
													query_page=notes_query_page
													query_ipp=notes_query_ipp
													row_count=count
													hidden_fields
												/>
												<NotesForm id=id notes_upload_action=notes_upload_action />
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
						}
					};
					let actions = {
						move || {
							if actions_data.get().is_some() {
								match actions_data.get().unwrap() {
									Err(error) => {
										view! {
											<pre class="error">
												Actions Server Error: {error.to_string()}
											</pre>
										}
											.into_view()
									}
									Ok((actions, count)) => {
										let params = use_params_map();
										let id = params
											.with(|p| p.get("id").cloned().unwrap_or_default());
										let hidden_fields = vec![
											(
												String::from("notes_page"),
												notes_query_page.get().to_string(),
											),
											(
												String::from("notes_items_per_page"),
												notes_query_ipp.get().to_string(),
											),
										];
										view! {
											<div class=css::attachment_list>
												<h2>Log:</h2>
												<Pagination
													action=format!("/equipment/{id}")
													page_key="actions_page"
													ipp_key="actions_items_per_page"
													query_page=actions_query_page
													query_ipp=actions_query_ipp
													row_count=count
													hidden_fields
												/>
												<div>
													{actions
														.into_iter()
														.map(|action| {
															view! {
																<Avatar data=action.person />
																<span>
																	-{format!("{}", action.action.action_type)}-
																	<MultiLine text=action.action.notes.unwrap_or_default() />
																</span>
															}
														})
														.collect_view()}
												</div>
											</div>
										}
											.into_view()
									}
								}
							} else {
								view! { <div>No Actions found</div> }.into_view()
							}
						}
					};
					view! { <div>{equipment} {notes} {actions}</div> }
				}}
			</ErrorBoundary>
		</Transition>
	}
}

#[server]
pub async fn get_equipment_data_by_id(
	id: String,
) -> Result<EquipmentData, ServerFnError> {
	use crate::{db::ssr::get_db, equipment::EquipmentSQLData};

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
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

	Ok(equipment_sql_data.into())
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

#[server]
pub async fn get_actions_for_equipment(
	id: String,
	page: u16,
	items_per_page: u8,
) -> Result<(Vec<ActionsPerson>, i64), ServerFnError> {
	use crate::{db::ssr::get_db, equipment::ActionsPersonSQL};

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let limit = items_per_page as i64;
	let offset = (page as i64 - 1) * items_per_page as i64;

	let notes_sql_data = sqlx::query_as::<_, ActionsPersonSQL>(
		r#"SELECT
			equipment_actions.id AS action_id,
			equipment_actions.action_type AS action_action_type,
			equipment_actions.equipment AS action_equipment,
			equipment_actions.create_date AS action_create_date,
			equipment_actions.person AS action_person,
			equipment_actions.notes AS action_notes,
			equipment_actions.field AS action_field,
			equipment_actions.old_value AS action_old_value,
			equipment_actions.new_value AS action_new_value,

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
			equipment_actions
		JOIN people ON equipment_actions.person = people.id
		WHERE
			equipment_actions.equipment = $1
		ORDER BY equipment_actions.id DESC
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

	let notes_data: Vec<ActionsPerson> =
		notes_sql_data.into_iter().map(Into::into).collect();

	let row_count: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM equipment_actions WHERE equipment = $1",
	)
	.bind(id)
	.fetch_one(get_db())
	.await?;

	Ok((notes_data, row_count))
}
