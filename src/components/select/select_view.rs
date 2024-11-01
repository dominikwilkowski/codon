use leptos::*;
use thaw::MultiSelect as ThawMultiSelect;

stylance::import_style!(css, "select.module.css");

pub use thaw::{MultiSelectOption, TagVariant};

#[component]
pub fn Select(
	#[prop(optional)] name: &'static str,
	#[prop(optional)] disabled: RwSignal<bool>,
	#[prop(optional)] required: bool,
	children: Children,
) -> impl IntoView {
	let class = create_rw_signal("");

	create_effect(move |_| {
		if disabled.get() {
			class.set(css::select_disabled)
		} else {
			class.set("")
		};
	});

	view! {
		<label class=move || format!("{} {}", css::select_wrapper, class.get())>
			<select class=css::select name=name disabled=disabled required=required>
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
