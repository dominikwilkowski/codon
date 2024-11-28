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
							<A href=format!("/equipment/{}", equipment.id) class="none">
								<EquipmentCell cell=equipment.id table_view=true />
							</A>
						</td>
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("equipment_type"))
					}>
						{
							view! {
								<td class="equipment_listing_equipment_type">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment.equipment_type table_view=true />
									</A>
								</td>
							}
						}
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("person"))
					}>
						{
							let equipment_person = equipment.person.clone();
							view! {
								<td class="equipment_listing_person">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment_person table_view=true />
									</A>
								</td>
							}
						}
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("qrcode"))
					}>
						{
							let equipment_qrcode = equipment.qrcode.clone();
							view! {
								<td class="equipment_listing_qrcode">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment_qrcode table_view=true />
									</A>
								</td>
							}
						}
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("create_date"))
					}>
						{
							view! {
								<td class="equipment_listing_create_date">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment.create_date table_view=true />
									</A>
								</td>
							}
						}
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("name"))
					}>
						{
							let equipment_name = equipment.name.clone();
							view! {
								<td class="equipment_listing_name">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment_name table_view=true />
									</A>
								</td>
							}
						}
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("status"))
					}>
						{
							view! {
								<td class="equipment_listing_status">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment.status table_view=true />
									</A>
								</td>
							}
						}
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("manufacturer"))
					}>
						{
							let equipment_manufacturer = equipment.manufacturer.clone();
							view! {
								<td class="equipment_listing_manufacturer">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment_manufacturer table_view=true />
									</A>
								</td>
							}
						}
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("purchase_date"))
					}>
						{
							view! {
								<td class="equipment_listing_purchase_date">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment.purchase_date table_view=true />
									</A>
								</td>
							}
						}
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("vendor"))
					}>
						{
							let equipment_vendor = equipment.vendor.clone();
							view! {
								<td class="equipment_listing_vendor">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment_vendor table_view=true />
									</A>
								</td>
							}
						}
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("cost_in_cent"))
					}>
						{
							let equipment_cost_in_cent = equipment.cost_in_cent.clone();
							view! {
								<td class="equipment_listing_cost_in_cent">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment_cost_in_cent table_view=true />
									</A>
								</td>
							}
						}
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("warranty_expiration_date"))
					}>
						{
							view! {
								<td class="equipment_listing_warranty_expiration_date">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment.warranty_expiration_date table_view=true />
									</A>
								</td>
							}
						}
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("location"))
					}>
						{
							let equipment_location = equipment.location.clone();
							view! {
								<td class="equipment_listing_location">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment_location table_view=true />
									</A>
								</td>
							}
						}
					</Show>
					<Show when=move || {
						query_filter.get().contains(&String::from("notes"))
					}>
						{
							let equipment_notes = equipment.notes.clone();
							view! {
								<td class="equipment_listing_notes">
									<A href=format!("/equipment/{}", equipment.id) class="none">
										<EquipmentCell cell=equipment_notes table_view=true />
									</A>
								</td>
							}
						}
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
