use crate::{
	components::pagination::pagination::{
		ItemsPerPage, PaginationNext, PaginationPrev,
	},
	equipment::schema::EquipmentData,
	error_template::ErrorTemplate,
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment.module.css");

#[component]
pub fn Equipment() -> impl IntoView {
	let query = use_query_map();
	let query_field = create_rw_signal(
		query.with(|p| p.get("field").cloned().unwrap_or(String::from("id"))),
	);
	let query_order = create_rw_signal(
		query.with(|p| p.get("order").cloned().unwrap_or(String::from("asc"))),
	);
	let query_page = create_rw_signal(
		query
			.with(|p| p.get("page").cloned().unwrap_or(String::from("1")))
			.parse::<u16>()
			.unwrap_or(1),
	);
	let query_ipp = create_rw_signal(
		query
			.with(|p| p.get("items_per_page").cloned().unwrap_or(String::from("25")))
			.parse::<u8>()
			.unwrap_or(25),
	);
	let items = create_rw_signal(0);

	let equipment_data = create_resource(
		move || (),
		move |_| {
			get_equipment_data(
				query_field.get(),
				query_order.get(),
				query_page.get(),
				query_ipp.get(),
			)
		},
	);

	view! {
		<h1>Equipment</h1>
		<Transition fallback=move || view! { <p>"Loading equipment..."</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors=errors /> }
			}>
				{move || {
					let equipment_list = {
						move || {
							equipment_data
								.get()
								.map(move |data| match data {
									Err(e) => {
										view! {
											<pre class="error">"Server Error: " {e.to_string()}</pre>
										}
											.into_view()
									}
									Ok(equipment) => {
										if equipment.is_empty() {
											view! { <p>"No equipment found."</p> }.into_view()
										} else {
											items.set(equipment.len());
											equipment
												.into_iter()
												.map(move |equipment| {
													view! {
														<tr>
															<td>{equipment.id}</td>
															<td>{equipment.equipment_type.to_string()}</td>
															<td>{equipment.qrcode}</td>
															<td>
																{EquipmentData::format_date(&Some(equipment.create_date))}
															</td>
															<td>{equipment.name}</td>
															<td>{equipment.status.to_string()}</td>
															<td>{equipment.manufacturer.unwrap_or_default()}</td>
															<td>
																{EquipmentData::format_date(&equipment.purchase_date)}
															</td>
															<td>{equipment.vendor.unwrap_or_default()}</td>
															<td>{equipment.cost.unwrap_or_default()}</td>
															<td>
																{EquipmentData::format_date(
																	&equipment.warranty_expiration_date,
																)}
															</td>
															<td>{equipment.location.unwrap_or_default()}</td>
															<td>{equipment.notes.unwrap_or_default()}</td>
															<td>
																<A href=format!("/equipment/{}", equipment.id)>Details</A>
															</td>
														</tr>
													}
												})
												.collect_view()
										}
									}
								})
								.unwrap_or_default()
						}
					};
					let head = EquipmentData::get_fields()
						.iter()
						.map(move |name| {
							let label = if query_field.get() == *name {
								"*↕️"
							} else {
								"↕️"
							};
							let order = if query_field.get() == *name
								&& query_order.get() == "asc"
							{
								"desc"
							} else {
								"asc"
							};
							view! {
								<th>
									{name} <form action="/equipment" method="get">
										<input type="hidden" name="field" value=name />
										<input type="hidden" name="order" value=order />
										<input type="hidden" name="page" value=query_page.get() />
										<input
											type="hidden"
											name="items_per_page"
											value=query_ipp.get()
										/>
										<button type="submit">{label}</button>
									</form>
								</th>
							}
						})
						.collect_view();
					view! {
						<ItemsPerPage
							action="/equipment"
							query_page=query_page
							query_ipp=query_ipp
						>
							<input type="hidden" name="field" value=query_field.get() />
							<input type="hidden" name="order" value=query_order.get() />
						</ItemsPerPage>
						<PaginationNext
							action="/equipment"
							query_page=query_page
							query_ipp=query_ipp
							items=items
						>
							<input type="hidden" name="field" value=query_field.get() />
							<input type="hidden" name="order" value=query_order.get() />
						</PaginationNext>
						<PaginationPrev
							action="/equipment"
							query_page=query_page
							query_ipp=query_ipp
						>
							<input type="hidden" name="field" value=query_field.get() />
							<input type="hidden" name="order" value=query_order.get() />
						</PaginationPrev>
						<table>
							<thead>
								<tr>{head}<th></th></tr>
							</thead>
							<tbody>{equipment_list}</tbody>
						</table>
					}
				}}
			</ErrorBoundary>
		</Transition>
	}
}

