use leptos::*;
use thaw::{
	Input as ThawInput,
	// TextArea as ThawTextArea,
};

stylance::import_style!(css, "input.module.css");

#[component]
pub fn Input(
	#[prop(optional)] placeholder: &'static str,
	#[prop(optional)] value: RwSignal<String>,
	#[prop(optional)] disabled: RwSignal<bool>,
	#[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
	view! { <ThawInput class=css::input placeholder value disabled attrs /> }
}

#[component]
pub fn MoneyInput(
	#[prop(optional)] placeholder: &'static str,
	#[prop(optional)] value: RwSignal<String>,
	#[prop(optional)] disabled: RwSignal<bool>,
) -> impl IntoView {
	let dis_class = if disabled.get() {
		css::money_input_disabled
	} else {
		""
	};
	view! {
		<div class=format!("{} {} {dis_class}", css::input, css::money_input)>
			<input
				type="number"
				class="thaw-input__input-el"
				placeholder=placeholder
				prop:value=value
				disabled=disabled.get()
				step="0.01"
			/>
		</div>
	}
}
