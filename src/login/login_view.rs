use crate::app::LoginAction;

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "login.module.css");

#[component]
pub fn Login() -> impl IntoView {
	let login_action = use_context::<LoginAction>().expect("No login action found in context");

	// TODO: handle errors

	view! {
		<div>
			<ActionForm action=login_action>
				<h1>"Log In"</h1>
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
		</div>
	}
}
