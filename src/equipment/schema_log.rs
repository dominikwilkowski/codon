use crate::equipment::{AvatarData, AvatarSQLData};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{FromRow, Row};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum EquipmentLogType {
	#[default]
	Edit,
	Cleaning,
	Preparation,
	Sterilization,
	Using,
	Dirty,
	Archive,
}

impl EquipmentLogType {
	pub fn parse(input: String) -> Self {
		match input.to_lowercase().as_str() {
			"edit" => EquipmentLogType::Edit,
			"cleaning" => EquipmentLogType::Cleaning,
			"preparation" => EquipmentLogType::Preparation,
			"sterilization" => EquipmentLogType::Sterilization,
			"using" => EquipmentLogType::Using,
			"dirty" => EquipmentLogType::Dirty,
			"archive" => EquipmentLogType::Archive,
			_ => Default::default(),
		}
	}
}

impl std::fmt::Display for EquipmentLogType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			EquipmentLogType::Edit => write!(f, "Edit"),
			EquipmentLogType::Cleaning => write!(f, "Cleaning"),
			EquipmentLogType::Preparation => write!(f, "Preparation"),
			EquipmentLogType::Sterilization => write!(f, "Sterilization"),
			EquipmentLogType::Using => write!(f, "Using"),
			EquipmentLogType::Dirty => write!(f, "Dirty"),
			EquipmentLogType::Archive => write!(f, "Archive"),
		}
	}
}

impl EquipmentLogType {
	pub fn get_fields() -> Vec<String> {
		vec![
			String::from("Edit"),
			String::from("Cleaning"),
			String::from("Preparation"),
			String::from("Sterilization"),
			String::from("Using"),
			String::from("Dirty"),
			String::from("Archive"),
		]
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct EquipmentLogSQLData {
	pub id: i32,
	pub log_type: String,
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
pub struct EquipmentLogData {
	pub id: i32,
	pub log_type: EquipmentLogType,
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

impl EquipmentLogData {
	pub fn get_fields() -> Vec<(String, String)> {
		vec![
			(String::from("id"), String::from("ID")),
			(String::from("log_type"), String::from("Log Type")),
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

impl std::default::Default for EquipmentLogData {
	fn default() -> Self {
		EquipmentLogData {
			id: Default::default(),
			log_type: Default::default(),
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

impl From<EquipmentLogSQLData> for EquipmentLogData {
	fn from(val: EquipmentLogSQLData) -> Self {
		EquipmentLogData {
			id: val.id,
			log_type: EquipmentLogType::parse(val.log_type),
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogPersonSQL {
	pub log: EquipmentLogSQLData,
	pub person: AvatarSQLData,
}

#[cfg(feature = "ssr")]
impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for LogPersonSQL {
	fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
		Ok(LogPersonSQL {
			log: EquipmentLogSQLData {
				id: row.try_get("log_id")?,
				log_type: row.try_get("log_log_type")?,
				equipment: row.try_get("log_equipment")?,
				create_date: row.try_get("log_create_date")?,
				person: row.try_get("log_person")?,
				notes: row.try_get("log_notes")?,
				field: row.try_get("log_field")?,
				old_value: row.try_get("log_old_value")?,
				new_value: row.try_get("log_new_value")?,
				media1: row.try_get("log_media1")?,
				media2: row.try_get("log_media2")?,
				media3: row.try_get("log_media3")?,
				media4: row.try_get("log_media4")?,
				media5: row.try_get("log_media5")?,
				media6: row.try_get("log_media6")?,
				media7: row.try_get("log_media7")?,
				media8: row.try_get("log_media8")?,
				media9: row.try_get("log_media9")?,
				media10: row.try_get("log_media10")?,
			},
			person: AvatarSQLData {
				id: row.try_get("person_id")?,
				status: row.try_get("person_status")?,
				preferred_name: row.try_get("person_preferred_name")?,
				picture: row.try_get("person_picture")?,
			},
		})
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogPerson {
	pub log: EquipmentLogData,
	pub person: AvatarData,
}

impl From<LogPersonSQL> for LogPerson {
	fn from(val: LogPersonSQL) -> Self {
		LogPerson {
			log: val.log.into(),
			person: val.person.into(),
		}
	}
}
