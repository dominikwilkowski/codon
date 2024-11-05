use crate::{
	app::{LoginAction, LogoutAction},
	error_template::ErrorTemplate,
	icons::{Flask, IncubationCabinet, Vessel},
	login::Login,
};

use leptos::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

stylance::import_style!(css, "home.module.css");

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct EquipmentStats {
	all: i64,
	type_flask: i64,
	type_vessel: i64,
	type_incubation_cabinet: i64,
	status_cleaned: i64,
	status_prepared: i64,
	status_sterilized: i64,
	status_in_use: i64,
	status_dirty: i64,
	status_archived: i64,
}

#[component]
pub fn Home() -> impl IntoView {
	let login_action = use_context::<LoginAction>().expect("No login action found in context");
	let logout_action = use_context::<LogoutAction>().expect("No logout action found in context");

	let equipment_stats = create_resource(
		move || (login_action.version().get(), logout_action.version().get()),
		move |_| get_equipment_stats(),
	);

	view! {
		<Suspense fallback=move || view! { <p>Loading equipment...</p> }>
			<ErrorBoundary fallback=|errors| {
				view! { <ErrorTemplate errors /> }
			}>
				{move || {
					view! {
						{equipment_stats
							.get()
							.map(move |data| match data {
								Err(error) => {
									if error.to_string().contains("User not authenticated") {
										view! { <Login redirect="/" /> }
									} else {
										view! { <pre class="error">Server Error: {error.to_string()}</pre> }.into_view()
									}
								}
								Ok(stats) => {
									view! {
										<h1>Equipment</h1>
										<dl class=css::stats>
											<dt>All:</dt>
											<dd>
												<code class=css::code>{stats.all}</code>
											</dd>

											<dt>
												<Flask />
												Flasks:
											</dt>
											<dd>
												<code class=css::code>{stats.type_flask}</code>
											</dd>

											<dt>
												<Vessel />
												Vessel:
											</dt>
											<dd>
												<code class=css::code>{stats.type_vessel}</code>
											</dd>

											<dt>
												<IncubationCabinet />
												Incubation Cabinet:
											</dt>
											<dd>
												<code class=css::code>{stats.type_incubation_cabinet}</code>
											</dd>

											<dt>Cleaned:</dt>
											<dd>
												<code class=css::code>{stats.status_cleaned}</code>
											</dd>

											<dt>Prepared:</dt>
											<dd>
												<code class=css::code>{stats.status_prepared}</code>
											</dd>

											<dt>Sterillized:</dt>
											<dd>
												<code class=css::code>{stats.status_sterilized}</code>
											</dd>

											<dt>In Use:</dt>
											<dd>
												<code class=css::code>{stats.status_in_use}</code>
											</dd>

											<dt>Dirty:</dt>
											<dd>
												<code class=css::code>{stats.status_dirty}</code>
											</dd>

											<dt>Archived:</dt>
											<dd>
												<code class=css::code>{stats.status_archived}</code>
											</dd>
										</dl>
									}
										.into_view()
								}
							})}
					}
				}}
			</ErrorBoundary>
		</Suspense>
	}
}

#[server(prefix = "/api")]
pub async fn get_equipment_stats() -> Result<EquipmentStats, ServerFnError> {
	use crate::{auth::get_user, permission::Permissions};

	use sqlx::PgPool;

	let pool = use_context::<PgPool>()
		.ok_or_else::<ServerFnError, _>(|| ServerFnError::ServerError(String::from("Database not initialized")))?;
	let user = get_user().await?;

	let auth_query = match user {
		Some(user) => {
			let Permissions::All {
				read: perm,
				write: _,
				create: _,
			} = user.permission_equipment;
			perm.get_query_select_without_where("equipment.id")
		},
		None => return Err(ServerFnError::Request(String::from("User not authenticated"))),
	};

	Ok(
		sqlx::query_as::<_, EquipmentStats>(&format!(
			r#"
			SELECT
				COUNT(*) AS "all",
				COUNT(*) FILTER (WHERE equipment_type = 'Flask') AS "type_flask",
				COUNT(*) FILTER (WHERE equipment_type = 'Vessel') AS "type_vessel",
				COUNT(*) FILTER (WHERE equipment_type = 'IncubationCabinet') AS "type_incubation_cabinet",
				COUNT(*) FILTER (WHERE status = 'Cleaned') AS "status_cleaned",
				COUNT(*) FILTER (WHERE status = 'Prepared') AS "status_prepared",
				COUNT(*) FILTER (WHERE status = 'Sterilized') AS "status_sterilized",
				COUNT(*) FILTER (WHERE status = 'InUse') AS "status_in_use",
				COUNT(*) FILTER (WHERE status = 'Dirty') AS "status_dirty",
				COUNT(*) FILTER (WHERE status = 'Archived') AS "status_archived"
			FROM equipment WHERE id IS NOT NULL {auth_query}
		"#
		))
		.fetch_one(&pool)
		.await?,
	)
}
