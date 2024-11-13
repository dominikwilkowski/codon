use crate::{
	app::UserSignal,
	components::{button::Button, datepicker::DatePicker, input::TextArea, timezone_offset::Timezone},
	equipment::{EquipmentData, EquipmentFormToggle},
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment_details_edits.module.css");

#[component]
pub fn WarrantyExpirationDateEdit(
	equipment: EquipmentData,
	user_signal: UserSignal,
	refetch_resources: RwSignal<usize>,
) -> impl IntoView {
	let warranty_expiration_date_action = create_server_action::<EditWarrantyExpirationDate>();

	view! {
		<EquipmentFormToggle
			user_id=equipment.person.id
			id=equipment.id
			user_signal=user_signal
			item=equipment.warranty_expiration_date
		>
			{
				let warranty_expiration_date_clone = equipment.warranty_expiration_date;
				view! {
					<ActionForm action=warranty_expiration_date_action class=css::edit_form>
						<input type="hidden" name="id" value=equipment.id />
						<Timezone />
						<DatePicker
							attr:name="warranty_expiration_date"
							value=create_rw_signal(
								Some(warranty_expiration_date_clone.unwrap_or_default().date_naive()),
							)
						/>
						<TextArea name="note" placeholder="Add a note why you made this change" />
						<div class=css::btns>
							{move || {
								if let Some(responds) = warranty_expiration_date_action.value().get() {
									match responds {
										Ok(_) => {
											warranty_expiration_date_action.value().set(None);
											refetch_resources.update(|version| *version += 1);
											view! {}.into_view()
										}
										Err(error) => {
											view! {
												<span>
													{error
														.to_string()
														.replace("error reaching server to call server function: ", "")}
												</span>
											}
												.into_view()
										}
									}
								} else {
									view! {}.into_view()
								}
							}} <Button kind="submit">Save</Button>
						</div>
					</ActionForm>
				}
			}
		</EquipmentFormToggle>
	}
}

#[server(prefix = "/api")]
pub async fn edit_warranty_expiration_date(
	id: String,
	warranty_expiration_date: String,
	timezone_offset: i32,
	note: String,
) -> Result<(), ServerFnError> {
	use crate::{auth::get_user, permission::Permissions};

	use chrono::prelude::*;
	use sqlx::PgPool;

	let pool = use_context::<PgPool>()
		.ok_or_else::<ServerFnError, _>(|| ServerFnError::ServerError(String::from("Database not initialized")))?;
	let user = get_user().await?;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

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
				sqlx::query_scalar("SELECT person FROM equipment WHERE id = $1").bind(id).fetch_one(&pool).await?;
			if !perm.has_permission("write", id, person) {
				return Err(ServerFnError::Request(String::from("User not authenticated")));
			}
		},
		None => return Err(ServerFnError::Request(String::from("User not authenticated"))),
	};

	let hours = timezone_offset / 60;
	let minutes = timezone_offset % 60;
	let offset_str = format!("{:+03}:{:02}", hours, minutes.abs());
	let warranty_expiration_date_with_tz = format!("{}T00:00:00{}", warranty_expiration_date, offset_str);

	let warranty_expiration_date: DateTime<Utc> =
		match DateTime::parse_from_str(&warranty_expiration_date_with_tz, "%Y-%m-%dT%H:%M:%S%z") {
			Ok(date) => date,
			Err(error) => return Err(ServerFnError::Request(format!("Invalid date: {}", error))),
		}
		.with_timezone(&Utc);

	let old_value: Option<DateTime<Utc>> =
		sqlx::query_scalar("SELECT warranty_expiration_date FROM equipment WHERE id = $1")
			.bind(id)
			.fetch_one(&pool)
			.await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		user_id,
		note,
		"warranty_expiration_date",
		old_value.unwrap_or_default().format("%d %b %Y").to_string(),
		warranty_expiration_date.format("%d %b %Y").to_string(),
	)
	.execute(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET warranty_expiration_date = $1 WHERE id = $2", warranty_expiration_date, id)
		.execute(&pool)
		.await
		.map(|_| ())?;

	Ok(())
}
