use leptos::*;

stylance::import_style!(css, "timezone_offset.module.css");

#[component]
pub fn Timezone() -> impl IntoView {
	view! {
		<input type="hidden" id="timezone_offset" name="timezone_offset" />
		<script>{r#"document.getElementById("timezone_offset").value = new Date().getTimezoneOffset();"#}</script>
	}
}
