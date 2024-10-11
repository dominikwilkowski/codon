use chrono::NaiveDate;
use leptos::*;
use thaw::DatePicker as ThawDatePicker;

stylance::import_style!(css, "datepicker.module.css");

pub use thaw::SelectOption;

#[component]
pub fn DatePicker(
	#[prop(optional, into)] value: RwSignal<Option<NaiveDate>>,
	#[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
	view! { <ThawDatePicker class=format!("input_shadow {}", css::datepicker) value attrs /> }
}
