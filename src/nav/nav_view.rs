use crate::{
	app::{LoginAction, LogoutAction},
	auth::get_user,
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "nav.module.css");

#[component]
pub fn Nav() -> impl IntoView {
	let login_action = use_context::<LoginAction>().expect("No login action found in context");
	let logout_action = use_context::<LogoutAction>().expect("No logout action found in context");

	let user =
		create_resource(move || (login_action.version().get(), logout_action.version().get()), move |_| get_user());

	view! {
		<nav class=css::nav>
			<ul>
				<li>
					<A href="/">Homepage</A>
				</li>
				<li>
					<A href="/samples">Samples</A>
				</li>
				<li>
					<A href="/equipment">Equipment</A>
				</li>
				<li>
					<Transition fallback=move || {
						view! { <span>"Loading..."</span> }
					}>
						{move || {
							user.get()
								.map(|user| match user {
									Err(error) => {
										view! {
											<A href="/login">"Login"</A>
											<span>{format!("Login error: {error}")}</span>
										}
											.into_view()
									}
									Ok(None) => view! { <A href="/login">"Login"</A> }.into_view(),
									Ok(Some(user)) => {
										view! {
											<ActionForm action=logout_action>
												<button type="submit" class="button">
													"Log Out"
												</button>
											</ActionForm>
											<span>{format!("Logged in as: {}", user.username)}</span>
										}
											.into_view()
									}
								})
						}}
					</Transition>
				</li>
			</ul>
		</nav>
	}
}
