use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
// #[cfg_attr(feature = "ssr", sqlx(type_name = "equipmenttypes", rename_all = "PascalCase"))]
pub enum EquipmentTypes {
	Flask,
	Vessel,
	IncubationCabinet,
}

impl std::fmt::Display for EquipmentTypes {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			EquipmentTypes::Flask => write!(f, "Flask"),
			EquipmentTypes::Vessel => write!(f, "Vessel"),
			EquipmentTypes::IncubationCabinet => write!(f, "Incubation Cabinet"),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

impl std::fmt::Display for EquipmentStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			EquipmentStatus::Working => write!(f, "Working"),
			EquipmentStatus::NeedsCleaning => write!(f, "Needs Cleaning"),
			EquipmentStatus::Preparation => write!(f, "Preparation"),
			EquipmentStatus::Sterilization => write!(f, "Sterilization"),
			EquipmentStatus::Broken => write!(f, "Broken"),
			EquipmentStatus::OutOfCommission => write!(f, "Out Of Commission"),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct EquipmentData {
	pub id: i32,
	pub equipment_type: EquipmentTypes,
	pub qrcode: String,
	pub create_date: DateTime<Utc>,
	pub name: String,
	pub status: EquipmentStatus,
	pub manufacturer: Option<String>,
	pub purchase_date: Option<DateTime<Utc>>,
	pub vendor: Option<String>,
	pub cost: Option<String>,
	pub warranty_expiration_date: Option<DateTime<Utc>>,
	pub location: Option<String>,
	pub notes: Option<String>,
}

impl EquipmentData {
	pub fn get_fields() -> Vec<String> {
		vec![
			String::from("id"),
			String::from("equipment_type"),
			String::from("qrcode"),
			String::from("create_date"),
			String::from("name"),
			String::from("status"),
			String::from("manufacturer"),
			String::from("purchase_date"),
			String::from("vendor"),
			String::from("cost"),
			String::from("warranty_expiration_date"),
			String::from("location"),
			String::from("notes"),
		]
	}

	pub fn format_date(date: &Option<DateTime<Utc>>) -> String {
		match date {
			Some(d) => d.format("%Y-%m-%d").to_string(),
			None => "N/A".to_string(),
		}
	}
}
