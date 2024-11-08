use crate::{
	app::UserSignal,
	components::{
		button::{Button, ButtonVariant},
		file_input::FileInput,
		input::TextArea,
	},
	equipment::{EquipmentData, EquipmentFormToggle, EquipmentStatus},
};

use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use web_sys::{FormData, SubmitEvent};

stylance::import_style!(css, "equipment_details_edits.module.css");

#[component]
pub fn StatusEdit(
	equipment: EquipmentData,
	user_signal: UserSignal,
	refetch_resources: RwSignal<usize>,
) -> impl IntoView {
	let status_action = create_action(|data: &FormData| edit_status(data.clone().into()));
	let is_archived = equipment.status == EquipmentStatus::Archived;
	let is_dirty = equipment.status == EquipmentStatus::Dirty;

	view! {
		<EquipmentFormToggle
			user_id=equipment.person.clone().id
			id=equipment.id
			user_signal=user_signal
			item=equipment.status
		>
			{
				let form_ref = create_node_ref::<html::Form>();
				let action_ref = create_node_ref::<html::Input>();
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
						class=css::edit_form
						method="post"
						action="#"
						enctype="multipart/form-data"
						on:submit=move |event: SubmitEvent| {
							event.prevent_default();
							loading.set(true);
							let form = form_ref.get().unwrap();
							let form_data = match FormData::new_with_form(&form) {
								Ok(fd) => fd,
								Err(error) => {
									logging::log!("Failed to create FormData");
									logging::log!("{error:?}");
									return;
								}
							};
							status_action.dispatch(form_data);
						}
					>
						<input type="hidden" name="id" value=equipment.id />
						<input ref=action_ref type="hidden" name="action" value="next_status" />
						<TextArea name="note" placeholder="Add a note why you made this change" />
						<div class=css::btns>
							<FileInput name="media1" value=media1 />
							<Show when=move || !media1.get().is_empty()>
								<FileInput name="media2" value=media2 />
							</Show>
							<Show when=move || !media2.get().is_empty()>
								<FileInput name="media3" value=media3 />
							</Show>
							<Show when=move || !media3.get().is_empty()>
								<FileInput name="media4" value=media4 />
							</Show>
							<Show when=move || !media4.get().is_empty()>
								<FileInput name="media5" value=media5 />
							</Show>
							<Show when=move || !media5.get().is_empty()>
								<FileInput name="media6" value=media6 />
							</Show>
							<Show when=move || !media6.get().is_empty()>
								<FileInput name="media7" value=media7 />
							</Show>
							<Show when=move || !media7.get().is_empty()>
								<FileInput name="media8" value=media8 />
							</Show>
							<Show when=move || !media8.get().is_empty()>
								<FileInput name="media9" value=media9 />
							</Show>
							<Show when=move || !media9.get().is_empty()>
								<FileInput name="media10" value=media10 />
							</Show>
						</div>
						<div class=css::btns>
							<span>
								{move || {
									if let Some(responds) = status_action.value().get() {
										loading.set(false);
										match responds {
											Ok(_) => {
												status_action.value().set(None);
												refetch_resources.update(|version| *version += 1);
												view! {}.into_view()
											}
											Err(error) => {
												view! {
													<span>
														{error
															.to_string()
															.replace(
																"error reaching server to call server function: ",
																"",
															)}
													</span>
												}
													.into_view()
											}
										}
									} else {
										view! {}.into_view()
									}
								}}
							</span>
							<Show when=move || !is_archived>
								<Button
									kind="submit"
									variant=ButtonVariant::Outlined
									loading
									on:click=move |_| {
										if let Some(action_element) = action_ref.get() {
											let _ = action_element.set_attribute("value", "archive");
										}
									}
								>
									Archive
								</Button>
							</Show>
							<Show when=move || !is_dirty>
								<Button
									kind="submit"
									variant=ButtonVariant::Outlined
									loading
									on:click=move |_| {
										if let Some(action_element) = action_ref.get() {
											let _ = action_element.set_attribute("value", "dirty");
										}
									}
								>
									Mark as Dirty
								</Button>
							</Show>
							<Button
								kind="submit"
								loading
								on:click=move |_| {
									if let Some(action_element) = action_ref.get() {
										let _ = action_element.set_attribute("value", "next_status");
									}
								}
							>
								{if equipment.status == EquipmentStatus::Archived {
									"Unarchive and mark"
								} else {
									"Mark"
								}}
								" as \""
								{EquipmentStatus::get_next_status(equipment.status, equipment.equipment_type)
									.to_string()}
								"\""
							</Button>
						</div>
					</form>
				}
			}
		</EquipmentFormToggle>
	}
}

