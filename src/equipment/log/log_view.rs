use crate::{
	components::{avatar::Avatar, img_attachment::ImgAttachment, multiline::MultiLine, pagination::Pagination},
	equipment::{EquipmentLogData, EquipmentLogType, LogAction},
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
pub fn LogItem(log: EquipmentLogData) -> impl IntoView {
	view! {
		<div class=css::log>
			<Avatar data=log.person />
			<div class=css::log_content>
				<small>
					{log.create_date.format("%d %b %Y %I:%M:%S %P").to_string()} {" "}
					<span class=format!(
						"{} type_{}",
						css::log_type,
						log.log_type.to_string().to_lowercase(),
					)>{log.log_type.to_string()}</span>
				</small>
				<MultiLine text=log.notes.unwrap_or_default() />
				<br />
				{if log.log_type == EquipmentLogType::Edit {
					view! {
						<span class=css::log_edit>
							<h2>"Old value for \""{log.field.clone()}"\""</h2>
							<p class=css::diff>{log.old_value}</p>
							<h2>"New value for \""{log.field}"\""</h2>
							<p class=css::diff>{log.new_value}</p>
						</span>
					}
						.into_view()
				} else {
					view! {}.into_view()
				}}
				<div class="codon_img_attachment">
					<ImgAttachment file_path=log.media1 />
					<ImgAttachment file_path=log.media2 />
					<ImgAttachment file_path=log.media3 />
					<ImgAttachment file_path=log.media4 />
					<ImgAttachment file_path=log.media5 />
					<ImgAttachment file_path=log.media6 />
					<ImgAttachment file_path=log.media7 />
					<ImgAttachment file_path=log.media8 />
					<ImgAttachment file_path=log.media9 />
					<ImgAttachment file_path=log.media10 />
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
) -> Result<(Vec<EquipmentLogData>, i64), ServerFnError> {
	use crate::{auth::get_user, equipment::EquipmentLogSQLData, permission::Permissions};

	use sqlx::PgPool;

	let pool = use_context::<PgPool>().expect("Database not initialized");
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
			perm.get_query_select_without_where("equipment_log.equipment")
		},
		None => return Err(ServerFnError::Request(String::from("User not authenticated"))),
	};

	let limit = items_per_page as i64;
	let offset = (page as i64 - 1) * items_per_page as i64;

	let notes_sql_data = sqlx::query_as::<_, EquipmentLogSQLData>(&format!(
		r#"SELECT
			equipment_log.*,
			people.id AS person_id,
			people.status AS person_status,
			people.preferred_name AS person_preferred_name,
			people.picture AS person_picture
		FROM
			equipment_log
			JOIN people ON equipment_log.person = people.id
		WHERE
			equipment_log.equipment = $1
			{auth_query}
			AND equipment_log.equipment = $1
		ORDER BY equipment_log.id DESC
		LIMIT $2 OFFSET $3"#
	))
	.bind(id)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	let notes_data: Vec<EquipmentLogData> = notes_sql_data.into_iter().map(Into::into).collect();

	let row_count: i64 = sqlx::query_scalar(&format!(
		"SELECT COUNT(*) FROM equipment_log WHERE equipment = $1 {auth_query} AND equipment = $1"
	))
	.bind(id)
	.fetch_one(&pool)
	.await?;

	Ok((notes_data, row_count))
}
