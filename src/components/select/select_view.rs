use leptos::*;
use std::hash::Hash;
use thaw::{MultiSelect as ThawMultiSelect, Select as ThawSelect};

stylance::import_style!(css, "select.module.css");

pub use thaw::{MultiSelectOption, SelectOption, TagVariant};

#[component]
pub fn Select<T>(
	#[prop(optional, into)] value: RwSignal<Option<T>>,
	#[prop(optional, into)] options: RwSignal<Vec<SelectOption<T>>>,
) -> impl IntoView
where
	T: Eq + Hash + Clone + 'static,
{
	view! { <ThawSelect class="input_shadow" value options /> }
}

#[component]
pub fn MultiSelect(
	value: RwSignal<Vec<String>>,
	options: RwSignal<Vec<MultiSelectOption<String>>>,
	#[prop(optional)] clearable: RwSignal<bool>,
) -> impl IntoView {
	view! {
		<ThawMultiSelect
			class="input_shadow"
			value
			options
			clearable=clearable
		/>
	}
}
