use crate::{
	equipment::schema::{Cost, EquipmentStatus, EquipmentTypes, Notes, QRCode},
	icons::{
		flask::Flask, incubation_cabinet::IncubationCabinet, vessel::Vessel,
	},
};

use chrono::prelude::*;
use leptos::*;

stylance::import_style!(css, "cell.module.css");

pub trait EquipmentCellView {
	fn view(self) -> impl IntoView;
}

impl EquipmentCellView for i32 {
	fn view(self) -> impl IntoView {
		view! { <span>ID: {format!("{self}")}</span> }
	}
}

impl EquipmentCellView for EquipmentTypes {
	fn view(self) -> impl IntoView {
		match self {
			EquipmentTypes::Flask => view! {
				<abbr title="A Flask">
					<Flask />
				</abbr>
			},
			EquipmentTypes::Vessel => view! {
				<abbr title="A Vessel">
					<Vessel />
				</abbr>
			},
			EquipmentTypes::IncubationCabinet => {
				view! {
					<abbr title="A Incubation Cabinet">
						<IncubationCabinet />
					</abbr>
				}
			},
		}
	}
}

impl EquipmentCellView for EquipmentStatus {
	fn view(self) -> impl IntoView {
		match self {
			EquipmentStatus::Working => view! {
				<div class=css::working>
					<span class="">Working</span>
				</div>
			},
			EquipmentStatus::NeedsCleaning => view! {
				<div class=css::needscleaning>
					<span class="">NeedsCleaning</span>
				</div>
			},
			EquipmentStatus::Preparation => view! {
				<div class=css::preparation>
					<span class="">Preparation</span>
				</div>
			},
			EquipmentStatus::Sterilization => view! {
				<div class=css::sterilization>
					<span class="">Sterilization</span>
				</div>
			},
			EquipmentStatus::Broken => view! {
				<div class=css::broken>
					<span class="">Broken</span>
				</div>
			},
			EquipmentStatus::OutOfCommission => view! {
				<div class=css::outofcommission>
					<span class="">OutOfCommission</span>
				</div>
			},
		}
	}
}

impl EquipmentCellView for String {
	fn view(self) -> impl IntoView {
		view! { <StringCell item=self /> }
	}
}

impl EquipmentCellView for QRCode {
	fn view(self) -> impl IntoView {
		view! {
			<img
				src=format!("/qrcodes/{self}")
				alt="The QR code"
				class=css::qrcode
			/>
		}
	}
}

impl EquipmentCellView for Option<String> {
	fn view(self) -> impl IntoView {
		match self {
			Some(value) => view! { <StringCell item=value /> }.into_view(),
			None => view! {}.into_view(),
		}
	}
}

impl EquipmentCellView for Option<Cost> {
	fn view(self) -> impl IntoView {
		match self {
			Some(value) => {
				view! { <div>Cost: {format!("{value}")}</div> }.into_view()
			},
			None => view! {}.into_view(),
		}
	}
}

impl EquipmentCellView for Option<Notes> {
	fn view(self) -> impl IntoView {
		match self {
			Some(value) => {
				view! { <div>Notes: {format!("{value}")}</div> }.into_view()
			},
			None => view! {}.into_view(),
		}
	}
}

#[component]
fn StringCell(item: String) -> impl IntoView {
	view! { <div>String: {item}</div> }
}

impl EquipmentCellView for DateTime<Utc> {
	fn view(self) -> impl IntoView {
		view! { <DateCell item=Some(self) /> }
	}
}

impl EquipmentCellView for Option<DateTime<Utc>> {
	fn view(self) -> impl IntoView {
		match self {
			Some(_) => view! { <DateCell item=self /> }.into_view(),
			None => view! {}.into_view(),
		}
	}
}

#[component]
fn DateCell(item: Option<DateTime<Utc>>) -> impl IntoView {
	match item {
		Some(d) => {
			view! { <div>Date: {d.format("%Y-%m-%d").to_string()}</div> }.into_view()
		},
		None => view! {}.into_view(),
	}
}

#[component]
pub fn EquipmentCell<T: EquipmentCellView + 'static>(cell: T) -> impl IntoView {
	cell.view().into_view()
}
