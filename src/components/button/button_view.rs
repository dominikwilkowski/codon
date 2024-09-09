use leptos::*;
use thaw::{Button as ThawButton, ButtonVariant};

stylance::import_style!(css, "button.module.css");

pub use thaw::use_message;
pub use thaw::MessageOptions;
pub use thaw::MessageVariant;

#[component]
pub fn Button(
	#[prop(optional)] disabled: RwSignal<bool>,
	#[prop(optional)] loading: RwSignal<bool>,
	#[prop(optional)] outlined: bool,
	#[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
	children: Children,
) -> impl IntoView {
	let variant = match outlined {
		true => ButtonVariant::Outlined,
		false => ButtonVariant::Primary,
	};

	view! {
		<ThawButton
			variant
			disabled
			loading
			on_click=on_click.unwrap_or_else(|| Callback::from(|_| {}))
			class="codon-thaw-btn"
		>
			{children()}
		</ThawButton>
	}
}
