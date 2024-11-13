use crate::{
	app::{LoginAction, UserSignal},
	components::{
		button::Button,
		input::{Input, TextArea},
	},
	equipment::{
		get_log_for_equipment, CostEdit, EquipmentCell, EquipmentData, EquipmentFormToggle, EquipmentLogData,
		EquipmentType, Heading, Log, ManufacturerEdit, NameEdit, Notes, PurchaseDateEdit, StatusEdit, TypeEdit, VendorEdit,
		WarrantyExpirationDateEdit,
	},
	error_template::ErrorTemplate,
	icons::{FlaskLogo, IncubationCabinetLogo, VesselLogo},
	login::Login,
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment_details.module.css");

pub type LogAction = Resource<(String, usize), Result<(Vec<EquipmentLogData>, i64), ServerFnError>>;

#[component]
pub fn EquipmentDetail() -> impl IntoView {
	let params = use_params_map();
	let query = use_query_map();

	let id = create_rw_signal(String::new());
	let refetch_resources = create_rw_signal(0);
	let notes_query_page = create_rw_signal::<u16>(1);
	let notes_query_ipp = create_rw_signal::<u8>(25);
	let log_query_page = create_rw_signal::<u16>(1);
	let log_query_ipp = create_rw_signal::<u8>(25);
	let tab_query = create_rw_signal(String::from("notes"));

	create_effect(move |_| {
		id.set(params.with(|p| p.get("id").cloned().unwrap_or_default()));
	});

	create_effect(move |_| {
		let (notes_page, notes_ipp, log_page, log_ipp, tab) = query.with(|p| {
			let notes_page = p.get("notes_page").cloned().unwrap_or(String::from("1")).parse::<u16>().unwrap_or(1);
			let notes_ipp = p.get("notes_items_per_page").cloned().unwrap_or(String::from("25")).parse::<u8>().unwrap_or(25);
			let log_page = p.get("log_page").cloned().unwrap_or(String::from("1")).parse::<u16>().unwrap_or(1);
			let log_ipp = p.get("log_items_per_page").cloned().unwrap_or(String::from("25")).parse::<u8>().unwrap_or(25);
			let tab = match p.get("tab").cloned().unwrap_or(String::from("notes")).as_str() {
				"notes" => String::from("notes"),
				"log" => String::from("log"),
				_ => String::from("notes"),
			};

			(notes_page, notes_ipp, log_page, log_ipp, tab)
		});

		notes_query_page.set(if notes_page > 0 { notes_page } else { 1 });
		notes_query_ipp.set(if notes_ipp > 0 { notes_ipp } else { 1 });
		log_query_page.set(if log_page > 0 { log_page } else { 1 });
		log_query_ipp.set(if log_ipp > 0 { log_ipp } else { 1 });
		tab_query.set(tab);
	});

	let login_action = use_context::<LoginAction>().expect("No login action found in context");
	let user_signal = use_context::<UserSignal>().expect("No user signal found in context");

	let location_action = create_server_action::<EditLocation>();
	let notes_action = create_server_action::<EditNotes>();

	let equipment_data = create_resource(
		move || {
			(login_action.version().get(), params.with(|p| p.get("id").cloned().unwrap_or_default()), refetch_resources.get())
		},
		move |(_, id, _)| get_equipment_data_by_id(id),
	);

	let log_data: LogAction = create_resource(
		move || (params.with(|p| p.get("id").cloned().unwrap_or_default()), refetch_resources.get()),
		move |(id, _)| get_log_for_equipment(id, log_query_page.get(), log_query_ipp.get()),
	);

	view! {
		<Suspense fallback=move || view! { <p>Loading equipment...</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors=errors /> }
			}>
				{move || {
					let equipment = {
						move || {
							if equipment_data.get().is_some() {
								match equipment_data.get().unwrap() {
									Err(error) => {
										let error = error.to_string();
										if error.contains("User not authenticated") {
											view! { <Login redirect=format!("/equipment/{}", id.get()) /> }
										} else {
											view! { <pre class="error">Server Error: {error}</pre> }.into_view()
										}
									}
									Ok(equipment) => {
										let title = equipment.name.clone();
										view! {
											<div class=css::details>
												<Heading>
													{match equipment.equipment_type {
														EquipmentType::Flask => view! { <FlaskLogo /> }.into_view(),
														EquipmentType::Vessel => view! { <VesselLogo /> }.into_view(),
														EquipmentType::IncubationCabinet => {
															view! { <IncubationCabinetLogo /> }.into_view()
														}
													}} " " {title}
												</Heading>

												<dl class=css::list>
													<dt>ID</dt>
													<dd>
														<EquipmentCell cell=equipment.id />
													</dd>

													<dt>Name</dt>
													<dd class=css::edit>
														<NameEdit
															equipment=equipment.clone()
															user_signal
															refetch_resources
														/>
													</dd>

													<dt>Equipment Type</dt>
													<dd class=css::edit>
														<TypeEdit
															equipment=equipment.clone()
															user_signal
															refetch_resources
														/>
													</dd>

													<dt>Created By</dt>
													<dd>
														<EquipmentCell cell=equipment.person.clone() />
													</dd>

													<dt>Qrcode</dt>
													<dd>
														<EquipmentCell cell=equipment.qrcode.clone() />
													</dd>

													<dt>Create Date</dt>
													<dd>
														<EquipmentCell cell=equipment.create_date />
													</dd>

													<dt>Status</dt>
													<dd class=css::edit>
														<StatusEdit
															equipment=equipment.clone()
															user_signal
															refetch_resources
														/>
													</dd>

													<dt>Manufacturer</dt>
													<dd class=css::edit>
														<ManufacturerEdit
															equipment=equipment.clone()
															user_signal
															refetch_resources
														/>
													</dd>

													<dt>Purchase Date</dt>
													<dd class=css::edit>
														<PurchaseDateEdit
															equipment=equipment.clone()
															user_signal
															refetch_resources
														/>
													</dd>

													<dt>Vendor</dt>
													<dd class=css::edit>
														<VendorEdit
															equipment=equipment.clone()
															user_signal
															refetch_resources
														/>
													</dd>

													<dt>Cost</dt>
													<dd class=css::edit>
														<CostEdit
															equipment=equipment.clone()
															user_signal
															refetch_resources
														/>
													</dd>

													<dt>Warranty Expiration Date</dt>
													<dd class=css::edit>
														<WarrantyExpirationDateEdit
															equipment=equipment.clone()
															user_signal
															refetch_resources
														/>
													</dd>

													<dt>Location</dt>
													<dd class=css::edit>
														<EquipmentFormToggle
															user_id=equipment.person.id
															id=equipment.id
															user_signal=user_signal
															item=equipment.location.clone()
														>
															{
																let location_clone = equipment.location.clone();
																view! {
																	<ActionForm action=location_action class=css::edit_form>
																		<input type="hidden" name="id" value=equipment.id />
																		<Input
																			name="location"
																			value=create_rw_signal(location_clone.unwrap_or_default())
																		/>
																		<TextArea
																			name="note"
																			placeholder="Add a note why you made this change"
																		/>
																		<div class=css::btns>
																			{move || {
																				if let Some(responds) = location_action.value().get() {
																					match responds {
																						Ok(_) => {
																							location_action.value().set(None);
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
																			}} <Button kind="submit">Save</Button>
																		</div>
																	</ActionForm>
																}
															}
														</EquipmentFormToggle>
													</dd>

													<dt>Notes</dt>
													<dd class=css::edit>
														<EquipmentFormToggle
															user_id=equipment.person.id
															id=equipment.id
															user_signal=user_signal
															item=equipment.notes.clone()
														>
															{
																let notes_clone = equipment.notes.clone();
																view! {
																	<ActionForm action=notes_action class=css::edit_form>
																		<input type="hidden" name="id" value=equipment.id />
																		<TextArea
																			class=css::notes.to_string()
																			name="notes"
																			value=create_rw_signal(
																				notes_clone.unwrap_or_default().to_string(),
																			)
																		/>
																		<TextArea
																			name="note"
																			placeholder="Add a note why you made this change"
																		/>
																		<div class=css::btns>
																			{move || {
																				if let Some(responds) = notes_action.value().get() {
																					match responds {
																						Ok(_) => {
																							notes_action.value().set(None);
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
																			}} <Button kind="submit">Save</Button>
																		</div>
																	</ActionForm>
																}
															}
														</EquipmentFormToggle>
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
					let id_clone = id;
					let notes_query_page_clone = notes_query_page;
					let notes_query_ipp_clone = notes_query_ipp;
					let log_query_page_clone = log_query_page;
					let log_query_ipp_clone = log_query_ipp;
					let tab_query_clone = tab_query;
					view! {
						<div>
							{equipment}
							<Suspense fallback=move || {
								view! { <A href="/login">"Login"</A> }
							}>
								{move || {
									match user_signal.get() {
										None => view! { <span /> }.into_view(),
										Some(_) => {
											view! {
												<div id="equipment_tab" class=css::tab>
													<form
														class=if tab_query.get() == *"notes" {
															"is-selected"
														} else {
															""
														}
														action=format!("/equipment/{}#equipment_tab", id.get())
														method="GET"
													>
														<input
															type="hidden"
															name="notes_page"
															value=notes_query_page_clone
														/>
														<input
															type="hidden"
															name="notes_items_per_page"
															value=notes_query_ipp_clone
														/>
														<input
															type="hidden"
															name="log_page"
															value=log_query_page_clone
														/>
														<input
															type="hidden"
															name="log_items_per_page"
															value=log_query_ipp_clone
														/>
														<input type="hidden" name="tab" value="notes" />
														<button class=css::btn>Notes</button>
													</form>
													<form
														class=if tab_query.get() == *"log" { "is-selected" } else { "" }
														action=format!("/equipment/{}#equipment_tab", id.get())
														method="GET"
													>
														<input
															type="hidden"
															name="notes_page"
															value=notes_query_page_clone
														/>
														<input
															type="hidden"
															name="notes_items_per_page"
															value=notes_query_ipp_clone
														/>
														<input
															type="hidden"
															name="log_page"
															value=log_query_page_clone
														/>
														<input
															type="hidden"
															name="log_items_per_page"
															value=log_query_ipp_clone
														/>
														<input type="hidden" name="tab" value="log" />
														<button class=css::btn>Log</button>
													</form>
												</div>
												<Show
													when=move || tab_query.get().as_str() == "log"
													fallback=move || {
														view! {
															<Notes
																id=id_clone
																notes_query_page=notes_query_page_clone
																notes_query_ipp=notes_query_ipp_clone
																log_query_page=log_query_page_clone
																log_query_ipp=log_query_ipp_clone
																tab_query=tab_query_clone
															/>
														}
													}
												>
													<Log
														id=id
														notes_query_page=notes_query_page
														notes_query_ipp=notes_query_ipp
														log_query_page=log_query_page
														log_query_ipp=log_query_ipp
														tab_query=tab_query
														log_data=log_data
													/>
												</Show>
											}
												.into_view()
										}
									}
								}}
							</Suspense>
						</div>
					}
				}}
			</ErrorBoundary>
		</Suspense>
	}
}

#[server(prefix = "/api")]
pub async fn edit_location(id: String, location: String, note: String) -> Result<(), ServerFnError> {
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
		sqlx::query_scalar("SELECT location FROM equipment WHERE id = $1").bind(id).fetch_one(&pool).await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		user_id,
		note,
		"location",
		old_value,
		location,
	)
	.execute(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET location = $1 WHERE id = $2", location, id).execute(&pool).await.map(|_| ())?;

	Ok(())
}

#[server(prefix = "/api")]
pub async fn edit_notes(id: String, notes: String, note: String) -> Result<(), ServerFnError> {
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
		sqlx::query_scalar("SELECT notes FROM equipment WHERE id = $1").bind(id).fetch_one(&pool).await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		user_id,
		note,
		"notes",
		old_value,
		notes,
	)
	.execute(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET notes = $1 WHERE id = $2", notes, id).execute(&pool).await.map(|_| ())?;

	Ok(())
}

#[server(prefix = "/api")]
pub async fn get_equipment_data_by_id(id: String) -> Result<EquipmentData, ServerFnError> {
	use crate::{auth::get_user, equipment::EquipmentSQLData, permission::Permissions};

	use sqlx::PgPool;

	let pool = use_context::<PgPool>()
		.ok_or_else::<ServerFnError, _>(|| ServerFnError::ServerError(String::from("Database not initialized")))?;
	let user = get_user().await?;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	match user {
		Some(user) => {
			let Permissions::All {
				read: perm,
				write: _,
				create: _,
			} = user.permission_equipment;
			let person: i32 =
				sqlx::query_scalar("SELECT person FROM equipment WHERE id = $1").bind(id).fetch_one(&pool).await?;
			if !perm.has_permission("read", id, person) {
				return Err(ServerFnError::Request(String::from("User not authenticated")));
			}
		},
		None => return Err(ServerFnError::Request(String::from("User not authenticated"))),
	};

	let equipment_sql_data = sqlx::query_as::<_, EquipmentSQLData>(
		r#"
		SELECT
			equipment.*,
			people.id AS person_id,
			people.status AS person_status,
			people.preferred_name AS person_preferred_name,
			people.picture AS person_picture
		FROM
			equipment
			JOIN people ON equipment.person = people.id
		WHERE equipment.id = $1"#,
	)
	.bind(id)
	.fetch_one(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	Ok(equipment_sql_data.into())
}
