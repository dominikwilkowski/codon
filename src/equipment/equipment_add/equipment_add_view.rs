use crate::{equipment::EquipmentData, icons::EquipmentLogo};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment_add.module.css");

#[component]
pub fn EquipmentAdd() -> impl IntoView {
	let add_equipment = create_server_action::<AddEquipment>();

	view! {
		<h2>
			<EquipmentLogo />
			Add new Equipment
		</h2>
		<EquipmentAddEditForm add_equipment />
	}
}

#[component]
pub fn EquipmentAddEditForm(
	#[prop(optional)] is_edit: bool,
	#[prop(optional)] data: EquipmentData,
	add_equipment: Action<AddEquipment, Result<(), ServerFnError>>,
) -> impl IntoView {
	let id_value = create_rw_signal(format!("{}", data.id));
	let equipment_type_value =
		create_rw_signal(format!("{}", data.equipment_type));
	let qrcode_value = create_rw_signal(format!("{}", data.qrcode));
	let create_date_value = create_rw_signal(format!("{}", data.create_date));
	let name_value = create_rw_signal(format!("{}", data.name));
	let status_value = create_rw_signal(format!("{}", data.status));
	let manufacturer_value =
		create_rw_signal(format!("{}", data.manufacturer.unwrap_or_default()));
	let purchase_date_value =
		create_rw_signal(format!("{}", data.purchase_date.unwrap_or_default()));
	let vendor_value =
		create_rw_signal(format!("{}", data.vendor.unwrap_or_default()));
	let cost_value =
		create_rw_signal(format!("{}", data.cost.unwrap_or_default()));
	let warranty_expiration_date_value = create_rw_signal(format!(
		"{}",
		data.warranty_expiration_date.unwrap_or_default()
	));
	let location_value =
		create_rw_signal(format!("{}", data.location.unwrap_or_default()));
	let notes_value =
		create_rw_signal(format!("{}", data.notes.unwrap_or_default()));

	view! {
		<ActionForm action=add_equipment>
			<input type="text" name="id" prop:value=id_value />
			<input
				type="text"
				name="equipment_type"
				prop:value=equipment_type_value
			/>
			<input type="text" name="qrcode" prop:value=qrcode_value />
			<input
				type="text"
				name="create_date"
				prop:value=create_date_value
			/>
			<input type="text" name="name" prop:value=name_value />
			<input type="text" name="status" prop:value=status_value />
			<input
				type="text"
				name="manufacturer"
				prop:value=manufacturer_value
			/>
			<input
				type="text"
				name="purchase_date"
				prop:value=purchase_date_value
			/>
			<input type="text" name="vendor" prop:value=vendor_value />
			<input type="text" name="cost" prop:value=cost_value />
			<input
				type="text"
				name="warranty_expiration_date"
				prop:value=warranty_expiration_date_value
			/>
			<input type="text" name="location" prop:value=location_value />
			<input type="text" name="notes" prop:value=notes_value />
			<button
				type="submit"
				prop:disabled=move || add_equipment.pending().get()
			>
				{if is_edit { "Edit" } else { "Add" }}
			</button>
		</ActionForm>
	}
}

#[server]
pub async fn add_equipment(
	equipment_type: String,
	qrcode: String,
	name: String,
	status: String,
	manufacturer: String,
	purchase_date: String,
	vendor: String,
	cost: String,
	warranty_expiration_date: String,
	location: String,
	notes: String,
) -> Result<(), ServerFnError> {
	use crate::{
		db::ssr::get_db,
		equipment::{Cost, EquipmentStatus, EquipmentTypes, Notes, QRCode},
	};
	use chrono::prelude::*;

	let equipment_type = EquipmentTypes::parse(equipment_type);

	let purchase_date = match purchase_date.parse::<DateTime<Utc>>() {
		Ok(date) => Some(date),
		Err(_) => None,
	};

	let warranty_expiration_date =
		match warranty_expiration_date.parse::<DateTime<Utc>>() {
			Ok(date) => Some(date),
			Err(_) => None,
		};

	Ok(())
	// Ok(
	// 	sqlx::query!(
	// 		"INSERT INTO equipment (equipment_type, qrcode, create_date, name, status, manufacturer, purchase_date, vendor, cost, warranty_expiration_date, location, notes) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
	// 		equipment_type,
	// 		QRCode(qrcode),
	// 		Utc::now(),
	// 		name,
	// 		EquipmentStatus::parse(status),
	// 		manufacturer,
	// 		purchase_date,
	// 		vendor,
	// 		Cost(cost),
	// 		warranty_expiration_date,
	// 		location,
	// 		Notes(notes),
	// 	)
	// 	.execute(get_db())
	// 	.await
	// 	.map(|_| ())?,
	// )
}
