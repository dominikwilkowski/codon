use crate::{
	// components::button::{Button, ButtonVariant},
	equipment::{/*DeleteEquipment,*/ EquipmentCell, EquipmentData},
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "row.module.css");

#[component]
pub fn Row(
	equipment: Vec<EquipmentData>,
	// delete_equipment: Action<DeleteEquipment, Result<(), ServerFnError>>,
	query_filter: RwSignal<Vec<String>>,
) -> impl IntoView {
	equipment
		.into_iter()
		.map(move |equipment| {
			view! {
				<tr>
					<Show when=move || { query_filter.get().contains(&String::from("id")) }>
						<td class="equipment_listing_id">
							<EquipmentCell cell=equipment.id table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("equipment_type")) }>
						<td class="equipment_listing_equipment_type">
							<EquipmentCell cell=equipment.equipment_type table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("person")) }>
						<td class="equipment_listing_person">
							<EquipmentCell cell=equipment.person.clone() table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("qrcode")) }>
						<td class="equipment_listing_qrcode">
							<EquipmentCell cell=equipment.qrcode.clone() table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("create_date")) }>
						<td class="equipment_listing_create_date">
							<EquipmentCell cell=equipment.create_date table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("name")) }>
						<td class="equipment_listing_name">
							<EquipmentCell cell=equipment.name.clone() table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("status")) }>
						<td class="equipment_listing_status">
							<EquipmentCell cell=equipment.status table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("manufacturer")) }>
						<td class="equipment_listing_manufacturer">
							<EquipmentCell cell=equipment.manufacturer.clone() table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("purchase_date")) }>
						<td class="equipment_listing_purchase_date">
							<EquipmentCell cell=equipment.purchase_date table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("vendor")) }>
						<td class="equipment_listing_vendor">
							<EquipmentCell cell=equipment.vendor.clone() table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("cost_in_cent")) }>
						<td class="equipment_listing_cost_in_cent">
							<EquipmentCell cell=equipment.cost_in_cent.clone() table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("warranty_expiration_date")) }>
						<td class="equipment_listing_warranty_expiration_date">
							<EquipmentCell cell=equipment.warranty_expiration_date table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("location")) }>
						<td class="equipment_listing_location">
							<EquipmentCell cell=equipment.location.clone() table_view=true />
						</td>
					</Show>
					<Show when=move || { query_filter.get().contains(&String::from("notes")) }>
						<td class="equipment_listing_notes">
							<EquipmentCell cell=equipment.notes.clone() table_view=true />
						</td>
					</Show>
					<td class="equipment_listing_details_link">
						<A href=format!("/equipment/{}", equipment.id)>Details</A>
					</td>
				// <td class="equipment_listing_delete_link">
				// <Button
				// variant=ButtonVariant::Text
				// on:click=move |_| {
				// if web_sys::window()
				// .unwrap()
				// .confirm_with_message("Are you sure you want to delete this item?")
				// .unwrap_or(false)
				// {
				// delete_equipment
				// .dispatch(DeleteEquipment {
				// id: equipment.id,
				// });
				// }
				// }
				// >
				// Delete
				// </Button>
				// </td>
				</tr>
			}
		})
		.collect_view()
}
