use chrono::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum PeopleStatus {
	#[default]
	Active,
	OnLeave,
	Left,
}

impl PeopleStatus {
	pub fn parse(input: String) -> Self {
		match input.to_lowercase().as_str() {
			"active" => PeopleStatus::Active,
			"onleave" => PeopleStatus::OnLeave,
			"left" => PeopleStatus::Left,
			_ => Default::default(),
		}
	}
}

impl std::fmt::Display for PeopleStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PeopleStatus::Active => write!(f, "Active"),
			PeopleStatus::OnLeave => write!(f, "On Leave"),
			PeopleStatus::Left => write!(f, "Left"),
		}
	}
}

impl PeopleStatus {
	pub fn get_fields() -> Vec<String> {
		vec![String::from("Active"), String::from("OnLeave"), String::from("Left")]
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct PeopleSQLData {
	pub id: i32,
	pub employee_id: Option<String>,
	pub status: String,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	pub preferred_name: String,
	pub email: String,
	pub phone_number: Option<String>,
	pub department: Option<String>,
	pub role: Option<String>,
	pub hire_date: Option<DateTime<Utc>>,
	pub emergency_contact: Option<String>,
	pub certifications: Option<String>,
	pub specializations: Option<String>,
	pub picture: Option<String>,
	pub bio: Option<String>,
	pub create_date: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeopleData {
	pub id: i32,
	pub employee_id: Option<String>,
	pub status: PeopleStatus,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	pub preferred_name: String,
	pub email: String,
	pub phone_number: Option<String>,
	pub department: Option<String>,
	pub role: Option<String>,
	pub hire_date: Option<DateTime<Utc>>,
	pub emergency_contact: Option<String>,
	pub certifications: Option<String>,
	pub specializations: Option<String>,
	pub picture: Option<String>,
	pub bio: Option<String>,
	pub create_date: DateTime<Utc>,
}

impl PeopleData {
	pub fn get_fields() -> Vec<(String, String)> {
		vec![
			(String::from("id"), String::from("ID")),
			(String::from("employee_id"), String::from("Employee ID")),
			(String::from("status"), String::from("Status")),
			(String::from("first_name"), String::from("First Name")),
			(String::from("last_name"), String::from("Last Name")),
			(String::from("preferred_name"), String::from("Preferred Name")),
			(String::from("email"), String::from("Email")),
			(String::from("phone_number"), String::from("Phone Number")),
			(String::from("department"), String::from("Department")),
			(String::from("role"), String::from("Role")),
			(String::from("hire_date"), String::from("Hire Date")),
			(String::from("emergency_contact"), String::from("Emergency Contact")),
			(String::from("certifications"), String::from("Certifications")),
			(String::from("specializations"), String::from("Specializations")),
			(String::from("picture"), String::from("Picture")),
			(String::from("bio"), String::from("Bio")),
			(String::from("create_date"), String::from("Created")),
		]
	}
}

impl std::default::Default for PeopleData {
	fn default() -> Self {
		PeopleData {
			id: Default::default(),
			employee_id: None,
			status: Default::default(),
			first_name: None,
			last_name: None,
			preferred_name: Default::default(),
			email: Default::default(),
			phone_number: None,
			department: None,
			role: None,
			hire_date: None,
			emergency_contact: None,
			certifications: None,
			specializations: None,
			picture: None,
			bio: None,
			create_date: Utc::now(),
		}
	}
}

impl From<PeopleSQLData> for PeopleData {
	fn from(val: PeopleSQLData) -> Self {
		PeopleData {
			id: val.id,
			employee_id: val.employee_id,
			status: PeopleStatus::parse(val.status),
			first_name: val.first_name,
			last_name: val.last_name,
			preferred_name: val.preferred_name,
			email: val.email,
			phone_number: val.phone_number,
			department: val.department,
			role: val.role,
			hire_date: val.hire_date,
			emergency_contact: val.emergency_contact,
			certifications: val.certifications,
			specializations: val.specializations,
			picture: val.picture,
			bio: val.bio,
			create_date: val.create_date,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct AvatarSQLData {
	pub id: i32,
	pub status: String,
	pub preferred_name: String,
	pub picture: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AvatarData {
	pub id: i32,
	pub status: PeopleStatus,
	pub preferred_name: String,
	pub picture: Option<String>,
}

impl From<PeopleSQLData> for AvatarData {
	fn from(val: PeopleSQLData) -> Self {
		AvatarData {
			id: val.id,
			status: PeopleStatus::parse(val.status),
			preferred_name: val.preferred_name,
			picture: val.picture,
		}
	}
}

impl From<AvatarSQLData> for AvatarData {
	fn from(val: AvatarSQLData) -> Self {
		AvatarData {
			id: val.id,
			status: PeopleStatus::parse(val.status),
			preferred_name: val.preferred_name,
			picture: val.picture,
		}
	}
}
