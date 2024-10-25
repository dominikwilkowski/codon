use crate::equipment::{AvatarData, AvatarSQLData, EquipmentLogType};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::Row;

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum EquipmentType {
	#[default]
	Flask,
	Vessel,
	IncubationCabinet,
}

impl EquipmentType {
	pub fn parse(input: String) -> Self {
		match input.to_lowercase().as_str() {
			"flask" => EquipmentType::Flask,
			"vessel" => EquipmentType::Vessel,
			"incubationcabinet" => EquipmentType::IncubationCabinet,
			_ => Default::default(),
		}
	}
}

impl std::fmt::Display for EquipmentType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			EquipmentType::Flask => write!(f, "Flask"),
			EquipmentType::Vessel => write!(f, "Vessel"),
			EquipmentType::IncubationCabinet => {
				// Alt formatter for how the value looks in SQL
				if f.alternate() {
					write!(f, "IncubationCabinet")
				} else {
					write!(f, "Incubation Cabinet")
				}
			},
		}
	}
}

impl EquipmentType {
	pub fn get_fields() -> Vec<String> {
		vec![
			String::from("Flask"),
			String::from("Vessel"),
			String::from("IncubationCabinet"),
		]
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum EquipmentStatus {
	Cleaned,
	Prepared,
	Sterilized,
	InUse,
	#[default]
	Dirty,
	Archived,
}

impl EquipmentStatus {
	pub fn parse(input: String) -> Self {
		match input.to_lowercase().as_str() {
			"cleaned" => EquipmentStatus::Cleaned,
			"prepared" => EquipmentStatus::Prepared,
			"sterilized" => EquipmentStatus::Sterilized,
			"inuse" => EquipmentStatus::InUse,
			"dirty" => EquipmentStatus::Dirty,
			"archived" => EquipmentStatus::Archived,
			_ => Default::default(),
		}
	}
}

impl std::fmt::Display for EquipmentStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			EquipmentStatus::Cleaned => write!(f, "Cleaned"),
			EquipmentStatus::Prepared => write!(f, "Prepared"),
			EquipmentStatus::Sterilized => write!(f, "Sterilized"),
			EquipmentStatus::InUse => write!(f, "In Use"),
			EquipmentStatus::Dirty => write!(f, "Dirty"),
			EquipmentStatus::Archived => write!(f, "Archived"),
		}
	}
}

impl EquipmentStatus {
	pub fn get_fields() -> Vec<String> {
		vec![
			String::from("Cleaned"),
			String::from("Prepared"),
			String::from("Sterilized"),
			String::from("InUse"),
			String::from("Dirty"),
			String::from("Archived"),
		]
	}

	pub fn get_next_status(current: Self, _etype: EquipmentType) -> Self {
		match current {
			EquipmentStatus::Cleaned => EquipmentStatus::Prepared,
			EquipmentStatus::Prepared => {
				// In the future we may add a Bio Reactor which won't be able to be sterilized
				// if etype == EquipmentType::BioReactor {
				// 	EquipmentStatus::InUse
				// } else {
				EquipmentStatus::Sterilized
				// }
			},
			EquipmentStatus::Sterilized => EquipmentStatus::InUse,
			EquipmentStatus::InUse => EquipmentStatus::Dirty,
			EquipmentStatus::Dirty => EquipmentStatus::Cleaned,
			EquipmentStatus::Archived => EquipmentStatus::Dirty,
		}
	}
}

