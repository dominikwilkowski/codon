use leptos::*;
use std::hash::Hash;
use thaw::Select as ThawSelect;

stylance::import_style!(css, "select.module.css");

pub use thaw::SelectOption;

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
