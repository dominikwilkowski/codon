use crate::{
	app::{LoginAction, LogoutAction},
	equipment::PeopleData,
	error_template::ErrorTemplate,
	login::Login,
};

use leptos::*;

stylance::import_style!(css, "profile.module.css");

#[component]
pub fn Profile() -> impl IntoView {
	let login_action = use_context::<LoginAction>().expect("No login action found in context");
	let logout_action = use_context::<LogoutAction>().expect("No logout action found in context");

	let profile_data =
		create_resource(move || (login_action.version().get(), logout_action.version().get()), move |_| get_profile_data());

	view! {
		<Suspense fallback=move || view! { <p>Loading equipment...</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors=errors /> }
			}>
				{move || {
					if profile_data.get().is_some() {
						match profile_data.get().unwrap() {
							Err(error) => {
								let error = error.to_string();
								if error.contains("User not authenticated") {
									view! { <Login redirect=String::from("/profile") /> }.into_view()
								} else {
									view! { <pre class="error">Server Error: {error}</pre> }.into_view()
								}
							}
							Ok(profile) => {
								view! {
									<div>
										<h1>Your profile</h1>
										- employee_id =
										{profile.employee_id}
										- status =
										{format!("{:?}", profile.status)}
										- first_name =
										{profile.first_name}
										- last_name =
										{profile.last_name}
										- preferred_name =
										{profile.preferred_name}
										- email =
										{profile.email}
										- phone_number =
										{profile.phone_number}
										- department =
										{profile.department}
										- role =
										{profile.role}
										- hire_date =
										{format!("{:?}", profile.hire_date)}
										- emergency_contact =
										{profile.emergency_contact}
										- certifications =
										{profile.certifications}
										- specializations =
										{profile.specializations}
										- picture =
										{profile.picture}
										- bio =
										{profile.bio}
									</div>
								}
									.into_view()
							}
						}
					} else {
						view! { <div>Nothing found</div> }.into_view()
					}
				}}
			</ErrorBoundary>
		</Suspense>
	}
}

#[server(prefix = "/api")]
pub async fn get_profile_data() -> Result<PeopleData, ServerFnError> {
	use crate::{auth::get_user, equipment::PeopleSQLData, permission::Permissions};

	use sqlx::PgPool;

	let pool = use_context::<PgPool>()
		.ok_or_else::<ServerFnError, _>(|| ServerFnError::ServerError(String::from("Database not initialized")))?;
	let user = get_user().await?;

	let user_id;
	match user {
		Some(user) => {
			let Permissions::All {
				read: perm,
				write: _,
				create: _,
			} = user.permission_equipment;
			user_id = user.id;
			if !perm.has_permission("read", user_id, user_id) {
				return Err(ServerFnError::Request(String::from("User not authenticated")));
			}
		},
		None => return Err(ServerFnError::Request(String::from("User not authenticated"))),
	};

	let equipment_sql_data = sqlx::query_as::<_, PeopleSQLData>(
		r#"
		SELECT
			*
		FROM
			people
		WHERE id = $1"#,
	)
	.bind(user_id)
	.fetch_one(&pool)
	.await
	.map_err::<ServerFnError, _>(|error| ServerFnError::ServerError(error.to_string()))?;

	Ok(equipment_sql_data.into())
}
