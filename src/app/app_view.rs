use crate::{
	auth::{Login, Logout, User, get_user},
	ds::Ds,
	equipment::{Equipment, EquipmentAdd, EquipmentDetail},
	error_template::{AppError, ErrorTemplate},
	footer::Footer,
	header::Header,
	home::Home,
	login::Login,
	profile::Profile,
};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use thaw::*;

stylance::import_style!(css, "app.module.css");

pub type ScrollableBody = RwSignal<bool>;
pub type LoginAction = Action<Login, Result<(), ServerFnError>>;
pub type LogoutAction = Action<Logout, Result<(), ServerFnError>>;
pub type UserSignal = RwSignal<Option<User>>;

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
	theme.switch.background_color = String::from("var(--input-bg)");
	theme.select.background_color = String::from("var(--input-bg)");
	theme.select.menu_background_color = String::from("var(--input-bg)");
	theme.message.background_color = String::from("var(--bg)");
	theme.dropdown.background_color = String::from("var(--input-bg)");

	let is_body_scrollable = create_rw_signal(true);
	provide_context::<ScrollableBody>(is_body_scrollable);

	let login = create_server_action::<Login>();
	provide_context::<LoginAction>(login);

	let logout = create_server_action::<Logout>();
	provide_context::<LogoutAction>(logout);

	let user_signal: UserSignal = create_rw_signal(None);
	let user = create_local_resource(move || (login.version().get(), logout.version().get()), move |_| get_user());
	create_effect(move |_| {
		match user.get() {
			Some(Ok(Some(user))) => user_signal.set(Some(user)),
			_ => user_signal.set(None),
		};
	});
	provide_context::<UserSignal>(user_signal);

	view! {
		<Body class=move || { if is_body_scrollable.get() { "" } else { "not_scrollable" } } />
		<Stylesheet id="leptos" href="/pkg/codon.css" />
		<Link rel="preload" as_="font" type_="font/woff2" href="/noto_sans_mono_latin.woff2" crossorigin="" />
		<Link rel="preload" as_="font" type_="font/woff2" href="/noto_sans_mono_latin_ext.woff2" crossorigin="" />
		<Title text="Welcome to Codon" />

		<ThemeProvider theme>
			<MessageProvider>
				<Router
					trailing_slash=TrailingSlash::Drop
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
							<Route path="/login" view=move || view! { <Login redirect="/" /> } />
							<Route path="/profile" view=move || view! { <Profile /> } />
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
