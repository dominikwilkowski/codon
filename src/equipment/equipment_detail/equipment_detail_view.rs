use crate::{
	components::{
		button::{Button, ButtonVariant},
		input::{Input, TextArea},
		select::Select,
	},
	equipment::{
		get_log_for_equipment, EquipmentCell, EquipmentCellView, EquipmentData, EquipmentStatus, EquipmentType, Log, Notes,
	},
	error_template::ErrorTemplate,
	icons::{FlaskLogo, IncubationCabinetLogo, VesselLogo},
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment_details.module.css");

#[component]
pub fn EquipmentDetail() -> impl IntoView {
	let params = use_params_map();
	let query = use_query_map();
	let navigate = use_navigate();

	let notes_query_page = create_rw_signal({
		let page = query.with(|p| p.get("notes_page").cloned().unwrap_or(String::from("1"))).parse::<u16>().unwrap_or(1);
		if page > 0 {
			page
		} else {
			1
		}
	});

	let notes_query_ipp = create_rw_signal({
		let ipp =
			query.with(|p| p.get("notes_items_per_page").cloned().unwrap_or(String::from("25"))).parse::<u8>().unwrap_or(25);
		if ipp > 0 {
			ipp
		} else {
			1
		}
	});

	let log_query_page = create_rw_signal({
		let page = query.with(|p| p.get("log_page").cloned().unwrap_or(String::from("1"))).parse::<u16>().unwrap_or(1);
		if page > 0 {
			page
		} else {
			1
		}
	});

	let log_query_ipp = create_rw_signal({
		let ipp =
			query.with(|p| p.get("log_items_per_page").cloned().unwrap_or(String::from("25"))).parse::<u8>().unwrap_or(25);
		if ipp > 0 {
			ipp
		} else {
			1
		}
	});

	let tab_query = create_rw_signal({
		query.with(|p| match p.get("tab").cloned().unwrap_or(String::from("notes")).as_str() {
			"notes" => String::from("notes"),
			"log" => String::from("log"),
			_ => String::from("notes"),
		})
	});

	let go_to_listing = create_rw_signal(false);
	let id = create_rw_signal(params.with(|p| p.get("id").cloned().unwrap_or_default()));

	create_effect(move |_| {
		if id.get().is_empty() || go_to_listing.get() {
			navigate("/equipment", Default::default());
		}
	});

	let name_action = create_server_action::<EditName>();
	let equipment_type_action = create_server_action::<EditType>();
	let manufacturer_action = create_server_action::<EditManufacturer>();

	let equipment_data = create_resource(
		move || {
			(
				id.get(),
				name_action.version().get(),
				equipment_type_action.version().get(),
				manufacturer_action.version().get(),
			)
		},
		move |_| get_equipment_data_by_id(id.get()),
	);

	let log_data = create_resource(
		move || {
			(
				id.get(),
				name_action.version().get(),
				equipment_type_action.version().get(),
				manufacturer_action.version().get(),
			)
		},
		move |_| get_log_for_equipment(id.get(), log_query_page.get(), log_query_ipp.get()),
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
										view! { <pre class="error">Server Error: {error.to_string()}</pre> }.into_view()
									}
									Ok(equipment) => {
										let is_archived = equipment.status.clone() == EquipmentStatus::Archived;
										view! {
											<div class=css::details>
												<h1 class=css::heading>
													{match equipment.equipment_type {
														EquipmentType::Flask => view! { <FlaskLogo /> }.into_view(),
														EquipmentType::Vessel => view! { <VesselLogo /> }.into_view(),
														EquipmentType::IncubationCabinet => {
															view! { <IncubationCabinetLogo /> }.into_view()
														}
													}} " " {equipment.name.clone()}
												</h1>

												<dl class=css::list>
													<dt>ID</dt>
													<dd>
														<EquipmentCell cell=equipment.id />
													</dd>

													<dt>Name</dt>
													<dd class=css::edit>
														<EquipmentFormToggle item=equipment
															.name
															.clone()>
															{
																let name_clone = equipment.name.clone();
																view! {
																	<ActionForm action=name_action class=css::edit_form>
																		<input type="hidden" name="id" value=equipment.id />
																		<Input name="name" value=create_rw_signal(name_clone) />
																		<TextArea
																			name="note"
																			placeholder="Add a note why you made this change"
																		/>
																		<Button kind="submit">Save</Button>
																	</ActionForm>
																}
															}
														</EquipmentFormToggle>
													</dd>

													<dt>Equipment Type</dt>
													<dd class=css::edit>
														<EquipmentFormToggle item=equipment
															.equipment_type>
															{
																view! {
																	<ActionForm
																		action=equipment_type_action
																		class=css::edit_form
																	>
																		<input type="hidden" name="id" value=equipment.id />
																		<Select name="equipment_type">
																			<option
																				value=format!("{:#?}", EquipmentType::Flask)
																				selected=equipment.equipment_type == EquipmentType::Flask
																			>
																				Flask
																			</option>
																			<option
																				value=format!("{:#?}", EquipmentType::Vessel)
																				selected=equipment.equipment_type == EquipmentType::Vessel
																			>
																				Vessel
																			</option>
																			<option
																				value=format!("{:#?}", EquipmentType::IncubationCabinet)
																				selected=equipment.equipment_type
																					== EquipmentType::IncubationCabinet
																			>
																				IncubationCabinet
																			</option>
																		</Select>
																		<TextArea
																			name="note"
																			placeholder="Add a note why you made this change"
																		/>
																		<Button kind="submit">Save</Button>
																	</ActionForm>
																}
															}
														</EquipmentFormToggle>
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
													<dd class=css::status>
														<EquipmentCell cell=equipment.status.clone() />
														<div class=css::btns>
															<Button variant=ButtonVariant::Outlined>
																"Mark as \""
																{EquipmentStatus::get_next_status(
																		equipment.status.clone(),
																		equipment.equipment_type,
																	)
																	.to_string()}"\""
															</Button>
															<Show when=move || !is_archived>
																<Button variant=ButtonVariant::Outlined>"Archive"</Button>
															</Show>
														</div>
													</dd>

													<dt>Manufacturer</dt>
													<dd class=css::edit>
														<EquipmentFormToggle item=equipment
															.manufacturer
															.clone()>
															{
																let manufacturer_clone = equipment.manufacturer.clone();
																view! {
																	<ActionForm action=manufacturer_action class=css::edit_form>
																		<input type="hidden" name="id" value=equipment.id />
																		<Input
																			name="manufacturer"
																			value=create_rw_signal(
																				manufacturer_clone.unwrap_or_default(),
																			)
																		/>
																		<TextArea
																			name="note"
																			placeholder="Add a note why you made this change"
																		/>
																		<Button kind="submit">Save</Button>
																	</ActionForm>
																}
															}
														</EquipmentFormToggle>
													</dd>

													<dt>Purchase Date</dt>
													<dd class=css::edit>
														<EquipmentCell cell=equipment.purchase_date />
														<Button variant=ButtonVariant::Text>Edit</Button>
													</dd>

													<dt>Vendor</dt>
													<dd class=css::edit>
														<EquipmentCell cell=equipment.vendor />
														<Button variant=ButtonVariant::Text>Edit</Button>
													</dd>

													<dt>Cost</dt>
													<dd class=css::edit>
														<EquipmentCell cell=equipment.cost_in_cent />
														<Button variant=ButtonVariant::Text>Edit</Button>
													</dd>

													<dt>Warranty Expiration Date</dt>
													<dd class=css::edit>
														<EquipmentCell cell=equipment.warranty_expiration_date />
														<Button variant=ButtonVariant::Text>Edit</Button>
													</dd>

													<dt>Location</dt>
													<dd class=css::edit>
														<EquipmentCell cell=equipment.location />
														<Button variant=ButtonVariant::Text>Edit</Button>
													</dd>

													<dt>Notes</dt>
													<dd class=css::edit>
														<EquipmentCell cell=equipment.notes />
														<Button variant=ButtonVariant::Text>Edit</Button>
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
							{equipment} <div id="equipment_tab" class=css::tab>
								<form
									class=if tab_query.get() == *"notes" { "is-selected" } else { "" }
									action=format!("/equipment/{}#equipment_tab", id.get())
									method="GET"
								>
									<input type="hidden" name="notes_page" value=notes_query_page_clone />
									<input type="hidden" name="notes_items_per_page" value=notes_query_ipp_clone />
									<input type="hidden" name="log_page" value=log_query_page_clone />
									<input type="hidden" name="log_items_per_page" value=log_query_ipp_clone />
									<input type="hidden" name="tab" value="notes" />
									<button class=css::btn>Notes</button>
								</form>
								<form
									class=if tab_query.get() == *"log" { "is-selected" } else { "" }
									action=format!("/equipment/{}#equipment_tab", id.get())
									method="GET"
								>
									<input type="hidden" name="notes_page" value=notes_query_page_clone />
									<input type="hidden" name="notes_items_per_page" value=notes_query_ipp_clone />
									<input type="hidden" name="log_page" value=log_query_page_clone />
									<input type="hidden" name="log_items_per_page" value=log_query_ipp_clone />
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
						</div>
					}
				}}
			</ErrorBoundary>
		</Transition>
	}
}

