use crate::{
	ds::Ds,
	equipment::{Equipment, EquipmentAdd, EquipmentDetail},
	error_template::{AppError, ErrorTemplate},
	footer::Footer,
	header::Header,
	home::Home,
	samples::Samples,
};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use thaw::*;

stylance::import_style!(css, "app.module.css");

pub type ScrollableBody = RwSignal<bool>;

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();
	let mut theme = Theme::dark();
	theme.common.font_color = String::from("var(--text)");
	theme.common.font_size = String::from("1rem;");
	theme.common.color_primary = String::from("var(--action)");
	theme.common.color_primary_hover = String::from("var(--action)");
	theme.common.color_primary_active = String::from("var(--action)");
	// theme.common.color_success = String::from("");
	// theme.common.color_success_hover = String::from("");
	// theme.common.color_success_active = String::from("");
	// theme.common.color_warning = String::from("");
	// theme.common.color_warning_hover = String::from("");
	// theme.common.color_warning_active = String::from("");
	// theme.common.color_error = String::from("");
	// theme.common.color_error_hover = String::from("");
	// theme.common.color_error_active = String::from("");
	theme.common.border_radius = String::from("3px");

	theme.input.background_color = String::from("var(--input-bg)");
	theme.date_picker.panel_background_color = String::from("var(--input-bg)");
	theme.date_picker.panel_date_item_background_color_hover = String::from("var(--action)");
	theme.date_picker.panel_other_month_font_color = String::from("var(--text)");
	theme.button.color_text_disabled = String::from("var(--disabled-fg)");
	theme.button.color_background_disabled = String::from("var(--disabled-bg)");
	theme.button.color_border_disabled = String::from("var(--disabled-bg)");
	theme.button.border_color_outlined = String::from("var(--action)");
	theme.switch.background_color = String::from("var(--input-bg)");
	theme.select.background_color = String::from("var(--input-bg)");
	theme.select.menu_background_color = String::from("var(--input-bg)");
	theme.message.background_color = String::from("var(--bg)");

	let is_body_scrollable = create_rw_signal(true);
	provide_context::<ScrollableBody>(is_body_scrollable);

	view! {
		<Body class=move || { if is_body_scrollable.get() { "" } else { "not_scrollable" } } />
		<Stylesheet id="leptos" href="/pkg/codon.css" />
		<Link rel="preload" as_="font" type_="font/woff2" href="/noto_sans_mono_latin.woff2" crossorigin="" />
		<Link rel="preload" as_="font" type_="font/woff2" href="/noto_sans_mono_latin_ext.woff2" crossorigin="" />
		<Title text="Welcome to Codon" />

		<ThemeProvider theme>
			<MessageProvider>
				<Router
					trailing_slash=TrailingSlash::Redirect
					fallback=|| {
						let mut outside_errors = Errors::default();
						outside_errors.insert_with_default_key(AppError::NotFound);
						view! { <ErrorTemplate outside_errors /> }.into_view()
					}
				>
					<Header />
					<main class=format!("{} frame", css::main)>
						<Routes>
							<Route path="" view=Home />
							<Route path="/ds" view=Ds />
							<Route path="/samples" view=Samples />
							<Route path="/equipment" view=Equipment />
							<Route path="/equipment/add" view=EquipmentAdd />
							<Route path="/equipment/:id" view=EquipmentDetail />
						</Routes>
					</main>
				</Router>
				<Footer />
			</MessageProvider>
		</ThemeProvider>
	}
}
