use crate::equipment::PeopleData;

use leptos::*;

stylance::import_style!(css, "avatar.module.css");

#[component]
pub fn Avatar(data: PeopleData) -> impl IntoView {
	view! {
		<figure class=format!(
			"avata-status-{} {}",
			data.status.to_string().replace(" ", "").to_lowercase(),
			css::avatar,
		)>
			<div>
				<img
					src=format!(
						"/avatars/{}",
						data.picture.unwrap_or(String::from("default.png")),
					)
					alt=format!("Picture of {}", data.preferred_name)
				/>
			</div>
			<figcaption>{data.preferred_name}</figcaption>
		</figure>
	}
}
