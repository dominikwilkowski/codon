use leptos::*;

stylance::import_style!(css, "multiline.module.css");

#[component]
pub fn MultiLine(text: String) -> impl IntoView {
	text.lines().map(|line| view! { <p class=css::line>{line.to_string()}</p> }).collect_view()
}