#[server(input = MultipartFormData, prefix = "/api")]
pub async fn edit_status(data: MultipartData) -> Result<(), ServerFnError> {
	use crate::{
		auth::get_user,
		components::file_upload::{file_upload, remove_temp_files},
		equipment::{EquipmentLogType, EquipmentType},
		permission::Permissions,
		utils::{get_equipment_base_folder, get_equipment_log_folder, move_file},
	};

	use sqlx::PgPool;

	let pool = use_context::<PgPool>()
		.ok_or_else::<ServerFnError, _>(|| ServerFnError::ServerError(String::from("Database not initialized")))?;
	let user = get_user().await?;

	let result = file_upload(data, |id| format!("{}temp/", get_equipment_base_folder(id))).await?;

	let user_id;
	match user {
		Some(user) => {
			let Permissions::All {
				read: _,
				write: perm,
				create: _,
			} = user.permission_equipment;
			user_id = user.id;

			let person: i32 =
				sqlx::query_scalar("SELECT person FROM equipment WHERE id = $1").bind(result.id).fetch_one(&pool).await?;
			if !perm.has_permission("write", result.id, person) {
				remove_temp_files(result).await?;
				return Err(ServerFnError::Request(String::from("User not authenticated")));
			}
		},
		None => {
			remove_temp_files(result).await?;
			return Err(ServerFnError::Request(String::from("User not authenticated")));
		},
	};

	let mut action = None;
	let mut note = None;

	for (name, value) in &result.additional_fields {
		match name.as_str() {
			"action" => action = Some(value),
			"note" => note = Some(value),
			_ => {},
		}
	}

	if action.is_none() {
		return Err(ServerFnError::Request(String::from("Missing button action field")));
	}

	if note.is_none() {
		return Err(ServerFnError::Request(String::from("Missing note field")));
	}

	let action = action.unwrap();
	let note = note.unwrap();

	let (old_status, equipment_type): (String, String) =
		sqlx::query_as::<_, (String, String)>("SELECT status, equipment_type FROM equipment WHERE id = $1")
			.bind(result.id)
			.fetch_one(&pool)
			.await?;
	let next_status = if action == "next_status" {
		EquipmentStatus::get_next_status(EquipmentStatus::parse(old_status.clone()), EquipmentType::parse(equipment_type))
	} else if action == "dirty" {
		EquipmentStatus::Dirty
	} else {
		EquipmentStatus::Archived
	};

	let log = sqlx::query!(
		r#"
		INSERT INTO equipment_log
		(log_type, equipment, person, notes, old_value)
		VALUES
		($1, $2, $3, $4, $5)
		RETURNING id"#,
		EquipmentLogType::from(next_status).to_string(),
		result.id,
		user_id,
		note.to_string(),
		old_status
	)
	.fetch_one(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	let log_folder = get_equipment_log_folder(log.id);

	let media1 = move_file(result.media1, &log_folder).await?;
	let media2 = move_file(result.media2, &log_folder).await?;
	let media3 = move_file(result.media3, &log_folder).await?;
	let media4 = move_file(result.media4, &log_folder).await?;
	let media5 = move_file(result.media5, &log_folder).await?;
	let media6 = move_file(result.media6, &log_folder).await?;
	let media7 = move_file(result.media7, &log_folder).await?;
	let media8 = move_file(result.media8, &log_folder).await?;
	let media9 = move_file(result.media9, &log_folder).await?;
	let media10 = move_file(result.media10, &log_folder).await?;

	sqlx::query!(
		r#"UPDATE equipment_log set
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
		log.id,
	)
	.execute(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query("UPDATE equipment SET status = $1 WHERE id = $2")
		.bind(format!("{next_status:#?}"))
		.bind(result.id)
		.execute(&pool)
		.await
		.map(|_| ())?;

	Ok(())
}
