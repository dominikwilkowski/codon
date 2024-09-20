use crate::{
	components::multiline::MultiLine,
	equipment::{Cost, EquipmentStatus, EquipmentType, Notes, QRCode},
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
		view! { <span>{format!("{self}")}</span> }
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
							<span>" Fask"</span>
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

impl EquipmentCellView for EquipmentStatus {
	fn view(self, _: bool) -> impl IntoView {
		match self {
			EquipmentStatus::Working => {
				view! { <div class=css::working>"Working"</div> }
			},
			EquipmentStatus::NeedsCleaning => {
				view! { <div class=css::needscleaning>"Needs" "\u{00A0}" "Cleaning"</div> }
			},
			EquipmentStatus::Preparation => {
				view! { <div class=css::preparation>"Preparation"</div> }
			},
			EquipmentStatus::Sterilization => {
				view! { <div class=css::sterilization>"Sterilization"</div> }
			},
			EquipmentStatus::Broken => {
				view! { <div class=css::broken>"Broken"</div> }
			},
			EquipmentStatus::OutOfCommission => view! {
				<div class=css::outofcommission>
					"Out" "\u{00A0}" "Of" "\u{00A0}" "Commission"
				</div>
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
		view! {
			<img
				src=format!("/qrcodes/equipment/{self}")
				alt="The QR code"
				class=css::qrcode
			/>
		}
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
			None => view! {}.into_view(),
		}
	}
}

impl EquipmentCellView for Option<Notes> {
	fn view(self, table_view: bool) -> impl IntoView {
		match self {
			Some(value) => {
				let mut text = value.to_string();
				if table_view && text.chars().count() > 100 {
					text = text
						.chars()
						.take(100)
						.chain(std::iter::once('…'))
						.collect::<String>()
				}
				view! {
					<div>
						<MultiLine text />
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
			None => view! {}.into_view(),
		}
	}
}

#[component]
fn DateCell(item: DateTime<Utc>) -> impl IntoView {
	view! { <span>{item.format("%d %b %Y").to_string()}</span> }
}

#[component]
pub fn EquipmentCell<T: EquipmentCellView + 'static>(
	cell: T,
	#[prop(optional)] table_view: bool,
) -> impl IntoView {
	cell.view(table_view).into_view()
}
