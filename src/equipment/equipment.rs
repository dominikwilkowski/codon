use crate::error_template::ErrorTemplate;

use chrono::NaiveDate;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

stylance::import_style!(css, "equipment.module.css");

#[component]
pub fn Equipment() -> impl IntoView {
	let equipment_data =
		create_resource(move || (), move |_| get_equipment_data(String::from("")));

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
								.map(move |todos| match todos {
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
											equipment
												.into_iter()
												.map(move |equipment| {
													view! { <li>{equipment.id}, {equipment.name}</li> }
												})
												.collect_view()
										}
									}
								})
								.unwrap_or_default()
						}
					};
					view! { <ul>{equipment_list}</ul> }
				}}
			</ErrorBoundary>
		</Transition>
	}
}

#[component]
pub fn EquipmentDetail() -> impl IntoView {
	let params = use_params_map();
	let equipment_data = create_resource(
		move || params.with(|p| p.get("id").cloned().unwrap_or_default()),
		move |id| get_equipment_data(id),
	);

	view! { <h1>Equipment Details</h1> }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
// #[cfg_attr(feature = "ssr", sqlx(type_name = "equipmenttypes", rename_all = "PascalCase"))]
pub enum EquipmentTypes {
	Flask,
	Vessel,
	IncubationCabinet,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
// #[cfg_attr(feature = "ssr", sqlx(type_name = "equipmentstatus", rename_all = "PascalCase"))]
pub enum EquipmentStatus {
	Working,
	NeedsCleaning,
	Preparation,
	Sterilization,
	Broken,
	OutOfCommission,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct EquipmentData {
	pub id: i32,
	pub equipment_type: EquipmentTypes,
	pub qrcode: String,
	pub create_date: NaiveDate,
	pub name: String,
	pub status: EquipmentStatus,
	pub manufacturer: Option<String>,
	pub purchase_date: Option<NaiveDate>,
	pub vendor: Option<String>,
	pub cost: Option<String>,
	pub warranty_expiration_date: Option<NaiveDate>,
	pub location: Option<String>,
	pub notes: Option<String>,
}

#[server]
pub async fn get_equipment_data(
	id: String,
) -> Result<Vec<EquipmentData>, ServerFnError> {
	use crate::db::ssr::get_db;

	sqlx::query_as::<_, EquipmentData>("SELECT * FROM equipment ORDER BY id")
		.fetch_all(get_db())
		.await
		.map_err(|error| ServerFnError::ServerError(error.to_string()))
}
