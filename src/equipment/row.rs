use crate::equipment::schema::EquipmentData;

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment.module.css");

#[component]
pub fn Row(equipment: Vec<EquipmentData>) -> impl IntoView {
	equipment
		.into_iter()
		.map(move |equipment| {
			view! {
				<tr>
					<td>{equipment.id}</td>
					<td>{equipment.equipment_type.to_string()}</td>
					<td>{equipment.qrcode}</td>
					<td>
						{EquipmentData::format_date(&Some(equipment.create_date))}
					</td>
					<td>{equipment.name}</td>
					<td>{equipment.status.to_string()}</td>
					<td>{equipment.manufacturer.unwrap_or_default()}</td>
					<td>
						{EquipmentData::format_date(&equipment.purchase_date)}
					</td>
					<td>{equipment.vendor.unwrap_or_default()}</td>
					<td>{equipment.cost.unwrap_or_default()}</td>
					<td>
						{EquipmentData::format_date(
							&equipment.warranty_expiration_date,
						)}
					</td>
					<td>{equipment.location.unwrap_or_default()}</td>
					<td>{equipment.notes.unwrap_or_default()}</td>
					<td>
						<A href=format!("/equipment/{}", equipment.id)>Details</A>
					</td>
					<td>Edit</td>
					<td>Delete</td>
				</tr>
			}
		})
		.collect_view()
}
