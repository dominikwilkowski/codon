use crate::app::ScrollableBody;

use leptos::*;

stylance::import_style!(css, "img_attachment.module.css");

#[component]
pub fn ImgAttachment(file_path: Option<String>) -> impl IntoView {
	let is_open = create_rw_signal(false);

	let is_body_scrollable = use_context::<ScrollableBody>().expect("No ScrollableBody context provider");

	view! {
		{if file_path.is_some() {
			let file_path = file_path.unwrap();
			view! {
				<form
					class=move || {
						if is_open.get() { format!("{} form-isopen", css::form) } else { css::form.to_string() }
					}
					action=file_path.clone()
					method="GET"
					on:submit=move |event| {
						event.prevent_default();
						is_body_scrollable.set(is_open.get());
						is_open.update(|open| *open = !*open);
					}
				>
					<button type="submit" class=css::btn>
						{if file_path.to_lowercase().ends_with(".mov") {
							view! {
								<Show
									when=move || is_open.get()
									fallback=|| {
										view! {
											<svg
												class=css::placeholder
												xmlns="http://www.w3.org/2000/svg"
												viewBox="0 0 24 24"
												fill="currentColor"
											>
												<path d="m9 17 8-5-8-5z" />
											</svg>
										}
									}
								>
									<div>
										<video class=css::video controls>
											<source src=file_path.clone() type="video/quicktime" />
											"Your browser doen't support the videos it seems."
										</video>
									</div>
								</Show>
							}
								.into_view()
						} else {
							view! { <img class=css::img src=file_path /> }.into_view()
						}}
					</button>
				</form>
			}
				.into_view()
		} else {
			view! {}.into_view()
		}}
	}
}