impl From<EquipmentStatus> for EquipmentLogType {
	fn from(val: EquipmentStatus) -> EquipmentLogType {
		match val {
			EquipmentStatus::Cleaned => EquipmentLogType::Cleaning,
			EquipmentStatus::Prepared => EquipmentLogType::Preparation,
			EquipmentStatus::Sterilized => EquipmentLogType::Sterilization,
			EquipmentStatus::InUse => EquipmentLogType::Using,
			EquipmentStatus::Dirty => EquipmentLogType::Dirty,
			EquipmentStatus::Archived => EquipmentLogType::Archive,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Cost(i32);
impl std::fmt::Display for Cost {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:.2}", self.0 as f64 / 100.0)
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QRCode(String);
display_default_for_string_struct!(QRCode);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notes(String);
display_default_for_string_struct!(Notes);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentSQLData {
	pub id: i32,
	pub equipment_type: String,
	pub person: AvatarSQLData,
	pub qrcode: String,
	pub create_date: DateTime<Utc>,
	pub name: String,
	pub status: String,
	pub manufacturer: Option<String>,
	pub purchase_date: Option<DateTime<Utc>>,
	pub vendor: Option<String>,
	pub cost_in_cent: Option<i32>,
	pub warranty_expiration_date: Option<DateTime<Utc>>,
	pub location: Option<String>,
	pub notes: Option<String>,
}

#[cfg(feature = "ssr")]
impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for EquipmentSQLData {
	fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
		Ok(EquipmentSQLData {
			id: row.try_get("id")?,
			equipment_type: row.try_get("equipment_type")?,
			person: AvatarSQLData {
				id: row.try_get("person_id")?,
				status: row.try_get("person_status")?,
				preferred_name: row.try_get("person_preferred_name")?,
				picture: row.try_get("person_picture")?,
			},
			qrcode: row.try_get("qrcode")?,
			create_date: row.try_get("create_date")?,
			name: row.try_get("name")?,
			status: row.try_get("status")?,
			manufacturer: row.try_get("manufacturer")?,
			purchase_date: row.try_get("purchase_date")?,
			vendor: row.try_get("vendor")?,
			cost_in_cent: row.try_get("cost_in_cent")?,
			warranty_expiration_date: row.try_get("warranty_expiration_date")?,
			location: row.try_get("location")?,
			notes: row.try_get("notes")?,
		})
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentData {
	pub id: i32,
	pub equipment_type: EquipmentType,
	pub person: AvatarData,
	pub qrcode: QRCode,
	pub create_date: DateTime<Utc>,
	pub name: String,
	pub status: EquipmentStatus,
	pub manufacturer: Option<String>,
	pub purchase_date: Option<DateTime<Utc>>,
	pub vendor: Option<String>,
	pub cost_in_cent: Option<Cost>,
	pub warranty_expiration_date: Option<DateTime<Utc>>,
	pub location: Option<String>,
	pub notes: Option<Notes>,
}

impl EquipmentData {
	pub fn get_fields() -> Vec<(String, String)> {
		vec![
			(String::from("id"), String::from("ID")),
			(String::from("equipment_type"), String::from("Type")),
			(String::from("person"), String::from("Person")),
			(String::from("qrcode"), String::from("QRCode")),
			(String::from("create_date"), String::from("Create")),
			(String::from("name"), String::from("Name")),
			(String::from("status"), String::from("Status")),
			(String::from("manufacturer"), String::from("Manufacturer")),
			(String::from("purchase_date"), String::from("Purchased")),
			(String::from("vendor"), String::from("Vendor")),
			(String::from("cost_in_cent"), String::from("Cost")),
			(String::from("warranty_expiration_date"), String::from("Warranty Expiration Date")),
			(String::from("location"), String::from("Location")),
			(String::from("notes"), String::from("Notes")),
		]
	}
}

impl std::default::Default for EquipmentData {
	fn default() -> Self {
		EquipmentData {
			id: Default::default(),
			equipment_type: Default::default(),
			person: Default::default(),
			qrcode: Default::default(),
			create_date: Utc::now(),
			name: Default::default(),
			status: Default::default(),
			manufacturer: None,
			purchase_date: None,
			vendor: None,
			cost_in_cent: None,
			warranty_expiration_date: None,
			location: None,
			notes: None,
		}
	}
}

impl From<EquipmentSQLData> for EquipmentData {
	fn from(val: EquipmentSQLData) -> Self {
		EquipmentData {
			id: val.id,
			equipment_type: EquipmentType::parse(val.equipment_type),
			person: val.person.into(),
			qrcode: QRCode(val.qrcode),
			create_date: val.create_date,
			name: val.name,
			status: EquipmentStatus::parse(val.status),
			manufacturer: val.manufacturer,
			purchase_date: val.purchase_date,
			vendor: val.vendor,
			cost_in_cent: val.cost_in_cent.map(Cost),
			warranty_expiration_date: val.warranty_expiration_date,
			location: val.location,
			notes: val.notes.map(Notes),
		}
	}
}
