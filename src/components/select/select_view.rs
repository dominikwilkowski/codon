use leptos::*;
use thaw::MultiSelect as ThawMultiSelect;

stylance::import_style!(css, "select.module.css");

pub use thaw::{MultiSelectOption, TagVariant};

#[component]
pub fn Select(
	#[prop(optional)] name: &'static str,
	#[prop(optional)] disabled: RwSignal<bool>,
	children: Children,
) -> impl IntoView {
	let class = match disabled.get() {
		true => css::select_disabled,
		false => "",
	};

	view! {
		<label class=format!("{} {}", css::select_wrapper, class)>
			<select class=css::select name=name disabled=disabled>
				{children()}
			</select>
		</label>
	}
}

#[component]
pub fn MultiSelect(
	value: RwSignal<Vec<String>>,
	options: RwSignal<Vec<MultiSelectOption<String>>>,
	#[prop(optional)] clearable: RwSignal<bool>,
) -> impl IntoView {
	view! { <ThawMultiSelect class="input_shadow codon_multiselect" value options clearable=clearable /> }
}
