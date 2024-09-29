use leptos::*;
use std::collections::HashSet;
use thaw::{Checkbox as ThawCheckbox, CheckboxGroup as ThawCheckboxGroup, CheckboxItem as ThawCheckboxItem};

stylance::import_style!(css, "checkbox.module.css");

#[component]
pub fn CheckboxGroup(#[prop(optional)] value: RwSignal<HashSet<String>>, children: Children) -> impl IntoView {
	view! { <ThawCheckboxGroup value children /> }
}

#[component]
pub fn CheckboxItem(#[prop(into)] key: String, #[prop(optional, into)] label: String) -> impl IntoView {
	view! { <ThawCheckboxItem key label /> }
}

#[component]
pub fn Checkbox(#[prop(optional)] value: RwSignal<bool>, children: Children) -> impl IntoView {
	view! { <ThawCheckbox value children /> }
}