#[component]
pub fn EquipmentFormToggle<T: EquipmentCellView + Clone + 'static>(item: T, children: ChildrenFn) -> impl IntoView {
	let toggle = create_rw_signal(false);

	view! {
		<Show when=move || toggle.get() fallback=move || view! { <EquipmentCell cell=item.clone() /> }>
			{children()}
		</Show>

		<Button variant=ButtonVariant::Text on_click=move |_| toggle.update(|toggle| *toggle = !*toggle)>
			{move || if toggle.get() { "Cancel" } else { "Edit" }}
		</Button>
	}
}

#[server]
pub async fn edit_name(id: String, name: String, note: String) -> Result<(), ServerFnError> {
	use crate::db::ssr::get_db;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let old_value: String =
		sqlx::query_scalar("SELECT name FROM equipment WHERE id = $1").bind(id).fetch_one(get_db()).await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		14, // TODO
		note,
		"name",
		old_value,
		name,
	)
	.execute(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET name = $1 WHERE id = $2", name, id).execute(get_db()).await.map(|_| ())?;

	Ok(())
}

#[server]
pub async fn edit_type(id: String, equipment_type: String, note: String) -> Result<(), ServerFnError> {
	use crate::db::ssr::get_db;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let old_value: String =
		sqlx::query_scalar("SELECT equipment_type FROM equipment WHERE id = $1").bind(id).fetch_one(get_db()).await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		14, // TODO
		note,
		"type",
		old_value,
		equipment_type,
	)
	.execute(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET equipment_type = $1 WHERE id = $2", equipment_type, id)
		.execute(get_db())
		.await
		.map(|_| ())?;

	Ok(())
}

#[server]
pub async fn edit_manufacturer(id: String, manufacturer: String, note: String) -> Result<(), ServerFnError> {
	use crate::db::ssr::get_db;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let old_value: String =
		sqlx::query_scalar("SELECT manufacturer FROM equipment WHERE id = $1").bind(id).fetch_one(get_db()).await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		14, // TODO
		note,
		"manufacturer",
		old_value,
		manufacturer,
	)
	.execute(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET manufacturer = $1 WHERE id = $2", manufacturer, id)
		.execute(get_db())
		.await
		.map(|_| ())?;

	Ok(())
}

#[server]
pub async fn get_equipment_data_by_id(id: String) -> Result<EquipmentData, ServerFnError> {
	use crate::{db::ssr::get_db, equipment::EquipmentSQLData};

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let equipment_sql_data = sqlx::query_as::<_, EquipmentSQLData>("SELECT * FROM equipment WHERE id = $1")
		.bind(id)
		.fetch_one(get_db())
		.await
		.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	Ok(equipment_sql_data.into())
}
