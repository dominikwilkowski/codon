use crate::{
	components::{avatar::Avatar, multiline::MultiLine},
	equipment::{AvatarData, Cost, EquipmentStatus, EquipmentType, Notes, QRCode},
	icons::{Flask, IncubationCabinet, Vessel},
};

use chrono::prelude::*;
use leptos::*;

stylance::import_style!(css, "cell.module.css");

pub trait EquipmentCellView {
	fn view(self, table_view: bool) -> impl IntoView;
}

impl EquipmentCellView for i32 {
	fn view(self, _: bool) -> impl IntoView {
		view! { <span class=css::equipment_id>{format!("{self:06}")}</span> }
	}
}

impl EquipmentCellView for EquipmentType {
	fn view(self, _: bool) -> impl IntoView {
		view! {
			<div class=css::equipment_type>
				{match self {
					EquipmentType::Flask => {
						view! {
							<Flask />
							<span>" Flask"</span>
						}
					}
					EquipmentType::Vessel => {
						view! {
							<Vessel />
							<span>" Vessel"</span>
						}
					}
					EquipmentType::IncubationCabinet => {
						view! {
							<IncubationCabinet />
							<span>" Incubation Cabinet"</span>
						}
					}
				}}
			</div>
		}
	}
}

impl EquipmentCellView for AvatarData {
	fn view(self, _: bool) -> impl IntoView {
		view! { <Avatar data=self tiny=true /> }
	}
}

impl EquipmentCellView for EquipmentStatus {
	fn view(self, _: bool) -> impl IntoView {
		match self {
			EquipmentStatus::Cleaned => {
				view! { <div class=css::cleaned>"Cleaned"</div> }
			},
			EquipmentStatus::Prepared => {
				view! { <div class=css::prepared>"Prepared"</div> }
			},
			EquipmentStatus::Sterilized => {
				view! { <div class=css::sterilized>"Sterilized"</div> }
			},
			EquipmentStatus::InUse => {
				view! { <div class=css::inuse>"In" "\u{00A0}" "Use"</div> }
			},
			EquipmentStatus::Dirty => {
				view! { <div class=css::dirty>"Dirty"</div> }
			},
			EquipmentStatus::Archived => {
				view! { <div class=css::archived>"Archived"</div> }
			},
		}
	}
}

impl EquipmentCellView for String {
	fn view(self, _: bool) -> impl IntoView {
		view! { <StringCell item=self /> }
	}
}

impl EquipmentCellView for QRCode {
	fn view(self, _: bool) -> impl IntoView {
		view! { <img src=format!("{self}") alt="The QR code" class=css::qrcode /> }
	}
}

impl EquipmentCellView for Option<String> {
	fn view(self, _: bool) -> impl IntoView {
		match self {
			Some(value) => view! { <StringCell item=value /> }.into_view(),
			None => view! {}.into_view(),
		}
	}
}

impl EquipmentCellView for Option<Cost> {
	fn view(self, _: bool) -> impl IntoView {
		match self {
			Some(value) => view! { <span>{format!("${value}")}</span> }.into_view(),
			None => view! { <span /> }.into_view(),
		}
	}
}

impl EquipmentCellView for Option<Notes> {
	fn view(self, table_view: bool) -> impl IntoView {
		match self {
			Some(value) => {
				let mut text = value.to_string();
				if table_view && text.chars().count() > 100 {
					text = text.chars().take(100).chain(std::iter::once('…')).collect::<String>()
				}
				view! {
					<div>
						<MultiLine text />
					</div>
				}
				.into_view()
			},
			None => view! { <div /> }.into_view(),
		}
	}
}

#[component]
fn StringCell(item: String) -> impl IntoView {
	view! { <span>{item}</span> }
}

impl EquipmentCellView for DateTime<Utc> {
	fn view(self, _: bool) -> impl IntoView {
		view! { <DateCell item=self /> }
	}
}

impl EquipmentCellView for Option<DateTime<Utc>> {
	fn view(self, _: bool) -> impl IntoView {
		match self {
			Some(item) => view! { <DateCell item /> }.into_view(),
			None => view! { <span /> }.into_view(),
		}
	}
}

#[component]
fn DateCell(item: DateTime<Utc>) -> impl IntoView {
	view! { <span>{item.format("%d %b %Y").to_string()}</span> }
}

#[component]
pub fn EquipmentCell<T: EquipmentCellView + 'static>(cell: T, #[prop(optional)] table_view: bool) -> impl IntoView {
	cell.view(table_view).into_view()
}
