use leptos::*;
use leptos_router::*;

stylance::import_style!(css, "nav.module.css");

#[component]
pub fn Nav() -> impl IntoView {
	view! {
		<nav class=format!("{} frame", css::nav)>
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
			</ul>
		</nav>
	}
}
