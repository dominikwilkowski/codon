use crate::equipment::{PeopleData, PeopleSQLData};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{FromRow, Row};

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
pub struct EquipmentActionsSQLData {
	pub id: i32,
	pub action_type: String,
	pub equipment: i32,
	pub create_date: DateTime<Utc>,
	pub person: i32,
	pub notes: Option<String>,
	pub field: Option<String>,
	pub old_value: Option<String>,
	pub new_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentActionsData {
	pub id: i32,
	pub action_type: EquipmentActionType,
	pub equipment: i32,
	pub create_date: DateTime<Utc>,
	pub person: i32,
	pub notes: Option<String>,
	pub field: Option<String>,
	pub old_value: Option<String>,
	pub new_value: Option<String>,
}

impl EquipmentActionsData {
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
		]
	}
}

impl std::default::Default for EquipmentActionsData {
	fn default() -> Self {
		EquipmentActionsData {
			id: Default::default(),
			action_type: Default::default(),
			equipment: Default::default(),
			create_date: Utc::now(),
			person: Default::default(),
			notes: None,
			field: None,
			old_value: None,
			new_value: None,
		}
	}
}

impl From<EquipmentActionsSQLData> for EquipmentActionsData {
	fn from(val: EquipmentActionsSQLData) -> Self {
		EquipmentActionsData {
			id: val.id,
			action_type: EquipmentActionType::parse(val.action_type),
			equipment: val.equipment,
			create_date: val.create_date,
			person: val.person,
			notes: val.notes,
			field: val.field,
			old_value: val.old_value,
			new_value: val.new_value,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsPersonSQL {
	pub action: EquipmentActionsSQLData,
	pub person: PeopleSQLData,
}

#[cfg(feature = "ssr")]
impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for ActionsPersonSQL {
	fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
		let action = EquipmentActionsSQLData {
			id: row.try_get("action_id")?,
			action_type: row.try_get("action_action_type")?,
			equipment: row.try_get("action_equipment")?,
			create_date: row.try_get("action_create_date")?,
			person: row.try_get("action_person")?,
			notes: row.try_get("action_notes")?,
			field: row.try_get("action_field")?,
			old_value: row.try_get("action_old_value")?,
			new_value: row.try_get("action_new_value")?,
		};

		let person = PeopleSQLData {
			id: row.try_get("person_id")?,
			employee_id: row.try_get("person_employee_id")?,
			status: row.try_get("person_status")?,
			first_name: row.try_get("person_first_name")?,
			last_name: row.try_get("person_last_name")?,
			preferred_name: row.try_get("person_preferred_name")?,
			email: row.try_get("person_email")?,
			phone_number: row.try_get("person_phone_number")?,
			department: row.try_get("person_department")?,
			role: row.try_get("person_role")?,
			hire_date: row.try_get("person_hire_date")?,
			emergency_contact: row.try_get("person_emergency_contact")?,
			certifications: row.try_get("person_certifications")?,
			specializations: row.try_get("person_specializations")?,
			picture: row.try_get("person_picture")?,
			bio: row.try_get("person_bio")?,
			create_date: row.try_get("person_create_date")?,
		};

		Ok(ActionsPersonSQL { action, person })
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionsPerson {
	pub action: EquipmentActionsData,
	pub person: PeopleData,
}

impl From<ActionsPersonSQL> for ActionsPerson {
	fn from(val: ActionsPersonSQL) -> Self {
		ActionsPerson {
			action: val.action.into(),
			person: val.person.into(),
		}
	}
}
