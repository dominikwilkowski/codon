use crate::{
	app::UserSignal,
	components::{
		button::Button,
		input::{Input, TextArea},
	},
	equipment::{EquipmentData, EquipmentFormToggle},
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment_details_edits.module.css");

#[component]
pub fn ManufacturerEdit(
	equipment: EquipmentData,
	user_signal: UserSignal,
	refetch_resources: RwSignal<usize>,
) -> impl IntoView {
	let manufacturer_action = create_server_action::<EditManufacturer>();

	view! {
		<EquipmentFormToggle
			user_id=equipment.person.id
			id=equipment.id
			user_signal=user_signal
			item=equipment.manufacturer.clone()
		>
			{
				let manufacturer_clone = equipment.manufacturer.clone();
				view! {
					<ActionForm action=manufacturer_action class=css::edit_form>
						<input type="hidden" name="id" value=equipment.id />
						<Input name="manufacturer" value=create_rw_signal(manufacturer_clone.unwrap_or_default()) />
						<TextArea name="note" placeholder="Add a note why you made this change" />
						<div class=css::btns>
							{move || {
								if let Some(responds) = manufacturer_action.value().get() {
									match responds {
										Ok(_) => {
											manufacturer_action.value().set(None);
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
pub async fn edit_manufacturer(id: String, manufacturer: String, note: String) -> Result<(), ServerFnError> {
	use crate::{auth::get_user, permission::Permissions};

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

	let old_value: String =
		sqlx::query_scalar("SELECT manufacturer FROM equipment WHERE id = $1").bind(id).fetch_one(&pool).await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		user_id,
		note,
		"manufacturer",
		old_value,
		manufacturer,
	)
	.execute(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET manufacturer = $1 WHERE id = $2", manufacturer, id)
		.execute(&pool)
		.await
		.map(|_| ())?;

	Ok(())
}