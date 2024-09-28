use crate::{
	components::{avatar::Avatar, multiline::MultiLine, pagination::Pagination},
	equipment::ActionsPerson,
	error_template::ErrorTemplate,
};

use leptos::*;

stylance::import_style!(css, "actions.module.css");

#[component]
pub fn Actions(
	id: RwSignal<String>,
	notes_query_page: RwSignal<u16>,
	notes_query_ipp: RwSignal<u8>,
	actions_query_page: RwSignal<u16>,
	actions_query_ipp: RwSignal<u8>,
) -> impl IntoView {
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
		<Transition fallback=move || view! { <p>Loading notes...</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors=errors /> }
			}>
				{move || {
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
									<div class=css::action_list>
										<h2>Log:</h2>
										<Pagination
											action=format!("/equipment/{}", id.get())
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
				}}
			</ErrorBoundary>
		</Transition>
	}
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
