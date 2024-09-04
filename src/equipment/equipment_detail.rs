use crate::{equipment::schema::EquipmentData, error_template::ErrorTemplate};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment.module.css");

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
