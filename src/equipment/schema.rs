use crate::custom_sql_string_type;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{FromRow, Type};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Type))]
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
#[cfg_attr(feature = "ssr", derive(Type))]
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
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct QRCode(String);
custom_sql_string_type!(QRCode);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Cost(String);
custom_sql_string_type!(Cost);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Notes(String);
custom_sql_string_type!(Notes);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct EquipmentData {
	pub id: i32,
	pub equipment_type: EquipmentTypes,
	pub qrcode: QRCode,
	pub create_date: DateTime<Utc>,
	pub name: String,
	pub status: EquipmentStatus,
	pub manufacturer: Option<String>,
	pub purchase_date: Option<DateTime<Utc>>,
	pub vendor: Option<String>,
	pub cost: Option<Cost>,
	pub warranty_expiration_date: Option<DateTime<Utc>>,
	pub location: Option<String>,
	pub notes: Option<Notes>,
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
}
