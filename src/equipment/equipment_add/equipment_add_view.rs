use crate::{
	equipment::{EquipmentStatus, EquipmentType},
	icons::EquipmentLogo,
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment_add.module.css");

#[component]
pub fn EquipmentAdd() -> impl IntoView {
	let add_equipment_action = create_server_action::<AddEquipment>();

	let id_value = create_rw_signal(String::new());
	let equipment_type_value = create_rw_signal(String::new());
	let name_value = create_rw_signal(String::new());
	let status_value = create_rw_signal(String::new());
	let manufacturer_value = create_rw_signal(String::new());
	let purchase_date_value = create_rw_signal(String::new());
	let vendor_value = create_rw_signal(String::new());
	let cost_in_cent_value = create_rw_signal(String::new());
	let warranty_expiration_date_value = create_rw_signal(String::new());
	let location_value = create_rw_signal(String::new());
	let notes_value = create_rw_signal(String::new());

	let navigate = use_navigate();
	create_effect(move |_| {
		if let Some(submission) = add_equipment_action.value().get() {
			if submission.is_ok() {
				println!("submit done!");
				navigate("/equipment", NavigateOptions::default());
			} else {
				println!("{submission:?}");
			}
		}
	});

	view! {
		<h2>
			<EquipmentLogo />
			" Add new Equipment"
		</h2>
		<ActionForm action=add_equipment_action>
			<input type="hidden" name="id" prop:value=id_value />
			<select name="equipment_type" prop:value=equipment_type_value required>
				{EquipmentType::get_fields()
					.iter()
					.map(|name| view! { <option value=name>{name}</option> })
					.collect_view()}
			</select>
			<input type="text" name="name" prop:value=name_value placeholder="Name" required />
			<select name="status" prop:value=status_value required>
				{EquipmentStatus::get_fields()
					.iter()
					.map(|name| view! { <option value=name>{name}</option> })
					.collect_view()}
			</select>
			<input type="text" name="manufacturer" prop:value=manufacturer_value placeholder="Manufacturer" />
			<input type="text" name="purchase_date" prop:value=purchase_date_value placeholder="Purchase Date" />
			<input type="text" name="vendor" prop:value=vendor_value placeholder="Vendor" />
			<input type="number" step="0.01" name="cost_in_cent" prop:value=cost_in_cent_value placeholder="Cost" />
			<input
				type="text"
				name="warranty_expiration_date"
				prop:value=warranty_expiration_date_value
				placeholder="Warranty Expiration Date"
			/>
			<input type="text" name="location" prop:value=location_value placeholder="Location" />
			<textarea type="text" name="notes" prop:value=notes_value placeholder="Notes" />
			<button type="submit" prop:disabled=move || add_equipment_action.pending().get()>
				"Add"
			</button>
		</ActionForm>
	}
}

#[allow(clippy::too_many_arguments)]
#[server]
pub async fn add_equipment(
	equipment_type: String,
	name: String,
	status: String,
	manufacturer: String,
	purchase_date: String,
	vendor: String,
	cost_in_cent: String,
	warranty_expiration_date: String,
	location: String,
	notes: String,
) -> Result<(), ServerFnError> {
	use crate::{
		db::ssr::get_db,
		equipment::{EquipmentStatus, EquipmentType},
		qrcode::generate_qr,
	};
	use chrono::prelude::*;
	use std::{fs, path::PathBuf};

	let purchase_date: Option<DateTime<Utc>> = match purchase_date.parse::<DateTime<Utc>>() {
		Ok(date) => Some(date),
		Err(_) => None,
	};

	let warranty_expiration_date: Option<DateTime<Utc>> = match warranty_expiration_date.parse::<DateTime<Utc>>() {
		Ok(date) => Some(date),
		Err(_) => None,
	};

	let cost_in_cent: Option<i32> =
		cost_in_cent.parse::<f64>().ok().map(|cost_in_cent_f64| (cost_in_cent_f64 * 100.0) as i32);

	let row = sqlx::query!(
		"INSERT INTO equipment\
		(equipment_type, create_date, name, status, manufacturer, purchase_date, vendor, cost_in_cent, warranty_expiration_date, location, notes)\
		VALUES\
		($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)\
		RETURNING id",
		EquipmentType::parse(equipment_type.clone()).to_string(),
		Utc::now(),
		name,
		EquipmentStatus::parse(status).to_string(),
		manufacturer,
		purchase_date,
		vendor,
		cost_in_cent,
		warranty_expiration_date,
		location,
		notes,
	)
	.fetch_one(get_db())
	.await
	.map_err::<ServerFnError, _>(|error| {
		ServerFnError::ServerError(error.to_string())
	})?;

	let qr_svg = generate_qr(&format!("https://codon.com/equipment/{}", row.id))
		.map_err::<ServerFnError, _>(|_| ServerFnError::ServerError("Failed to generate QR code".into()))?;

	let equipment_type_short = match EquipmentType::parse(equipment_type) {
		EquipmentType::Flask => "F",
		EquipmentType::Vessel => "V",
		EquipmentType::IncubationCabinet => "I",
	};
	let qrcode_path = format!("qr_{:06}_{}.svg", row.id, equipment_type_short);
	let file_path = PathBuf::from(format!("{}/public/qrcodes/equipment/{}", env!("CARGO_MANIFEST_DIR"), qrcode_path));

	fs::write(&file_path, qr_svg)
		.map_err::<ServerFnError, _>(|_| ServerFnError::ServerError("Failed to save QR code to file".into()))?;

	Ok(
		sqlx::query!("UPDATE equipment SET qrcode = $1 WHERE id = $2", qrcode_path, row.id)
			.execute(get_db())
			.await
			.map(|_| ())?,
	)
}

// #[allow(clippy::too_many_arguments)]
// #[server]
// pub async fn edit_equipment(
// 	id: String,
// 	equipment_type: String,
// 	name: String,
// 	status: String,
// 	manufacturer: String,
// 	purchase_date: String,
// 	vendor: String,
// 	cost_in_cent: String,
// 	warranty_expiration_date: String,
// 	location: String,
// 	notes: String,
// ) -> Result<(), ServerFnError> {
// 	use crate::{
// 		db::ssr::get_db,
// 		equipment::{EquipmentStatus, EquipmentType},
// 	};
// 	use chrono::prelude::*;

// 	let id: i32 = id.parse::<i32>()?;

// 	let purchase_date: Option<DateTime<Utc>> = match purchase_date.parse::<DateTime<Utc>>() {
// 		Ok(date) => Some(date),
// 		Err(_) => None,
// 	};

// 	let warranty_expiration_date: Option<DateTime<Utc>> = match warranty_expiration_date.parse::<DateTime<Utc>>() {
// 		Ok(date) => Some(date),
// 		Err(_) => None,
// 	};

// 	let cost_in_cent: Option<i32> =
// 		cost_in_cent.parse::<f64>().ok().map(|cost_in_cent_f64| (cost_in_cent_f64 * 100.0) as i32);

// 	Ok(
// 		sqlx::query!(
// 			"UPDATE equipment SET
// 			equipment_type = $1,
// 			name = $2,
// 			status = $3,
// 			manufacturer = $4,
// 			purchase_date = $5,
// 			vendor = $6,
// 			cost_in_cent = $7,
// 			warranty_expiration_date = $8,
// 			location = $9,
// 			notes = $10
// 		WHERE id = $11",
// 			EquipmentType::parse(equipment_type.clone()).to_string(),
// 			name,
// 			EquipmentStatus::parse(status).to_string(),
// 			manufacturer,
// 			purchase_date,
// 			vendor,
// 			cost_in_cent,
// 			warranty_expiration_date,
// 			location,
// 			notes,
// 			id,
// 		)
// 		.execute(get_db())
// 		.await
// 		.map(|_| ())?,
// 	)
// }
