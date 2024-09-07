pub mod cell {
	pub mod cell_view;
}
pub use cell::cell_view::{EquipmentCell, EquipmentCellView};

pub mod equipment_view;
pub use equipment_view::{DeleteEquipment, Equipment, GetEquipmentData};

pub mod equipment_detail {
	pub mod equipment_detail_view;
}
pub use equipment_detail::equipment_detail_view::{
	EquipmentDetail, GetEquipmentDataById,
};

pub mod equipment_add {
	pub mod equipment_add_view;
}
pub use equipment_add::equipment_add_view::{
	AddEquipment, EquipmentAdd, EquipmentAddEditForm,
};

pub mod row {
	pub mod row_view;
}
pub use row::row_view::Row;

pub mod schema;
pub use schema::{
	Cost, EquipmentData, EquipmentSQLData, EquipmentStatus, EquipmentType, Notes,
	QRCode,
};
