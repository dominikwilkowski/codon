use crate::{
	app::UserSignal,
	components::qr_scanner::QRScanner,
	permission::{Permission, Permissions},
};

use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "heading.module.css");

#[component]
pub fn Heading(children: Children, #[prop(optional)] hide_new: bool) -> impl IntoView {
	let user_signal = use_context::<UserSignal>().expect("No user signal found in context");

	view! {
		<div class=css::heading_wrapper>
			<h1 class=css::heading>{children()}</h1>
			<div class=css::heading_btns>
				<Suspense fallback=move || {
					view! { <A href="/login">"Login"</A> }
				}>
					{move || {
						match user_signal.get() {
							None => view! { <span /> }.into_view(),
							Some(user) => {
								let Permissions::All { read: _, write: _, create: perm } = user.permission_equipment;
								view! {
									<Show when=move || !hide_new && perm == Permission::Create(true)>
										<A href="/equipment/add">
											<svg
												xmlns="http://www.w3.org/2000/svg"
												viewBox="0 0 24 24"
												fill="currentColor"
											>
												<path d="M13 7h-2v4H7v2h4v4h2v-4h4v-2h-4z" />
												<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm0 18a8 8 0 1 1 0-16 8 8 0 0 1 0 16z" />
											</svg>
											Add new
										</A>
									</Show>
								}
									.into_view()
							}
						}
					}}
				</Suspense>
				<QRScanner />
			</div>
		</div>
	}
}
