use crate::{
	components::{
		button::{Button, ButtonVariant},
		datepicker::DatePicker,
		file_input::FileInput,
		input::{Input, MoneyInput, TextArea},
		select::Select,
	},
	equipment::{
		get_log_for_equipment, EquipmentCell, EquipmentCellView, EquipmentData, EquipmentStatus, EquipmentType, Log,
		LogPerson, Notes,
	},
	error_template::ErrorTemplate,
	icons::{FlaskLogo, IncubationCabinetLogo, VesselLogo},
};

use leptos::*;
use leptos_router::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use web_sys::{FormData, SubmitEvent};

stylance::import_style!(css, "equipment_details.module.css");

pub type LogAction = Resource<
	(String, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize),
	Result<(Vec<LogPerson>, i64), ServerFnError>,
>;

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
	let status_action = create_action(|data: &FormData| edit_status(data.clone().into()));
	let manufacturer_action = create_server_action::<EditManufacturer>();
	let purchase_date_action = create_server_action::<EditPurchaseDate>();
	let vendor_action = create_server_action::<EditVendor>();
	let cost_in_cent_action = create_server_action::<EditCostInCent>();
	let warranty_expiration_date_action = create_server_action::<EditWarrantyExpirationDate>();
	let location_action = create_server_action::<EditLocation>();
	let notes_action = create_server_action::<EditNotes>();

	let equipment_data = create_resource(
		move || {
			(
				id.get(),
				name_action.version().get(),
				equipment_type_action.version().get(),
				status_action.version().get(),
				manufacturer_action.version().get(),
				purchase_date_action.version().get(),
				vendor_action.version().get(),
				cost_in_cent_action.version().get(),
				warranty_expiration_date_action.version().get(),
				location_action.version().get(),
				notes_action.version().get(),
			)
		},
		move |_| get_equipment_data_by_id(id.get()),
	);

	let log_data: LogAction = create_resource(
		move || {
			(
				id.get(),
				name_action.version().get(),
				equipment_type_action.version().get(),
				status_action.version().get(),
				manufacturer_action.version().get(),
				purchase_date_action.version().get(),
				vendor_action.version().get(),
				cost_in_cent_action.version().get(),
				warranty_expiration_date_action.version().get(),
				location_action.version().get(),
				notes_action.version().get(),
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
										let is_archived = equipment.status == EquipmentStatus::Archived;
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
																			{EquipmentType::get_fields()
																				.into_iter()
																				.map(|field| {
																					let equipment_type = EquipmentType::parse(field);
																					view! {
																						<option
																							value=format!("{:#?}", equipment_type)
																							selected=equipment.equipment_type == equipment_type
																						>
																							{format!("{equipment_type}")}
																						</option>
																					}
																				})
																				.collect_view()}
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
													<dd class=css::edit>
														<EquipmentFormToggle item=equipment
															.status>
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
																		<input
																			ref=action_ref
																			type="hidden"
																			name="action"
																			value="next_status"
																		/>
																		<TextArea
																			name="note"
																			placeholder="Add a note why you made this change"
																		/>
																		<div class=css::btns>
																			<Show when=move || !media9.get().is_empty()>
																				<FileInput name="media10" value=media10 />
																			</Show>
																			<Show when=move || !media8.get().is_empty()>
																				<FileInput name="media9" value=media9 />
																			</Show>
																			<Show when=move || !media7.get().is_empty()>
																				<FileInput name="media8" value=media8 />
																			</Show>
																			<Show when=move || !media6.get().is_empty()>
																				<FileInput name="media7" value=media7 />
																			</Show>
																			<Show when=move || !media5.get().is_empty()>
																				<FileInput name="media6" value=media6 />
																			</Show>
																			<Show when=move || !media4.get().is_empty()>
																				<FileInput name="media5" value=media5 />
																			</Show>
																			<Show when=move || !media3.get().is_empty()>
																				<FileInput name="media4" value=media4 />
																			</Show>
																			<Show when=move || !media2.get().is_empty()>
																				<FileInput name="media3" value=media3 />
																			</Show>
																			<Show when=move || !media1.get().is_empty()>
																				<FileInput name="media2" value=media2 />
																			</Show>
																			<FileInput name="media1" value=media1 />
																		</div>
																		<div class=css::btns>
																			<span>
																				{move || {
																					if status_action.input().get().is_none()
																						&& status_action.value().get().is_none()
																					{
																						view! {}.into_view()
																					} else if status_action.pending().get() {
																						loading.set(true);
																						view! {}.into_view()
																					} else if let Some(Ok(_)) = status_action.value().get() {
																						loading.set(false);
																						view! { <span class=css::success>Saved successfully</span> }
																							.into_view()
																					} else {
																						loading.set(false);
																						view! {
																							<span>
																								{format!("Error: {:?}", status_action.value().get())}
																							</span>
																						}
																							.into_view()
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
																			<Button
																				kind="submit"
																				loading
																				on:click=move |_| {
																					if let Some(action_element) = action_ref.get() {
																						let _ = action_element
																							.set_attribute("value", "next_status");
																					}
																				}
																			>
																				{if equipment.status == EquipmentStatus::Archived {
																					"Unarchive and mark"
																				} else {
																					"Mark"
																				}}
																				" as \""
																				{EquipmentStatus::get_next_status(
																						equipment.status,
																						equipment.equipment_type,
																					)
																					.to_string()}
																				"\""
																			</Button>
																		</div>
																	</form>
																}
															}
														</EquipmentFormToggle>
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
														<EquipmentFormToggle item=equipment
															.purchase_date>
															{
																let purchase_date_clone = equipment.purchase_date;
																view! {
																	<ActionForm
																		action=purchase_date_action
																		class=css::edit_form
																	>
																		<input type="hidden" name="id" value=equipment.id />
																		<Timezone />
																		<DatePicker
																			attr:name="purchase_date"
																			value=create_rw_signal(
																				Some(purchase_date_clone.unwrap_or_default().date_naive()),
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

													<dt>Vendor</dt>
													<dd class=css::edit>
														<EquipmentFormToggle item=equipment
															.vendor
															.clone()>
															{
																let vendor_clone = equipment.vendor.clone();
																view! {
																	<ActionForm action=vendor_action class=css::edit_form>
																		<input type="hidden" name="id" value=equipment.id />
																		<Input
																			name="vendor"
																			value=create_rw_signal(vendor_clone.unwrap_or_default())
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

													<dt>Cost</dt>
													<dd class=css::edit>
														<EquipmentFormToggle item=equipment
															.cost_in_cent
															.clone()>
															{
																let cost_in_cent_clone = equipment.cost_in_cent.clone();
																view! {
																	<ActionForm action=cost_in_cent_action class=css::edit_form>
																		<input type="hidden" name="id" value=equipment.id />
																		<MoneyInput
																			name="cost_in_cent"
																			value=create_rw_signal(
																				cost_in_cent_clone.unwrap_or_default().to_string(),
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

													<dt>Warranty Expiration Date</dt>
													<dd class=css::edit>
														<EquipmentFormToggle item=equipment
															.warranty_expiration_date>
															{
																let warranty_expiration_date_clone = equipment
																	.warranty_expiration_date;
																view! {
																	<ActionForm
																		action=warranty_expiration_date_action
																		class=css::edit_form
																	>
																		<input type="hidden" name="id" value=equipment.id />
																		<Timezone />
																		<DatePicker
																			attr:name="warranty_expiration_date"
																			value=create_rw_signal(
																				Some(
																					warranty_expiration_date_clone
																						.unwrap_or_default()
																						.date_naive(),
																				),
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

													<dt>Location</dt>
													<dd class=css::edit>
														<EquipmentFormToggle item=equipment
															.location
															.clone()>
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
																		<Button kind="submit">Save</Button>
																	</ActionForm>
																}
															}
														</EquipmentFormToggle>
													</dd>

													<dt>Notes</dt>
													<dd class=css::edit>
														<EquipmentFormToggle item=equipment
															.notes
															.clone()>
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
																		<Button kind="submit">Save</Button>
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
pub fn Timezone() -> impl IntoView {
	view! {
		<input type="hidden" id="timezone_offset" name="timezone_offset" />
		<script>{r#"document.getElementById("timezone_offset").value = new Date().getTimezoneOffset();"#}</script>
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

#[server(input = MultipartFormData)]
pub async fn edit_status(data: MultipartData) -> Result<(), ServerFnError> {
	use crate::{
		components::file_upload::file_upload,
		db::ssr::get_db,
		equipment::{get_folder, EquipmentLogType},
		utils::string_to_option,
	};

	let result = file_upload(data, get_folder).await?;

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
			.fetch_one(get_db())
			.await?;
	let next_status = if action == "next_status" {
		EquipmentStatus::get_next_status(EquipmentStatus::parse(old_status.clone()), EquipmentType::parse(equipment_type))
	} else {
		EquipmentStatus::Archived
	};

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, old_value, media1, media2, media3, media4, media5, media6, media7, media8, media9, media10)
		VALUES
		($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"#,
		EquipmentLogType::from(next_status).to_string(),
		result.id,
		14,
		note.to_string(),
		old_status,
		string_to_option(result.media1.clone()),
		string_to_option(result.media2.clone()),
		string_to_option(result.media3.clone()),
		string_to_option(result.media4.clone()),
		string_to_option(result.media5.clone()),
		string_to_option(result.media6.clone()),
		string_to_option(result.media7.clone()),
		string_to_option(result.media8.clone()),
		string_to_option(result.media9.clone()),
		string_to_option(result.media10.clone()),
	)
	.execute(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET status = $1 WHERE id = $2", format!("{next_status:#?}"), result.id)
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
pub async fn edit_purchase_date(
	id: String,
	purchase_date: String,
	timezone_offset: i32,
	note: String,
) -> Result<(), ServerFnError> {
	use crate::db::ssr::get_db;

	use chrono::prelude::*;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let hours = timezone_offset / 60;
	let minutes = timezone_offset % 60;
	let offset_str = format!("{:+03}:{:02}", hours, minutes.abs());
	let purchase_date_with_tz = format!("{}T00:00:00{}", purchase_date, offset_str);

	let purchase_date: DateTime<Utc> = match DateTime::parse_from_str(&purchase_date_with_tz, "%Y-%m-%dT%H:%M:%S%z") {
		Ok(date) => date,
		Err(error) => return Err(ServerFnError::Request(format!("Invalid date: {}", error))),
	}
	.with_timezone(&Utc);

	let old_value: Option<DateTime<Utc>> =
		sqlx::query_scalar("SELECT purchase_date FROM equipment WHERE id = $1").bind(id).fetch_one(get_db()).await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		14, // TODO
		note,
		"purchase_date",
		old_value.unwrap_or_default().format("%d %b %Y").to_string(),
		purchase_date.format("%d %b %Y").to_string(),
	)
	.execute(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET purchase_date = $1 WHERE id = $2", purchase_date, id)
		.execute(get_db())
		.await
		.map(|_| ())?;

	Ok(())
}

#[server]
pub async fn edit_vendor(id: String, vendor: String, note: String) -> Result<(), ServerFnError> {
	use crate::db::ssr::get_db;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let old_value: String =
		sqlx::query_scalar("SELECT vendor FROM equipment WHERE id = $1").bind(id).fetch_one(get_db()).await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		14, // TODO
		note,
		"vendor",
		old_value,
		vendor,
	)
	.execute(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET vendor = $1 WHERE id = $2", vendor, id).execute(get_db()).await.map(|_| ())?;

	Ok(())
}

#[server]
pub async fn edit_cost_in_cent(id: String, cost_in_cent: f32, note: String) -> Result<(), ServerFnError> {
	use crate::db::ssr::get_db;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let cost_in_cent = (cost_in_cent * 100.0) as i32;

	let old_value: i32 =
		sqlx::query_scalar("SELECT cost_in_cent FROM equipment WHERE id = $1").bind(id).fetch_one(get_db()).await?;
	let old_value = format!("{}", (old_value as f32 / 100.0));

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		14, // TODO
		note,
		"cost_in_cent",
		old_value,
		format!("{:.2}", (cost_in_cent as f32 / 100.0)),
	)
	.execute(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET cost_in_cent = $1 WHERE id = $2", cost_in_cent, id)
		.execute(get_db())
		.await
		.map(|_| ())?;

	Ok(())
}

#[server]
pub async fn edit_warranty_expiration_date(
	id: String,
	warranty_expiration_date: String,
	timezone_offset: i32,
	note: String,
) -> Result<(), ServerFnError> {
	use crate::db::ssr::get_db;

	use chrono::prelude::*;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
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
			.fetch_one(get_db())
			.await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		14, // TODO
		note,
		"warranty_expiration_date",
		old_value.unwrap_or_default().format("%d %b %Y").to_string(),
		warranty_expiration_date.format("%d %b %Y").to_string(),
	)
	.execute(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET warranty_expiration_date = $1 WHERE id = $2", warranty_expiration_date, id)
		.execute(get_db())
		.await
		.map(|_| ())?;

	Ok(())
}

#[server]
pub async fn edit_location(id: String, location: String, note: String) -> Result<(), ServerFnError> {
	use crate::db::ssr::get_db;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let old_value: String =
		sqlx::query_scalar("SELECT location FROM equipment WHERE id = $1").bind(id).fetch_one(get_db()).await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		14, // TODO
		note,
		"location",
		old_value,
		location,
	)
	.execute(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET location = $1 WHERE id = $2", location, id).execute(get_db()).await.map(|_| ())?;

	Ok(())
}

#[server]
pub async fn edit_notes(id: String, notes: String, note: String) -> Result<(), ServerFnError> {
	use crate::db::ssr::get_db;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let old_value: String =
		sqlx::query_scalar("SELECT notes FROM equipment WHERE id = $1").bind(id).fetch_one(get_db()).await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		14, // TODO
		note,
		"notes",
		old_value,
		notes,
	)
	.execute(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET notes = $1 WHERE id = $2", notes, id).execute(get_db()).await.map(|_| ())?;

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
