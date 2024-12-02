use leptos::*;

stylance::import_style!(css, "profile.module.css");

#[component]
pub fn Profile() -> impl IntoView {
	view! {
		<div>
			<h1>Your profile</h1>
			- username
			- password
			- employee_id
			- status
			- first_name
			- last_name
			- preferred_name
			- email
			- phone_number
			- department
			- role
			- hire_date
			- emergency_contact
			- certifications
			- specializations
			- picture
			- bio
		</div>
	}
}
