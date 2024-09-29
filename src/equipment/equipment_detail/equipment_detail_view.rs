use crate::{
	components::button::Button,
	equipment::{EquipmentCell, EquipmentData, EquipmentStatus, Log, Notes},
	error_template::ErrorTemplate,
	icons::EquipmentLogo,
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

	#[expect(clippy::redundant_closure)]
	let equipment_data = create_resource(move || id.get(), move |id| get_equipment_data_by_id(id));

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
													<EquipmentLogo />
													" "
													{equipment.name.clone()}
												</h1>

												<dl class=css::list>
													<dt>ID</dt>
													<dd>
														<EquipmentCell cell=equipment.id />
													</dd>

													<dt>Name</dt>
													<dd>
														<EquipmentCell cell=equipment.name />
													</dd>

													<dt>Equipment Type</dt>
													<dd>
														<EquipmentCell cell=equipment.equipment_type.clone() />
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
													<dd>
														<EquipmentCell cell=equipment.status.clone() />

														<Button outlined=true>
															"Mark as \""
															{EquipmentStatus::get_next_status(
																	equipment.status.clone(),
																	equipment.equipment_type,
																)
																.to_string()}"\""
														</Button>
														<Show when=move || !is_archived>
															<Button outlined=true>"Archive"</Button>
														</Show>
													</dd>

													<dt>Manufacturer</dt>
													<dd>
														<EquipmentCell cell=equipment.manufacturer />
													</dd>

													<dt>Purchase Date</dt>
													<dd>
														<EquipmentCell cell=equipment.purchase_date />
													</dd>

													<dt>Vendor</dt>
													<dd>
														<EquipmentCell cell=equipment.vendor />
													</dd>

													<dt>Cost</dt>
													<dd>
														<EquipmentCell cell=equipment.cost_in_cent />
													</dd>

													<dt>Warranty Expiration Date</dt>
													<dd>
														<EquipmentCell cell=equipment.warranty_expiration_date />
													</dd>

													<dt>Location</dt>
													<dd>
														<EquipmentCell cell=equipment.location />
													</dd>

													<dt>Notes</dt>
													<dd>
														<EquipmentCell cell=equipment.notes />
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
								/>
							</Show>
						</div>
					}
				}}
			</ErrorBoundary>
		</Transition>
	}
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
