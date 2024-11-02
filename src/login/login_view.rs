use crate::{
	app::LoginAction,
	components::{button::Button, checkbox::Checkbox, input::Input},
};

use leptos::*;
use leptos_router::*;
use std::borrow::Cow;

stylance::import_style!(css, "login.module.css");

#[component]
pub fn Login(redirect: impl Into<Cow<'static, str>>) -> impl IntoView {
	let login_action = use_context::<LoginAction>().expect("No login action found in context");
	let redirect: Cow<'static, str> = redirect.into();

	view! {
		<div>
			<ActionForm action=login_action class=css::login_form>
				<h1>Login</h1>
				<input type="hidden" name="redirect" value=redirect />
				<label class=css::label>
					<span>User:</span>
					<Input name="username" placeholder="Username" value=create_rw_signal(String::new()) />
				</label>
				<label class=css::label>
					<span>Password:</span>
					<Input
						kind="password"
						name="password"
						placeholder="Password"
						value=create_rw_signal(String::new())
					/>
				</label>
				<div class=css::footer>
					<Checkbox attr::name="remember">Remember me</Checkbox>
					<Button kind="submit">Log In</Button>
				</div>
			</ActionForm>
			{move || {
				if let Some(responds) = login_action.value().get() {
					match responds {
						Ok(_) => view! {}.into_view(),
						Err(error) => {
							view! { <span>{error.to_string().replace("Error running server function: ", "")}</span> }
								.into_view()
						}
					}
				} else {
					view! {}.into_view()
				}
			}}
		</div>
	}
}
