use chrono::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
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
			EquipmentType::IncubationCabinet => write!(f, "Incubation Cabinet"),
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

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum EquipmentStatus {
	#[default]
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
			"working" => EquipmentStatus::Working,
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

impl EquipmentStatus {
	pub fn get_fields() -> Vec<String> {
		vec![
			String::from("Working"),
			String::from("NeedsCleaning"),
			String::from("Preparation"),
			String::from("Sterilization"),
			String::from("Broken"),
			String::from("OutOfCommission"),
		]
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
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct EquipmentSQLData {
	pub id: i32,
	pub equipment_type: String,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentData {
	pub id: i32,
	pub equipment_type: EquipmentType,
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
			(String::from("qrcode"), String::from("QRCode")),
			(String::from("create_date"), String::from("Create")),
			(String::from("name"), String::from("Name")),
			(String::from("status"), String::from("Status")),
			(String::from("manufacturer"), String::from("Manufacturer")),
			(String::from("purchase_date"), String::from("Purchased")),
			(String::from("vendor"), String::from("Vendor")),
			(String::from("cost_in_cent"), String::from("Cost")),
			(
				String::from("warranty_expiration_date"),
				String::from("Warranty Expiration Date"),
			),
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct EquipmentNotesSQLData {
	pub id: i32,
	pub equipment: i32,
	pub create_date: DateTime<Utc>,
	pub person: i32,
	pub notes: String,
	pub media1: Option<String>,
	pub media2: Option<String>,
	pub media3: Option<String>,
	pub media4: Option<String>,
	pub media5: Option<String>,
	pub media6: Option<String>,
	pub media7: Option<String>,
	pub media8: Option<String>,
	pub media9: Option<String>,
	pub media10: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum EquipmentActionType {
	#[default]
	Edit,
	Cleaning,
	Sterilization,
	Preparation,
}

impl EquipmentActionType {
	pub fn parse(input: String) -> Self {
		match input.to_lowercase().as_str() {
			"edit" => EquipmentActionType::Edit,
			"cleaning" => EquipmentActionType::Cleaning,
			"sterilization" => EquipmentActionType::Sterilization,
			"preparation" => EquipmentActionType::Preparation,
			_ => Default::default(),
		}
	}
}

impl std::fmt::Display for EquipmentActionType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			EquipmentActionType::Edit => write!(f, "Edit"),
			EquipmentActionType::Cleaning => write!(f, "Cleaning"),
			EquipmentActionType::Sterilization => write!(f, "Sterilization"),
			EquipmentActionType::Preparation => write!(f, "Preparation"),
		}
	}
}

impl EquipmentActionType {
	pub fn get_fields() -> Vec<String> {
		vec![
			String::from("Edit"),
			String::from("Cleaning"),
			String::from("Sterilization"),
			String::from("Preparation"),
		]
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct EquipmentActionSQLData {
	pub id: i32,
	pub action_type: EquipmentActionType,
	pub equipment: i32,
	pub create_date: DateTime<Utc>,
	pub person: i32,
	pub notes: Option<String>,
	pub field: Option<String>,
	pub old_value: Option<String>,
	pub new_value: Option<String>,
	pub media1: Option<String>,
	pub media2: Option<String>,
	pub media3: Option<String>,
	pub media4: Option<String>,
	pub media5: Option<String>,
	pub media6: Option<String>,
	pub media7: Option<String>,
	pub media8: Option<String>,
	pub media9: Option<String>,
	pub media10: Option<String>,
}
