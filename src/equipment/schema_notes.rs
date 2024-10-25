use crate::equipment::{AvatarData, AvatarSQLData};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::Row;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentNotesSQLData {
	pub id: i32,
	pub equipment: i32,
	pub create_date: DateTime<Utc>,
	pub person: AvatarSQLData,
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

#[cfg(feature = "ssr")]
impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for EquipmentNotesSQLData {
	fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
		Ok(EquipmentNotesSQLData {
			id: row.try_get("id")?,
			equipment: row.try_get("equipment")?,
			create_date: row.try_get("create_date")?,
			person: AvatarSQLData {
				id: row.try_get("person_id")?,
				status: row.try_get("person_status")?,
				preferred_name: row.try_get("person_preferred_name")?,
				picture: row.try_get("person_picture")?,
			},
			notes: row.try_get("notes")?,
			media1: row.try_get("media1")?,
			media2: row.try_get("media2")?,
			media3: row.try_get("media3")?,
			media4: row.try_get("media4")?,
			media5: row.try_get("media5")?,
			media6: row.try_get("media6")?,
			media7: row.try_get("media7")?,
			media8: row.try_get("media8")?,
			media9: row.try_get("media9")?,
			media10: row.try_get("media10")?,
		})
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentNotesData {
	pub id: i32,
	pub equipment: i32,
	pub create_date: DateTime<Utc>,
	pub person: AvatarData,
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

impl EquipmentNotesData {
	pub fn get_fields() -> Vec<(String, String)> {
		vec![
			(String::from("id"), String::from("ID")),
			(String::from("equipment"), String::from("Equipment")),
			(String::from("create_date"), String::from("Create Date")),
			(String::from("person"), String::from("Person")),
			(String::from("notes"), String::from("Notes")),
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

impl std::default::Default for EquipmentNotesData {
	fn default() -> Self {
		EquipmentNotesData {
			id: Default::default(),
			equipment: Default::default(),
			create_date: Utc::now(),
			person: Default::default(),
			notes: Default::default(),
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

impl From<EquipmentNotesSQLData> for EquipmentNotesData {
	fn from(val: EquipmentNotesSQLData) -> Self {
		EquipmentNotesData {
			id: val.id,
			equipment: val.equipment,
			create_date: val.create_date,
			person: val.person.into(),
			notes: val.notes,
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
