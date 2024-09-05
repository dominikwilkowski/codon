use crate::equipment::{
	cell::EquipmentCell, equipment::DeleteEquipment, schema::EquipmentData,
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "equipment.module.css");

#[component]
pub fn Row(
	equipment: Vec<EquipmentData>,
	delete_equipment: Action<DeleteEquipment, Result<(), ServerFnError>>,
) -> impl IntoView {
	equipment
		.into_iter()
		.map(move |equipment| {
			view! {
				<tr>
					<td>
						<EquipmentCell cell=equipment.id />
					</td>
					<td>
						<EquipmentCell cell=equipment.equipment_type />
					</td>
					<td>
						<EquipmentCell cell=equipment.qrcode />
					</td>
					<td>
						<EquipmentCell cell=equipment.create_date />
					</td>
					<td>
						<EquipmentCell cell=equipment.name />
					</td>
					<td>
						<EquipmentCell cell=equipment.status />
					</td>
					<td>
						<EquipmentCell cell=equipment.manufacturer />
					</td>
					<td>
						<EquipmentCell cell=equipment.purchase_date />
					</td>
					<td>
						<EquipmentCell cell=equipment.vendor />
					</td>
					<td>
						<EquipmentCell cell=equipment.cost />
					</td>
					<td>
						<EquipmentCell cell=equipment.warranty_expiration_date />
					</td>
					<td>
						<EquipmentCell cell=equipment.location />
					</td>
					<td>
						<EquipmentCell cell=equipment.notes />
					</td>
					<td>
						<A href=format!("/equipment/{}", equipment.id)>Details</A>
					</td>
					<td>Edit</td>
					<td>
						<button on:click=move |_| {
							if web_sys::window()
								.unwrap()
								.confirm_with_message(
									"Are you sure you want to delete this item?",
								)
								.unwrap_or(false)
							{
								delete_equipment
									.dispatch(DeleteEquipment {
										id: equipment.id,
									});
							}
						}>"Delete"</button>
					</td>
				</tr>
			}
		})
		.collect_view()
}
