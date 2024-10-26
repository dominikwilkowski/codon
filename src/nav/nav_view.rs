use crate::app::{LogoutAction, UserSignal};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "nav.module.css");

#[component]
pub fn Nav() -> impl IntoView {
	let logout_action = use_context::<LogoutAction>().expect("No logout action found in context");
	let user_signal = use_context::<UserSignal>().expect("No user signal found in context");

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
					<Suspense fallback=move || {
						view! { <A href="/login">"Login"</A> }
					}>
						{move || {
							match user_signal.get() {
								None => view! { <A href="/login">"Login"</A> }.into_view(),
								Some(user) => {
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
							}
						}}
					</Suspense>
				</li>
			</ul>
		</nav>
	}
}
