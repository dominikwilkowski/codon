use leptos::*;

stylance::import_style!(css, "button.module.css");

pub use thaw::{use_message, MessageOptions, MessageVariant};

#[derive(Debug, Default)]
pub enum ButtonVariant {
	#[default]
	Default,
	Outlined,
	Text,
}

#[component]
pub fn Button(
	#[prop(optional)] disabled: RwSignal<bool>,
	#[prop(optional)] loading: RwSignal<bool>,
	#[prop(optional)] name: &'static str,
	#[prop(optional)] value: RwSignal<String>,
	#[prop(optional, default = "button")] kind: &'static str,
	#[prop(optional, default = ButtonVariant::Default)] variant: ButtonVariant,
	#[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
	children: Children,
) -> impl IntoView {
	let class = create_rw_signal(vec![css::btn.to_string()]);

	match variant {
		ButtonVariant::Default => {},
		ButtonVariant::Outlined => class.update(|c| c.push(css::outlined.to_string())),
		ButtonVariant::Text => class.update(|c| c.push(css::text.to_string())),
	};

	create_effect(move |_| {
		if loading.get() {
			class.update(|c| c.push(css::loading.to_string()));
		}
	});

	view! {
		<button
			type=kind
			class=move || class.get().join(" ")
			disabled=move || disabled.get()
			name=name
			value=value
			on:click=move |x| {
				if !loading.get() {
					on_click.unwrap_or_else(|| Callback::from(|_| {}))(x);
				}
			}
		>
			<Show when=move || loading.get()>
				<svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 1024 1024">
					<path d="M988 548a36 36 0 0 1-36-36c0-59.4-11.6-117-34.6-171.3a440.4 440.4 0 0 0-94.3-139.9 437.7 437.7 0 0 0-139.9-94.3A437.5 437.5 0 0 0 512 72a36 36 0 1 1 0-72 507 507 0 0 1 199.3 40.3A508.4 508.4 0 0 1 874 150a511.1 511.1 0 0 1 149.9 362c.1 19.9-16 36-35.9 36z" />
				</svg>
			</Show>
			<span>{children()}</span>
		</button>
	}
}
