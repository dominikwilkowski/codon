use crate::equipment::{DeleteEquipment, EquipmentCell, EquipmentData};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "row.module.css");

#[component]
pub fn Row(
	equipment: Vec<EquipmentData>,
	delete_equipment: Action<DeleteEquipment, Result<(), ServerFnError>>,
	field_filter: RwSignal<Vec<String>>,
) -> impl IntoView {
	equipment
		.into_iter()
		.map(move |equipment| {
			view! {
				<tr>
					<Show when=move || {
						field_filter.get().contains(&String::from("id"))
					}>
						<td>
							<EquipmentCell cell=equipment.id />
						</td>
					</Show>
					<Show when=move || {
						field_filter.get().contains(&String::from("equipment_type"))
					}>
						<td>
							<EquipmentCell cell=equipment.equipment_type.clone() />
						</td>
					</Show>
					<Show when=move || {
						field_filter.get().contains(&String::from("qrcode"))
					}>
						<td>
							<EquipmentCell cell=equipment.qrcode.clone() />
						</td>
					</Show>
					<Show when=move || {
						field_filter.get().contains(&String::from("create_date"))
					}>
						<td>
							<EquipmentCell cell=equipment.create_date />
						</td>
					</Show>
					<Show when=move || {
						field_filter.get().contains(&String::from("name"))
					}>
						<td>
							<EquipmentCell cell=equipment.name.clone() />
						</td>
					</Show>
					<Show when=move || {
						field_filter.get().contains(&String::from("status"))
					}>
						<td>
							<EquipmentCell cell=equipment.status.clone() />
						</td>
					</Show>
					<Show when=move || {
						field_filter.get().contains(&String::from("manufacturer"))
					}>
						<td>
							<EquipmentCell cell=equipment.manufacturer.clone() />
						</td>
					</Show>
					<Show when=move || {
						field_filter.get().contains(&String::from("purchase_date"))
					}>
						<td>
							<EquipmentCell cell=equipment.purchase_date />
						</td>
					</Show>
					<Show when=move || {
						field_filter.get().contains(&String::from("vendor"))
					}>
						<td>
							<EquipmentCell cell=equipment.vendor.clone() />
						</td>
					</Show>
					<Show when=move || {
						field_filter.get().contains(&String::from("cost_in_cent"))
					}>
						<td>
							<EquipmentCell cell=equipment.cost_in_cent.clone() />
						</td>
					</Show>
					<Show when=move || {
						field_filter
							.get()
							.contains(&String::from("warranty_expiration_date"))
					}>
						<td>
							<EquipmentCell cell=equipment.warranty_expiration_date />
						</td>
					</Show>
					<Show when=move || {
						field_filter.get().contains(&String::from("location"))
					}>
						<td>
							<EquipmentCell cell=equipment.location.clone() />
						</td>
					</Show>
					<Show when=move || {
						field_filter.get().contains(&String::from("notes"))
					}>
						<td>
							<EquipmentCell cell=equipment.notes.clone() />
						</td>
					</Show>
					<td>
						<A href=format!("/equipment/{}", equipment.id)>Details</A>
					</td>
					<td>
						<A href=format!("edit/{}", equipment.id)>Edit</A>
					</td>
					<td>
						<button
							class=css::delete
							on:click=move |_| {
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
							}
						>
							Delete
						</button>
					</td>
				</tr>
			}
		})
		.collect_view()
}
