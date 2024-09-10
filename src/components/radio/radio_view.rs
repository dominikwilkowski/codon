use leptos::*;
use thaw::{
	Radio as ThawRadio, RadioGroup as ThawRadioGroup, RadioItem as ThawRadioItem,
};

stylance::import_style!(css, "radio.module.css");

#[component]
pub fn RadioGroup(
	#[prop(optional)] value: RwSignal<Option<String>>,
	children: Children,
) -> impl IntoView {
	view! { <ThawRadioGroup value children /> }
}

#[component]
pub fn RadioItem(
	#[prop(into)] key: String,
	children: Children,
) -> impl IntoView {
	view! { <ThawRadioItem key children /> }
}

#[component]
pub fn Radio(
	#[prop(optional)] value: RwSignal<bool>,
	children: Children,
) -> impl IntoView {
	view! { <ThawRadio value children /> }
}
