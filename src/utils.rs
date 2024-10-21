#[cfg(feature = "ssr")]
use tokio::fs::rename;

fn get_bounds(id: i32) -> (i64, i64) {
	let lower_bound: i64 = (id as i64 / 5_000) * 5_000;
	let upper_bound: i64 = lower_bound + 5_000;

	(lower_bound / 1_000, upper_bound / 1_000)
}

pub fn get_equipment_base_folder(id: i32) -> String {
	let (lower_bound, upper_bound) = get_bounds(id);
	format!("/upload_media/equipment/{lower_bound}-{upper_bound}k/{id}/")
}

#[test]
fn test_get_equipment_base_folder() {
	assert_eq!(get_equipment_base_folder(2), String::from("/upload_media/equipment/0-5k/2/"));
	assert_eq!(get_equipment_base_folder(50), String::from("/upload_media/equipment/0-5k/50/"));
	assert_eq!(get_equipment_base_folder(4999), String::from("/upload_media/equipment/0-5k/4999/"));
	assert_eq!(get_equipment_base_folder(5000), String::from("/upload_media/equipment/5-10k/5000/"));
	assert_eq!(get_equipment_base_folder(28_999), String::from("/upload_media/equipment/25-30k/28999/"));
	assert_eq!(
		get_equipment_base_folder(2_147_483_647),
		String::from("/upload_media/equipment/2147480-2147485k/2147483647/")
	);
}

pub fn get_equipment_notes_folder(id: i32) -> String {
	let (lower_bound, upper_bound) = get_bounds(id);
	format!("notes/{lower_bound}-{upper_bound}k/")
}

#[test]
fn test_get_equipment_notes_folder() {
	assert_eq!(get_equipment_notes_folder(2), String::from("notes/0-5k/"));
	assert_eq!(get_equipment_notes_folder(50), String::from("notes/0-5k/"));
	assert_eq!(get_equipment_notes_folder(4999), String::from("notes/0-5k/"));
	assert_eq!(get_equipment_notes_folder(5000), String::from("notes/5-10k/"));
	assert_eq!(get_equipment_notes_folder(28_999), String::from("notes/25-30k/"));
	assert_eq!(get_equipment_notes_folder(2_147_483_647), String::from("notes/2147480-2147485k/"));
}

pub fn get_equipment_log_folder(id: i32) -> String {
	let (lower_bound, upper_bound) = get_bounds(id);
	format!("log/{lower_bound}-{upper_bound}k/")
}

#[test]
fn test_get_equipment_log_folder() {
	assert_eq!(get_equipment_log_folder(2), String::from("log/0-5k/"));
	assert_eq!(get_equipment_log_folder(50), String::from("log/0-5k/"));
	assert_eq!(get_equipment_log_folder(4999), String::from("log/0-5k/"));
	assert_eq!(get_equipment_log_folder(5000), String::from("log/5-10k/"));
	assert_eq!(get_equipment_log_folder(28_999), String::from("log/25-30k/"));
	assert_eq!(get_equipment_log_folder(2_147_483_647), String::from("log/2147480-2147485k/"));
}

#[cfg(feature = "ssr")]
pub async fn move_file(from: String, to: &str) -> Result<Option<String>, std::io::Error> {
	if from.is_empty() {
		Ok(None)
	} else {
		let new_path = from.replace("temp/", to);
		rename(format!("public{from}"), format!("public{new_path}")).await?;
		Ok(Some(new_path))
	}
}
