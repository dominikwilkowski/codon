use crate::{
	equipment::{Cost, EquipmentStatus, EquipmentType, Notes, QRCode},
	icons::{Flask, IncubationCabinet, Vessel},
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

impl EquipmentCellView for EquipmentType {
	fn view(self) -> impl IntoView {
		match self {
			EquipmentType::Flask => view! {
				<Flask />
				" Fask"
			},
			EquipmentType::Vessel => view! {
				<Vessel />
				" Vessel"
			},
			EquipmentType::IncubationCabinet => {
				view! {
					<IncubationCabinet />
					" Incubation Cabinet"
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
				view! { <div>Cost: {format!("${value}")}</div> }.into_view()
			},
			None => view! {}.into_view(),
		}
	}
}

impl EquipmentCellView for Option<Notes> {
	fn view(self) -> impl IntoView {
		match self {
			Some(value) => {
				let text = value.to_string();
				view! {
					<div>
						{text
							.lines()
							.map(|line| {
								let line = line.to_string();
								view! { <>{line}<br /></> }
							})
							.collect_view()}
					</div>
				}
				.into_view()
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
