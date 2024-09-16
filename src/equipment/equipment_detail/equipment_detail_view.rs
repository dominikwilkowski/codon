use crate::{
	equipment::{
		/*EquipmentActionsData,*/ EquipmentCell, EquipmentData,
		EquipmentNotesData,
	},
	error_template::ErrorTemplate,
	icons::EquipmentLogo,
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment_details.module.css");

#[component]
pub fn EquipmentDetail() -> impl IntoView {
	let params = use_params_map();
	let navigate = use_navigate();

	let go_to_listing = create_rw_signal(false);

	create_effect(move |_| {
		if params.with(|p| p.get("id").cloned().unwrap_or_default()).is_empty()
			|| go_to_listing.get()
		{
			navigate("/equipment", Default::default());
		}
	});

	#[expect(clippy::redundant_closure)]
	let equipment_data = create_resource(
		move || params.with(|p| p.get("id").cloned().unwrap_or_default()),
		move |id| get_equipment_data_by_id(id),
	);

	#[expect(clippy::redundant_closure)]
	let notes_data = create_resource(
		move || params.with(|p| p.get("id").cloned().unwrap_or_default()),
		move |id| get_notes_for_equipment(id),
	);

	view! {
		<h1>
			<EquipmentLogo />
			" Equipment Details"
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
									Err(error) => {
										view! {
											go_to_listing.set(true);
											<pre class="error">Server Error: {error.to_string()}</pre>
										}
											.into_view()
									}
									Ok(equipment) => {
										view! {
											<div>
												<A href=format!("/equipment/edit/{}", equipment.id)>Edit</A>
												<h2>{equipment.name.clone()}</h2>
												<img
													src=format!("/qrcodes/{}", equipment.qrcode)
													alt=format!("The QR code for {}", equipment.name)
													class=css::qrcode
												/>
												<dl>
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
														<EquipmentCell cell=equipment.equipment_type />
													</dd>

													<dt>Qrcode</dt>
													<EquipmentCell cell=equipment.qrcode />
													<dd></dd>

													<dt>Create Date</dt>
													<dd>
														<EquipmentCell cell=equipment.create_date />
													</dd>

													<dt>Status</dt>
													<dd>
														<EquipmentCell cell=equipment.status />
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
					let notes = {
						move || {
							if notes_data.get().is_some() {
								match notes_data.get().unwrap() {
									Err(error) => {
										view! {
											<pre class="error">
												Notes Server Error: {error.to_string()}
											</pre>
										}
											.into_view()
									}
									Ok((notes, _)) => {
										notes
											.into_iter()
											.map(|note| {
												view! {
													<div>{note.person} {note.notes}</div>
												}
											})
											.collect_view()
									}
								}
							} else {
								view! {
									<div>No Notes found</div>
								}
									.into_view()
							}
						}
					};
					view! {
						<div>{equipment} <h2>Notes:</h2> {notes}</div>
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

#[server]
pub async fn get_notes_for_equipment(
	id: String,
) -> Result<(Vec<EquipmentNotesData>, i64), ServerFnError> {
	use crate::{db::ssr::get_db, equipment::EquipmentNotesSQLData};

	let notes_sql_data = sqlx::query_as::<_, EquipmentNotesSQLData>(
		"SELECT * FROM equipment_notes WHERE equipment = $1",
	)
	.bind(id.parse::<i32>().expect("Invalid id"))
	.fetch_all(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| {
		ServerFnError::ServerError(error.to_string())
	})?;

	let notes_data: Vec<EquipmentNotesData> =
		notes_sql_data.into_iter().map(Into::into).collect();

	let row_count: i64 =
		sqlx::query_scalar("SELECT COUNT(*) FROM equipment_notes")
			.fetch_one(get_db())
			.await?;

	Ok((notes_data, row_count))
}
