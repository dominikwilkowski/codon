use chrono::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

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
	pub action_type: String,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentActionData {
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

impl EquipmentActionData {
	pub fn get_fields() -> Vec<(String, String)> {
		vec![
			(String::from("id"), String::from("ID")),
			(String::from("action_type"), String::from("Action Type")),
			(String::from("equipment"), String::from("Equipment")),
			(String::from("create_date"), String::from("Create Date")),
			(String::from("person"), String::from("Person")),
			(String::from("notes"), String::from("Notes")),
			(String::from("field"), String::from("Field")),
			(String::from("old_value"), String::from("Old Value")),
			(String::from("new_value"), String::from("New Value")),
			(String::from("media1"), String::from("Media 1")),
			(String::from("media2"), String::from("Media 2")),
			(String::from("media3"), String::from("Media 3")),
			(String::from("media4"), String::from("Media 4")),
			(String::from("media5"), String::from("Media 5")),
			(String::from("media6"), String::from("Media 6")),
			(String::from("media7"), String::from("Media 7")),
			(String::from("media8"), String::from("Media 8")),
			(String::from("media9"), String::from("Media 9")),
			(String::from("media10"), String::from("Media 10")),
		]
	}
}

impl std::default::Default for EquipmentActionData {
	fn default() -> Self {
		EquipmentActionData {
			id: Default::default(),
			action_type: Default::default(),
			equipment: Default::default(),
			create_date: Utc::now(),
			person: Default::default(),
			notes: None,
			field: None,
			old_value: None,
			new_value: None,
			media1: None,
			media2: None,
			media3: None,
			media4: None,
			media5: None,
			media6: None,
			media7: None,
			media8: None,
			media9: None,
			media10: None,
		}
	}
}

impl From<EquipmentActionSQLData> for EquipmentActionData {
	fn from(val: EquipmentActionSQLData) -> Self {
		EquipmentActionData {
			id: val.id,
			action_type: EquipmentActionType::parse(val.action_type),
			equipment: val.equipment,
			create_date: val.create_date,
			person: val.person,
			notes: val.notes,
			field: val.field,
			old_value: val.old_value,
			new_value: val.new_value,
			media1: val.media1,
			media2: val.media2,
			media3: val.media3,
			media4: val.media4,
			media5: val.media5,
			media6: val.media6,
			media7: val.media7,
			media8: val.media8,
			media9: val.media9,
			media10: val.media10,
		}
	}
}
