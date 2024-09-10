use leptos::*;
use thaw::Switch as ThawSwitch;

stylance::import_style!(css, "switch.module.css");

#[component]
pub fn Switch(
	#[prop(optional)] value: RwSignal<bool>,
	#[prop(optional, into)] on_change: Option<Callback<bool>>,
) -> impl IntoView {
	view! {
		<ThawSwitch
			class="input_shadow"
			value
			on_change=on_change.unwrap_or_else(|| Callback::from(|_| {}))
		/>
	}
}
