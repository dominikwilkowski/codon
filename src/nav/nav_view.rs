use crate::{
	app::{LogoutAction, UserSignal},
	components::{
		avatar::Avatar,
		dropdown::{Dropdown, DropdownPlacement, DropdownTrigger},
	},
};

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
					<A href="/">Home</A>
				</li>
				<li>
					<A href="/equipment">Equipment</A>
				</li>
				<li class=css::person>
					<Suspense fallback=move || {
						view! { <A href="/login">"Login"</A> }
					}>
						{move || {
							match user_signal.get() {
								None => view! { <A href="/login">"Login"</A> }.into_view(),
								Some(user) => {
									view! {
										<Dropdown placement=DropdownPlacement::BottomEnd on_select=move |_| {}>
											<DropdownTrigger slot>
												<Avatar data=user.into() />
											</DropdownTrigger>
											<A href="/profile" class="dropdown_btn">
												Profile
											</A>
											<ActionForm action=logout_action>
												<button type="submit" class="dropdown_btn">
													"Log Out"
												</button>
											</ActionForm>
										</Dropdown>
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
