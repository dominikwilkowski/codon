use crate::{
	equipment::{EquipmentData, EquipmentStatus, EquipmentType},
	icons::EquipmentLogo,
};

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
		<EquipmentAddEditForm
			submit_action=add_equipment
			redirect_on_success="/equipment"
		/>
	}
}

#[component]
pub fn EquipmentAddEditForm(
	#[prop(optional)] is_edit: bool,
	#[prop(optional)] data: EquipmentData,
	submit_action: Action<AddEquipment, Result<(), ServerFnError>>,
	redirect_on_success: &'static str,
) -> impl IntoView {
	let id_value = create_rw_signal(if !is_edit {
		String::new()
	} else {
		data.id.to_string()
	});
	let equipment_type_value = create_rw_signal(if !is_edit {
		String::new()
	} else {
		data.equipment_type.to_string()
	});
	let name_value =
		create_rw_signal(if !is_edit { String::new() } else { data.name });
	let status_value = create_rw_signal(if !is_edit {
		String::new()
	} else {
		data.status.to_string()
	});
	let manufacturer_value =
		create_rw_signal(if !is_edit || data.manufacturer.is_none() {
			String::new()
		} else {
			data.manufacturer.unwrap_or_default()
		});
	let purchase_date_value =
		create_rw_signal(if !is_edit || data.purchase_date.is_none() {
			String::new()
		} else {
			data.purchase_date.unwrap_or_default().to_string()
		});
	let vendor_value = create_rw_signal(if !is_edit || data.vendor.is_none() {
		String::new()
	} else {
		data.vendor.unwrap_or_default()
	});
	let cost_in_cent_value =
		create_rw_signal(if !is_edit || data.cost_in_cent.is_none() {
			String::new()
		} else {
			data.cost_in_cent.unwrap_or_default().to_string()
		});
	let warranty_expiration_date_value =
		create_rw_signal(if !is_edit || data.warranty_expiration_date.is_none() {
			String::new()
		} else {
			data.warranty_expiration_date.unwrap_or_default().to_string()
		});
	let location_value =
		create_rw_signal(if !is_edit || data.location.is_none() {
			String::new()
		} else {
			data.location.unwrap_or_default()
		});
	let notes_value = create_rw_signal(if !is_edit || data.notes.is_none() {
		String::new()
	} else {
		data.notes.unwrap_or_default().to_string()
	});

	let navigate = use_navigate();
	create_effect(move |_| {
		if let Some(submission) = submit_action.value().get() {
			if submission.is_ok() {
				navigate(redirect_on_success, NavigateOptions::default());
			} else {
				println!("{submission:?}");
			}
		}
	});

	view! {
		<ActionForm action=submit_action>
			<input type="hidden" name="id" prop:value=id_value />
			<select
				name="equipment_type"
				prop:value=equipment_type_value
				required
			>
				{EquipmentType::get_fields()
					.iter()
					.map(|name| view! { <option value=name>{name}</option> })
					.collect_view()}
			</select>
			<input
				type="text"
				name="name"
				prop:value=name_value
				placeholder="Name"
				required
			/>
			<select name="status" prop:value=status_value required>
				{EquipmentStatus::get_fields()
					.iter()
					.map(|name| view! { <option value=name>{name}</option> })
					.collect_view()}
			</select>
			<input
				type="text"
				name="manufacturer"
				prop:value=manufacturer_value
				placeholder="Manufacturer"
			/>
			<input
				type="text"
				name="purchase_date"
				prop:value=purchase_date_value
				placeholder="Purchase Date"
			/>
			<input
				type="text"
				name="vendor"
				prop:value=vendor_value
				placeholder="Vendor"
			/>
			<input
				type="number"
				step="0.01"
				name="cost_in_cent"
				prop:value=cost_in_cent_value
				placeholder="Cost"
			/>
			<input
				type="text"
				name="warranty_expiration_date"
				prop:value=warranty_expiration_date_value
				placeholder="Warranty Expiration Date"
			/>
			<input
				type="text"
				name="location"
				prop:value=location_value
				placeholder="Location"
			/>
			<textarea
				type="text"
				name="notes"
				prop:value=notes_value
				placeholder="Notes"
			/>
			<button
				type="submit"
				prop:disabled=move || submit_action.pending().get()
			>
				{if is_edit { "Edit" } else { "Add" }}
			</button>
		</ActionForm>
	}
}

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
	};
	use chrono::prelude::*;

	let purchase_date: Option<DateTime<Utc>> =
		match purchase_date.parse::<DateTime<Utc>>() {
			Ok(date) => Some(date),
			Err(_) => None,
		};

	let warranty_expiration_date: Option<DateTime<Utc>> =
		match warranty_expiration_date.parse::<DateTime<Utc>>() {
			Ok(date) => Some(date),
			Err(_) => None,
		};

	let cost_in_cent: Option<i32> = cost_in_cent
		.parse::<f64>()
		.ok()
		.map(|cost_in_cent_f64| (cost_in_cent_f64 * 100.0) as i32);

	let row = sqlx::query!(
		"INSERT INTO equipment
		(equipment_type, create_date, name, status, manufacturer, purchase_date, vendor, cost_in_cent, warranty_expiration_date, location, notes)\
		VALUES\
		($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)\
		RETURNING id",
		EquipmentType::parse(equipment_type).to_string(),
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

	// TODO: build qrcode and save to disk
	let qrcode = String::from("path/to/qrcode.svg");

	Ok(
		sqlx::query!(
			"UPDATE equipment SET qrcode = $1 WHERE id = $2",
			qrcode,
			row.id
		)
		.execute(get_db())
		.await
		.map(|_| ())?,
	)
}
