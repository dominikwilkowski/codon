use crate::{
	components::{pagination::pagination::Pagination, table::table::TableHead},
	equipment::{row::Row, schema::EquipmentData},
	error_template::ErrorTemplate,
	icons::equipment::EquipmentLogo,
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
		<h1>
			<EquipmentLogo />
			Equipment
		</h1>
		<Transition fallback=move || view! { <p>Loading equipment...</p> }>
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
											<pre class="error">Server Error: {e.to_string()}</pre>
										}
											.into_view()
									}
									Ok(equipment) => {
										if equipment.is_empty() {
											view! {
												<tr>
													<td colspan=EquipmentData::get_fields()
														.len()>"No equipment found."</td>
												</tr>
											}
												.into_view()
										} else {
											items.set(equipment.len());
											view! { <Row equipment=equipment /> }
										}
									}
								})
								.unwrap_or_default()
						}
					};
					view! {
						<Pagination
							action="/equipment"
							query_page=query_page
							query_ipp=query_ipp
							items=items
						>
							<input type="hidden" name="field" value=query_field.get() />
							<input type="hidden" name="order" value=query_order.get() />
						</Pagination>
						<table>
							<thead>
								<tr>
									<TableHead
										action="/equipment"
										items=EquipmentData::get_fields()
										query_field=query_field
										query_order=query_order
									>
										<input type="hidden" name="page" value=query_page.get() />
										<input
											type="hidden"
											name="items_per_page"
											value=query_ipp.get()
										/>
									</TableHead>
									<th colspan="3"></th>
								</tr>
							</thead>
							<tbody>{equipment_list}</tbody>
						</table>
						<Pagination
							action="/equipment"
							query_page=query_page
							query_ipp=query_ipp
							items=items
						>
							<input type="hidden" name="field" value=query_field.get() />
							<input type="hidden" name="order" value=query_order.get() />
						</Pagination>
					}
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
