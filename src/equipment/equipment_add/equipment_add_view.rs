use crate::{
	components::{
		button::Button,
		datepicker::DatePicker,
		input::{Input, MoneyInput, TextArea},
		select::Select,
		timezone_offset::Timezone,
	},
	equipment::{EquipmentType, Heading},
	icons::EquipmentLogo,
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment_add.module.css");

#[component]
pub fn EquipmentAdd() -> impl IntoView {
	let add_equipment_action = create_server_action::<AddEquipment>();

	let loading = create_rw_signal(false);

	let navigate = use_navigate();
	create_effect(move |_| {
		if let Some(Ok(id)) = add_equipment_action.value().get() {
			navigate(&format!("/equipment/{id}"), NavigateOptions::default());
		}
	});

	create_effect(move |_| {
		if !add_equipment_action.pending().get() {
			loading.set(false);
		}
	});

	view! {
		<Heading hide_new=true>
			<EquipmentLogo />
			" Add new Equipment"
		</Heading>
		<ActionForm action=add_equipment_action class=css::form on:submit=move |_| loading.set(true)>
			<Timezone />

			<label class=css::label>
				<span class=css::text>Equipment Type:</span>
				<span class=css::input>
					<Select name="equipment_type" required=true>
						{EquipmentType::get_fields()
							.iter()
							.map(|name| view! { <option value=name>{name}</option> })
							.collect_view()}
					</Select>
				</span>
			</label>

			<label class=css::label>
				<span class=css::text>Name:</span>
				<span class=css::input>
					<Input name="name" placeholder="Name" required=true />
				</span>
			</label>

			<label class=css::label>
				<span class=css::text>Manufacturer:</span>
				<span class=css::input>
					<Input name="manufacturer" placeholder="Manufacturer" />
				</span>
			</label>

			<label class=css::label>
				<span class=css::text>Purchase Date:</span>
				<span class=css::input>
					<DatePicker attr:name="purchase_date" />
				</span>
			</label>

			<label class=css::label>
				<span class=css::text>Vendor:</span>
				<span class=css::input>
					<Input name="vendor" placeholder="Vendor" />
				</span>
			</label>

			<label class=css::label>
				<span class=css::text>Cost:</span>
				<span class=css::input>
					<MoneyInput name="cost_in_cent" />
				</span>
			</label>

			<label class=css::label>
				<span class=css::text>Warranty Expiration:</span>
				<span class=css::input>
					<DatePicker attr:name="warranty_expiration_date" />
				</span>
			</label>

			<label class=css::label>
				<span class=css::text>Location:</span>
				<span class=css::input>
					<Input name="location" placeholder="Location" />
				</span>
			</label>

			<label class=css::label>
				<span class=css::text>Notes:</span>
				<span class=css::input>
					<TextArea name="notes" placeholder="Notes" />
				</span>
			</label>

			<Button kind="submit" loading=loading>
				Add
			</Button>
		</ActionForm>
	}
}

#[allow(clippy::too_many_arguments)]
#[server]
pub async fn add_equipment(
	timezone_offset: i32,
	equipment_type: String,
	name: String,
	manufacturer: String,
	purchase_date: String,
	vendor: String,
	cost_in_cent: String,
	warranty_expiration_date: String,
	location: String,
	notes: String,
) -> Result<i32, ServerFnError> {
	use crate::{
		db::ssr::get_db,
		equipment::{EquipmentStatus, EquipmentType},
		qrcode::generate_qr,
	};
	use chrono::prelude::*;
	use std::{fs, path::PathBuf};

	let hours = timezone_offset / 60;
	let minutes = timezone_offset % 60;
	let offset_str = format!("{:+03}:{:02}", hours, minutes.abs());
	let purchase_date_with_tz = format!("{}T00:00:00{}", purchase_date, offset_str);
	let warranty_expiration_date_with_tz = format!("{}T00:00:00{}", warranty_expiration_date, offset_str);

	let purchase_date: Option<DateTime<Utc>> = if purchase_date.is_empty() {
		None
	} else {
		Some(
			match DateTime::parse_from_str(&purchase_date_with_tz, "%Y-%m-%dT%H:%M:%S%z") {
				Ok(date) => date,
				Err(error) => return Err(ServerFnError::Request(format!("Invalid purchase_date date: {}", error))),
			}
			.with_timezone(&Utc),
		)
	};

	let warranty_expiration_date: Option<DateTime<Utc>> = if warranty_expiration_date.is_empty() {
		None
	} else {
		Some(
			match DateTime::parse_from_str(&warranty_expiration_date_with_tz, "%Y-%m-%dT%H:%M:%S%z") {
				Ok(date) => date,
				Err(error) => return Err(ServerFnError::Request(format!("Invalid warranty_expiration_date date: {}", error))),
			}
			.with_timezone(&Utc),
		)
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
		EquipmentStatus::Dirty.to_string(),
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

	// TODO: make domain configurable
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

	sqlx::query!("UPDATE equipment SET qrcode = $1 WHERE id = $2", qrcode_path, row.id)
		.execute(get_db())
		.await
		.map(|_| ())?;

	Ok(row.id)
}
