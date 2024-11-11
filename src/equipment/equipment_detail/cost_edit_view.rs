use crate::{
	app::UserSignal,
	components::{
		button::Button,
		input::{MoneyInput, TextArea},
	},
	equipment::{EquipmentData, EquipmentFormToggle},
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment_details_edits.module.css");

#[component]
pub fn CostEdit(
	equipment: EquipmentData,
	user_signal: UserSignal,
	refetch_resources: RwSignal<usize>,
) -> impl IntoView {
	let cost_in_cent_action = create_server_action::<EditCostInCent>();

	view! {
		<EquipmentFormToggle
			user_id=equipment.person.id
			id=equipment.id
			user_signal=user_signal
			item=equipment.cost_in_cent.clone()
		>
			{
				let cost_in_cent_clone = equipment.cost_in_cent.clone();
				view! {
					<ActionForm action=cost_in_cent_action class=css::edit_form>
						<input type="hidden" name="id" value=equipment.id />
						<MoneyInput
							name="cost_in_cent"
							value=create_rw_signal(cost_in_cent_clone.unwrap_or_default().to_string())
						/>
						<TextArea name="note" placeholder="Add a note why you made this change" />
						<div class=css::btns>
							{move || {
								if let Some(responds) = cost_in_cent_action.value().get() {
									match responds {
										Ok(_) => {
											cost_in_cent_action.value().set(None);
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
pub async fn edit_cost_in_cent(id: String, cost_in_cent: f32, note: String) -> Result<(), ServerFnError> {
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

	let cost_in_cent = (cost_in_cent * 100.0) as i32;

	let old_value: i32 =
		sqlx::query_scalar("SELECT cost_in_cent FROM equipment WHERE id = $1").bind(id).fetch_one(&pool).await?;
	let old_value = format!("{}", (old_value as f32 / 100.0));

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		user_id,
		note,
		"cost_in_cent",
		old_value,
		format!("{:.2}", (cost_in_cent as f32 / 100.0)),
	)
	.execute(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET cost_in_cent = $1 WHERE id = $2", cost_in_cent, id)
		.execute(&pool)
		.await
		.map(|_| ())?;

	Ok(())
}
