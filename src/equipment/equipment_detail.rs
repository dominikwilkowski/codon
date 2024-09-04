use crate::{
	equipment::schema::EquipmentData, error_template::ErrorTemplate,
	icons::equipment::EquipmentLogo,
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment_details.module.css");

#[component]
pub fn EquipmentDetail() -> impl IntoView {
	let params = use_params_map();
	let navigate = use_navigate();

	let go_home = create_rw_signal(false);

	create_effect(move |_| {
		if params.with(|p| p.get("id").cloned().unwrap_or_default()).is_empty()
			|| go_home.get()
		{
			navigate("/equipment", Default::default());
		}
	});

	let equipment_data = create_resource(
		move || params.with(|p| p.get("id").cloned().unwrap_or_default()),
		move |id| get_equipment_data_by_id(id),
	);

	view! {
		<h1>
			<EquipmentLogo />
			Equipment Details
		</h1>
		<Transition fallback=move || view! { <p>Loading equipment...</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors=errors /> }
			}>
				{move || {
					let equipment = {
						move || {
							if equipment_data.get().is_some() {
								match equipment_data.get().unwrap() {
									Err(e) => {
										go_home.set(true);
										view! {
											<pre class="error">Server Error: {e.to_string()}</pre>
										}
											.into_view()
									}
									Ok(equipment) => {
										view! {
											<div>
												<h2>{equipment.name.clone()}</h2>
												<img
													src=format!("/qrcodes/{}", equipment.qrcode)
													alt=format!("The QR code for {}", equipment.name)
													class=css::qrcode
												/>
												<dl>
													<dt>ID</dt>
													<dd>{equipment.id}</dd>

													<dt>Name</dt>
													<dd>{equipment.name}</dd>

													<dt>Equipment Type</dt>
													<dd>{equipment.equipment_type.to_string()}</dd>

													<dt>Qrcode</dt>
													<dd>{equipment.qrcode}</dd>

													<dt>Create Date</dt>
													<dd>
														{EquipmentData::format_date(&Some(equipment.create_date))}
													</dd>

													<dt>Status</dt>
													<dd>{equipment.status.to_string()}</dd>

													<dt>Manufacturer</dt>
													<dd>{equipment.manufacturer.unwrap_or_default()}</dd>

													<dt>Purchase Date</dt>
													<dd>
														{EquipmentData::format_date(&equipment.purchase_date)}
													</dd>

													<dt>Vendor</dt>
													<dd>{equipment.vendor.unwrap_or_default()}</dd>

													<dt>Cost</dt>
													<dd>{equipment.cost.unwrap_or_default()}</dd>

													<dt>Warranty Expiration Date</dt>
													<dd>
														{EquipmentData::format_date(
															&equipment.warranty_expiration_date,
														)}
													</dd>

													<dt>Location</dt>
													<dd>{equipment.location.unwrap_or_default()}</dd>

													<dt>Notes</dt>
													<dd>{equipment.notes.unwrap_or_default()}</dd>
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
					view! { <div>{equipment}</div> }
				}}
			</ErrorBoundary>
		</Transition>
	}
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
