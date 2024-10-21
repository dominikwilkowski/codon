use crate::{
	components::{avatar::Avatar, img_attachment::ImgAttachment, multiline::MultiLine, pagination::Pagination},
	equipment::{EquipmentLogType, LogAction, LogPerson},
	error_template::ErrorTemplate,
};

use leptos::*;

stylance::import_style!(css, "log.module.css");

#[component]
pub fn Log(
	id: RwSignal<String>,
	notes_query_page: RwSignal<u16>,
	notes_query_ipp: RwSignal<u8>,
	log_query_page: RwSignal<u16>,
	log_query_ipp: RwSignal<u8>,
	tab_query: RwSignal<String>,
	log_data: LogAction,
) -> impl IntoView {
	view! {
		<Transition fallback=move || view! { <p>Loading notes...</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors=errors /> }
			}>
				{move || {
					if log_data.get().is_some() {
						match log_data.get().unwrap() {
							Err(error) => {
								view! { <pre class="error">Log Server Error: {error.to_string()}</pre> }.into_view()
							}
							Ok((log, count)) => {
								let hidden_fields = vec![
									(String::from("notes_page"), notes_query_page.get().to_string()),
									(String::from("notes_items_per_page"), notes_query_ipp.get().to_string()),
									(String::from("tab"), tab_query.get()),
								];
								view! {
									<div class=css::log_list id="equipment_log">
										<Pagination
											action=format!("/equipment/{}#equipment_log", id.get())
											page_key="log_page"
											ipp_key="log_items_per_page"
											query_page=log_query_page
											query_ipp=log_query_ipp
											row_count=count
											hidden_fields
										/>
										<div class=css::log_wrapper>
											{log
												.into_iter()
												.map(|log| {
													view! { <LogItem log=log /> }
												})
												.collect_view()}
										</div>
									</div>
								}
									.into_view()
							}
						}
					} else {
						view! { <div>No logs found</div> }.into_view()
					}
				}}
			</ErrorBoundary>
		</Transition>
	}
}

#[component]
pub fn LogItem(log: LogPerson) -> impl IntoView {
	view! {
		<div class=css::log>
			<Avatar data=log.person />
			<div class=css::log_content>
				<small>
					{log.log.create_date.format("%d %b %Y %I:%M:%S %P").to_string()} {" "}
					<span class=css::log_type>{format!("{}", log.log.log_type)}</span>
				</small>
				<MultiLine text=log.log.notes.unwrap_or_default() />
				<br />
				{if log.log.log_type == EquipmentLogType::Edit {
					view! {
						<span class=css::log_edit>
							<h2>"Old value for \""{log.log.field.clone()}"\""</h2>
							<p class=css::diff>{log.log.old_value}</p>
							<h2>"New value for \""{log.log.field}"\""</h2>
							<p class=css::diff>{log.log.new_value}</p>
						</span>
					}
						.into_view()
				} else {
					view! {}.into_view()
				}}
				<div class="codon_img_attachment">
					<ImgAttachment file_path=log.log.media1 />
					<ImgAttachment file_path=log.log.media2 />
					<ImgAttachment file_path=log.log.media3 />
					<ImgAttachment file_path=log.log.media4 />
					<ImgAttachment file_path=log.log.media5 />
					<ImgAttachment file_path=log.log.media6 />
					<ImgAttachment file_path=log.log.media7 />
					<ImgAttachment file_path=log.log.media8 />
					<ImgAttachment file_path=log.log.media9 />
					<ImgAttachment file_path=log.log.media10 />
				</div>
			</div>
		</div>
	}
}

#[server(prefix = "/api")]
pub async fn get_log_for_equipment(
	id: String,
	page: u16,
	items_per_page: u8,
) -> Result<(Vec<LogPerson>, i64), ServerFnError> {
	use crate::equipment::LogPersonSQL;

	use sqlx::PgPool;

	let pool = use_context::<PgPool>().expect("Database not initialized");

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let limit = items_per_page as i64;
	let offset = (page as i64 - 1) * items_per_page as i64;

	let notes_sql_data = sqlx::query_as::<_, LogPersonSQL>(
		r#"SELECT
			equipment_log.id AS log_id,
			equipment_log.log_type AS log_log_type,
			equipment_log.equipment AS log_equipment,
			equipment_log.create_date AS log_create_date,
			equipment_log.person AS log_person,
			equipment_log.notes AS log_notes,
			equipment_log.field AS log_field,
			equipment_log.old_value AS log_old_value,
			equipment_log.new_value AS log_new_value,
			equipment_log.media1 AS log_media1,
			equipment_log.media2 AS log_media2,
			equipment_log.media3 AS log_media3,
			equipment_log.media4 AS log_media4,
			equipment_log.media5 AS log_media5,
			equipment_log.media6 AS log_media6,
			equipment_log.media7 AS log_media7,
			equipment_log.media8 AS log_media8,
			equipment_log.media9 AS log_media9,
			equipment_log.media10 AS log_media10,

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
			equipment_log
			JOIN people ON equipment_log.person = people.id
		WHERE
			equipment_log.equipment = $1
			ORDER BY equipment_log.id DESC
		LIMIT $2 OFFSET $3"#,
	)
	.bind(id)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	let notes_data: Vec<LogPerson> = notes_sql_data.into_iter().map(Into::into).collect();

	let row_count: i64 =
		sqlx::query_scalar("SELECT COUNT(*) FROM equipment_log WHERE equipment = $1").bind(id).fetch_one(&pool).await?;

	Ok((notes_data, row_count))
}
