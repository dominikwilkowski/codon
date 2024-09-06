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

impl EquipmentTypes {
	pub fn parse(input: String) -> Self {
		match input.to_lowercase().as_str() {
			"flask" => EquipmentTypes::Flask,
			"vessel" => EquipmentTypes::Vessel,
			"incubationcabinet" => EquipmentTypes::IncubationCabinet,
			_ => Default::default(),
		}
	}
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

impl std::default::Default for EquipmentTypes {
	fn default() -> Self {
		EquipmentTypes::Flask
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

impl EquipmentStatus {
	pub fn parse(input: String) -> Self {
		match input.to_lowercase().as_str() {
			"Working" => EquipmentStatus::Working,
			"needscleaning" => EquipmentStatus::NeedsCleaning,
			"preparation" => EquipmentStatus::Preparation,
			"sterilization" => EquipmentStatus::Sterilization,
			"broken" => EquipmentStatus::Broken,
			"outofcommission" => EquipmentStatus::OutOfCommission,
			_ => Default::default(),
		}
	}
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

impl std::default::Default for EquipmentStatus {
	fn default() -> Self {
		EquipmentStatus::Working
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

impl std::default::Default for EquipmentData {
	fn default() -> Self {
		EquipmentData {
			id: Default::default(),
			equipment_type: Default::default(),
			qrcode: Default::default(),
			create_date: Utc::now(),
			name: Default::default(),
			status: Default::default(),
			manufacturer: None,
			purchase_date: None,
			vendor: None,
			cost: None,
			warranty_expiration_date: None,
			location: None,
			notes: None,
		}
	}
}
