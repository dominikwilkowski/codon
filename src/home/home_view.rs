use leptos::*;

stylance::import_style!(css, "home.module.css");

#[component]
pub fn Home() -> impl IntoView {
	let count = create_rw_signal(0);
	let on_click = move |_| count.update(|count| *count += 1);

	view! {
		<h1>Welcome to Codon</h1>
		<button on:click=on_click>"Click Me: " {count}</button>
	}
}
