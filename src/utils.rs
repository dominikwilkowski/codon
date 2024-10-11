use crate::equipment::EquipmentType;

pub fn string_to_option(s: String) -> Option<String> {
	if s.is_empty() {
		None
	} else {
		Some(s)
	}
}

pub fn get_equipment_folder_name(equipment_type: EquipmentType, id: i32) -> String {
	let equipment_type_short = match equipment_type {
		EquipmentType::Flask => "F",
		EquipmentType::Vessel => "V",
		EquipmentType::IncubationCabinet => "I",
	};
	format!("{equipment_type_short}-{id}/")
}