#[component]
pub fn EquipmentDetail() -> impl IntoView {
	let params = use_params_map();
	let navigate = use_navigate();

	create_effect(move |_| {
		if params.with(|p| p.get("id").cloned().unwrap_or_default()).is_empty() {
			navigate("/equipment", Default::default());
		}
	});

	let equipment_data = create_resource(
		move || params.with(|p| p.get("id").cloned().unwrap_or_default()),
		move |id| get_equipment_data_by_id(id),
	);

	view! {
		<h1>Equipment Details</h1>
		<Transition fallback=move || view! { <p>"Loading equipment..."</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors=errors /> }
			}>
				{move || {
					let equipment = {
						move || {
							if equipment_data.get().is_some() {
								match equipment_data.get().unwrap() {
									Err(e) => {
										view! {
											<pre class="error">"Server Error: " {e.to_string()}</pre>
										}
											.into_view()
									}
									Ok(equipment) => {
										view! { <p>{equipment.id}, {equipment.name}</p> }
											.into_view()
									}
								}
							} else {
								view! { <div>Nothing found</div> }.into_view()
							}
						}
					};
					view! { <div>{equipment}</div> }
				}}
			</ErrorBoundary>
		</Transition>
	}
}

#[server]
pub async fn get_equipment_data(
	field: String,
	order: String,
	page: u16,
	items_per_page: u8,
) -> Result<Vec<EquipmentData>, ServerFnError> {
	use crate::db::ssr::get_db;

	let order_sanitized = match order.to_lowercase().as_str() {
		"asc" => "ASC",
		"desc" => "DESC",
		_ => "ASC",
	};

	let field_sanitized = match field.to_lowercase().as_str() {
		ref f @ "id"
		| ref f @ "equipment_type"
		| ref f @ "qrcode"
		| ref f @ "create_date"
		| ref f @ "name"
		| ref f @ "status"
		| ref f @ "manufacturer"
		| ref f @ "purchase_date"
		| ref f @ "vendor"
		| ref f @ "cost"
		| ref f @ "warranty_expiration_date"
		| ref f @ "location"
		| ref f @ "notes" => String::from(*f),
		_ => String::from("id"),
	};

	let limit = items_per_page as i64;
	let offset = (page as i64 - 1) * items_per_page as i64;

	let query = format!(
		"SELECT * FROM equipment ORDER BY {field_sanitized} {order_sanitized} LIMIT $1 OFFSET $2",
	);
	sqlx::query_as::<_, EquipmentData>(&query)
		.bind(limit)
		.bind(offset)
		.fetch_all(get_db())
		.await
		.map_err(|error| ServerFnError::ServerError(error.to_string()))
}

#[server]
pub async fn get_equipment_data_by_id(
	id: String,
) -> Result<EquipmentData, ServerFnError> {
	use crate::db::ssr::get_db;

	let id = match id.parse::<i32>() {
		Ok(value) => value,
		Err(_) => return Err(ServerFnError::Request(String::from("Invalid ID"))),
	};

	sqlx::query_as::<_, EquipmentData>("SELECT * FROM equipment WHERE id = $1")
		.bind(id)
		.fetch_one(get_db())
		.await
		.map_err(|error| ServerFnError::ServerError(error.to_string()))
}
