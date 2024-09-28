use crate::{
	components::button::Button,
	equipment::{Actions, EquipmentCell, EquipmentData, EquipmentStatus, Notes},
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
		let page = query
			.with(|p| p.get("notes_page").cloned().unwrap_or(String::from("1")))
			.parse::<u16>()
			.unwrap_or(1);
		if page > 0 {
			page
		} else {
			1
		}
	});

	let notes_query_ipp = create_rw_signal({
		let ipp = query
			.with(|p| {
				p.get("notes_items_per_page").cloned().unwrap_or(String::from("25"))
			})
			.parse::<u8>()
			.unwrap_or(25);
		if ipp > 0 {
			ipp
		} else {
			1
		}
	});

	let actions_query_page = create_rw_signal({
		let page = query
			.with(|p| p.get("actions_page").cloned().unwrap_or(String::from("1")))
			.parse::<u16>()
			.unwrap_or(1);
		if page > 0 {
			page
		} else {
			1
		}
	});

	let actions_query_ipp = create_rw_signal({
		let ipp = query
			.with(|p| {
				p.get("actions_items_per_page").cloned().unwrap_or(String::from("25"))
			})
			.parse::<u8>()
			.unwrap_or(25);
		if ipp > 0 {
			ipp
		} else {
			1
		}
	});

	let go_to_listing = create_rw_signal(false);
	let id =
		create_rw_signal(params.with(|p| p.get("id").cloned().unwrap_or_default()));

	create_effect(move |_| {
		if id.get().is_empty() || go_to_listing.get() {
			navigate("/equipment", Default::default());
		}
	});

	#[expect(clippy::redundant_closure)]
	let equipment_data =
		create_resource(move || id.get(), move |id| get_equipment_data_by_id(id));

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
										view! {
											<pre class="error">Server Error: {error.to_string()}</pre>
										}
											.into_view()
									}
									Ok(equipment) => {
										let is_archived = equipment.status.clone()
											== EquipmentStatus::Archived;
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

														<Button>
															"Mark as \""
															{EquipmentStatus::get_next_status(
																	equipment.status.clone(),
																	equipment.equipment_type,
																)
																.to_string()}"\""
														</Button>
														<Show when=move || !is_archived>
															<Button>"Archive"</Button>
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
					view! {
						<div>
							{equipment}
							<Notes
								id=id
								notes_query_page=notes_query_page
								notes_query_ipp=notes_query_ipp
								actions_query_page=actions_query_page
								actions_query_ipp=actions_query_ipp
							/>
							<Actions
								id=id
								notes_query_page=notes_query_page
								notes_query_ipp=notes_query_ipp
								actions_query_page=actions_query_page
								actions_query_ipp=actions_query_ipp
							/>
						</div>
					}
				}}
			</ErrorBoundary>
		</Transition>
	}
}

#[server]
pub async fn get_equipment_data_by_id(
	id: String,
) -> Result<EquipmentData, ServerFnError> {
	use crate::{db::ssr::get_db, equipment::EquipmentSQLData};

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	let equipment_sql_data = sqlx::query_as::<_, EquipmentSQLData>(
		"SELECT * FROM equipment WHERE id = $1",
	)
	.bind(id)
	.fetch_one(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| {
		ServerFnError::ServerError(error.to_string())
	})?;

	Ok(equipment_sql_data.into())
}
