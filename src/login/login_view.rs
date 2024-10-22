use crate::app::LoginAction;

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "login.module.css");

#[component]
pub fn Login(#[prop(optional, default = "/")] redirect: &'static str) -> impl IntoView {
	let login_action = use_context::<LoginAction>().expect("No login action found in context");

	view! {
		<div>
			<h1>Login</h1>
			<ActionForm action=login_action>
				<input type="hidden" name="redirect" value=redirect />
				<label>"User:" <input type="text" placeholder="Username" maxlength="32" name="username" /></label>
				<br />
				<label>
					"Password:" <input type="password" placeholder="Password" name="password" class="auth-input" />
				</label>
				<br />
				<label>
					<input type="checkbox" name="remember" class="auth-input" />
					"Remember me?"
				</label>
				<br />
				<button type="submit" class="button">
					"Log In"
				</button>
			</ActionForm>
			{move || {
				if let Some(responds) = login_action.value().get() {
					match responds {
						Ok(_) => view! {}.into_view(),
						Err(error) => {
							view! { <span>{error.to_string().replace("error running server function: ", "")}</span> }
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
