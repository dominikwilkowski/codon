use crate::icons::Logo;

use leptos::*;

stylance::import_style!(css, "footer.module.css");

#[component]
pub fn Footer() -> impl IntoView {
	view! {
		<footer class=css::footer>
			<div>
				<span class=css::footer_name>
					<span>@2024</span>
					Codon
				</span>
				<ul class=css::footer_list>
					<li>
						<a href="https://github.com/dominikwilkowski/codon" target="_blank">
							Open source
						</a>
					</li>
					<li>
						<a href="https://github.com/dominikwilkowski/codon/issues" target="_blank">
							Submit issues
						</a>
					</li>
				</ul>
			</div>

			<div class=css::footer_logo>
				<Logo />
			</div>
		</footer>
	}
}
