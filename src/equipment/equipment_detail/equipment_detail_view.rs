use crate::{
	app::{LoginAction, UserSignal},
	components::{
		button::{Button, ButtonVariant},
		datepicker::DatePicker,
		file_input::FileInput,
		input::{Input, MoneyInput, TextArea},
		select::Select,
		timezone_offset::Timezone,
	},
	equipment::{
		get_log_for_equipment, EquipmentCell, EquipmentCellView, EquipmentData, EquipmentLogData, EquipmentStatus,
		EquipmentType, Heading, Log, Notes,
	},
	error_template::ErrorTemplate,
	icons::{FlaskLogo, IncubationCabinetLogo, VesselLogo},
	login::Login,
	permission::Permissions,
};

use leptos::*;
use leptos_router::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use web_sys::{FormData, SubmitEvent};

stylance::import_style!(css, "equipment_details.module.css");

pub type LogAction = Resource<(String, usize), Result<(Vec<EquipmentLogData>, i64), ServerFnError>>;

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
	let refetch_resources = create_rw_signal(0);

	create_effect(move |_| {
		if id.get().is_empty() || go_to_listing.get() {
			navigate("/equipment", Default::default());
		}
	});

	let login_action = use_context::<LoginAction>().expect("No login action found in context");
	let user_signal = use_context::<UserSignal>().expect("No user signal found in context");

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
		move || (login_action.version().get(), id.get(), refetch_resources.get()),
		move |_| get_equipment_data_by_id(id.get()),
	);

	let log_data: LogAction = create_resource(
		move || (id.get(), refetch_resources.get()),
		move |_| get_log_for_equipment(id.get(), log_query_page.get(), log_query_ipp.get()),
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
											go_to_listing.set(true);
											view! { <pre class="error">Server Error: {error}</pre> }.into_view()
										}
									}
									Ok(equipment) => {
										let is_archived = equipment.status == EquipmentStatus::Archived;
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
														<EquipmentFormToggle
															user_id=equipment.person.id
															id=equipment.id
															user_signal=user_signal
															item=equipment.name.clone()
														>
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
																		<div class=css::btns>
																			{move || {
																				if let Some(responds) = name_action.value().get() {
																					match responds {
																						Ok(_) => {
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

													<dt>Equipment Type</dt>
													<dd class=css::edit>
														<EquipmentFormToggle
															user_id=equipment.person.id
															id=equipment.id
															user_signal=user_signal
															item=equipment.equipment_type
														>
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
																		<div class=css::btns>
																			{move || {
																				if let Some(responds) = equipment_type_action.value().get()
																				{
																					match responds {
																						Ok(_) => {
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

													<dt>Created By</dt>
													<dd>
														<EquipmentCell cell=equipment.person.clone() />
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
																		<div class=css::btns>
																			{move || {
																				if let Some(responds) = manufacturer_action.value().get() {
																					match responds {
																						Ok(_) => {
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

													<dt>Purchase Date</dt>
													<dd class=css::edit>
														<EquipmentFormToggle
															user_id=equipment.person.id
															id=equipment.id
															user_signal=user_signal
															item=equipment.purchase_date
														>
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
																		<div class=css::btns>
																			{move || {
																				if let Some(responds) = purchase_date_action.value().get() {
																					match responds {
																						Ok(_) => {
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

													<dt>Vendor</dt>
													<dd class=css::edit>
														<EquipmentFormToggle
															user_id=equipment.person.id
															id=equipment.id
															user_signal=user_signal
															item=equipment.vendor.clone()
														>
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
																		<div class=css::btns>
																			{move || {
																				if let Some(responds) = vendor_action.value().get() {
																					match responds {
																						Ok(_) => {
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

													<dt>Cost</dt>
													<dd class=css::edit>
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
																			value=create_rw_signal(
																				cost_in_cent_clone.unwrap_or_default().to_string(),
																			)
																		/>
																		<TextArea
																			name="note"
																			placeholder="Add a note why you made this change"
																		/>
																		<div class=css::btns>
																			{move || {
																				if let Some(responds) = cost_in_cent_action.value().get() {
																					match responds {
																						Ok(_) => {
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

													<dt>Warranty Expiration Date</dt>
													<dd class=css::edit>
														<EquipmentFormToggle
															user_id=equipment.person.id
															id=equipment.id
															user_signal=user_signal
															item=equipment.warranty_expiration_date
														>
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
																		<div class=css::btns>
																			{move || {
																				if let Some(responds) = warranty_expiration_date_action
																					.value()
																					.get()
																				{
																					match responds {
																						Ok(_) => {
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

#[component]
pub fn EquipmentFormToggle<T: EquipmentCellView + Clone + 'static>(
	user_signal: UserSignal,
	user_id: i32,
	id: i32,
	item: T,
	children: ChildrenFn,
) -> impl IntoView {
	let toggle = create_rw_signal(false);

	view! {
		<Show when=move || toggle.get() fallback=move || view! { <EquipmentCell cell=item.clone() /> }>
			{children()}
		</Show>

		<Suspense fallback=move || {
			view! { <A href="/login">"Login"</A> }
		}>
			{move || {
				match user_signal.get() {
					None => view! { <span /> }.into_view(),
					Some(user) => {
						let Permissions::All { read: _, write: perm, create: _ } = user.permission_equipment;
						view! {
							<Show when=move || perm.has_permission("write", id, user_id)>
								<Button
									variant=ButtonVariant::Text
									on_click=move |_| toggle.update(|toggle| *toggle = !*toggle)
								>
									{move || if toggle.get() { "Cancel" } else { "Edit" }}
								</Button>
							</Show>
						}
							.into_view()
					}
				}
			}}
		</Suspense>
	}
}

#[server(prefix = "/api")]
pub async fn edit_name(id: String, name: String, note: String) -> Result<(), ServerFnError> {
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
		sqlx::query_scalar("SELECT name FROM equipment WHERE id = $1").bind(id).fetch_one(&pool).await?;

	sqlx::query(
		r#"
		INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
	)
	.bind("edit")
	.bind(id)
	.bind(user_id)
	.bind(note)
	.bind("name")
	.bind(old_value)
	.bind(name.clone())
	.execute(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query("UPDATE equipment SET name = $1 WHERE id = $2").bind(name).bind(id).execute(&pool).await.map(|_| ())?;

	Ok(())
}

#[server(prefix = "/api")]
pub async fn edit_type(id: String, equipment_type: String, note: String) -> Result<(), ServerFnError> {
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
		sqlx::query_scalar("SELECT equipment_type FROM equipment WHERE id = $1").bind(id).fetch_one(&pool).await?;

	sqlx::query(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
	)
	.bind("edit")
	.bind(id)
	.bind(user_id)
	.bind(note)
	.bind("type")
	.bind(old_value)
	.bind(equipment_type.clone())
	.execute(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query("UPDATE equipment SET equipment_type = $1 WHERE id = $2")
		.bind(equipment_type)
		.bind(id)
		.execute(&pool)
		.await
		.map(|_| ())?;

	Ok(())
}

#[server(input = MultipartFormData, prefix = "/api")]
pub async fn edit_status(data: MultipartData) -> Result<(), ServerFnError> {
	use crate::{
		auth::get_user,
		components::file_upload::{file_upload, remove_temp_files},
		equipment::EquipmentLogType,
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

#[server(prefix = "/api")]
pub async fn edit_purchase_date(
	id: String,
	purchase_date: String,
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
	let purchase_date_with_tz = format!("{}T00:00:00{}", purchase_date, offset_str);

	let purchase_date: DateTime<Utc> = match DateTime::parse_from_str(&purchase_date_with_tz, "%Y-%m-%dT%H:%M:%S%z") {
		Ok(date) => date,
		Err(error) => return Err(ServerFnError::Request(format!("Invalid date: {}", error))),
	}
	.with_timezone(&Utc);

	let old_value: Option<DateTime<Utc>> =
		sqlx::query_scalar("SELECT purchase_date FROM equipment WHERE id = $1").bind(id).fetch_one(&pool).await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		user_id,
		note,
		"purchase_date",
		old_value.unwrap_or_default().format("%d %b %Y").to_string(),
		purchase_date.format("%d %b %Y").to_string(),
	)
	.execute(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET purchase_date = $1 WHERE id = $2", purchase_date, id)
		.execute(&pool)
		.await
		.map(|_| ())?;

	Ok(())
}

#[server(prefix = "/api")]
pub async fn edit_vendor(id: String, vendor: String, note: String) -> Result<(), ServerFnError> {
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
		sqlx::query_scalar("SELECT vendor FROM equipment WHERE id = $1").bind(id).fetch_one(&pool).await?;

	sqlx::query!(
		r#"INSERT INTO equipment_log
		(log_type, equipment, person, notes, field, old_value, new_value)
		VALUES
		($1, $2, $3, $4, $5, $6, $7)"#,
		"edit",
		id,
		user_id,
		note,
		"vendor",
		old_value,
		vendor,
	)
	.execute(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	sqlx::query!("UPDATE equipment SET vendor = $1 WHERE id = $2", vendor, id).execute(&pool).await.map(|_| ())?;

	Ok(())
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
