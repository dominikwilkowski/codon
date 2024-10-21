#[macro_use]
pub mod macros;

pub mod app;
pub mod auth;
pub mod components {
	pub mod avatar;
	pub mod button;
	pub mod checkbox;
	pub mod datepicker;
	pub mod dropdown;
	pub mod file_input;
	pub mod file_upload;
	pub mod img_attachment;
	pub mod input;
	pub mod multiline;
	pub mod pagination;
	pub mod qr_scanner;
	pub mod radio;
	pub mod select;
	pub mod switch;
	pub mod timezone_offset;
}
pub mod db;
pub mod ds;
pub mod equipment;
pub mod error_template;
pub mod footer;
pub mod header;
pub mod home;
pub mod icons;
pub mod login;
pub mod nav;
pub mod permission;
pub mod qrcode;
pub mod samples;
pub mod utils;

#[cfg(feature = "ssr")]
pub mod fileserv;

#[cfg(feature = "ssr")]
use crate::{
	app::App,
	auth::{ssr::AuthSession, User},
	fileserv::file_and_error_handler,
};

#[cfg(feature = "ssr")]
use axum::{
	body::Body as AxumBody,
	extract::{FromRef, Path, State},
	http::Request,
	response::{IntoResponse, Response},
	routing::get,
	Router,
};
#[cfg(feature = "ssr")]
use axum_session::{SessionConfig, SessionLayer, SessionStore};
#[cfg(feature = "ssr")]
use axum_session_auth::{AuthConfig, AuthSessionLayer};
#[cfg(feature = "ssr")]
pub use axum_session_sqlx::SessionPgPool;
use leptos::*;
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, handle_server_fns_with_context, render_route_with_context, LeptosRoutes};
#[cfg(feature = "ssr")]
use leptos_router::RouteListing;
#[cfg(feature = "ssr")]
use sqlx::{migrate::Migrator, PgPool};

#[cfg_attr(feature = "ssr", derive(FromRef, Debug, Clone))]
pub struct AppState {
	pub leptos_options: LeptosOptions,
	#[cfg(feature = "ssr")]
	pub routes: Vec<RouteListing>,
	#[cfg(feature = "ssr")]
	pub pool: PgPool,
}

#[cfg(feature = "ssr")]
async fn server_fn_handler(
	State(app_state): State<AppState>,
	auth_session: AuthSession,
	_path: Path<String>,
	request: Request<AxumBody>,
) -> impl IntoResponse {
	handle_server_fns_with_context(
		move || {
			provide_context(auth_session.clone());
			provide_context(app_state.pool.clone());
		},
		request,
	)
	.await
}

#[cfg(feature = "ssr")]
async fn leptos_routes_handler(
	auth_session: AuthSession,
	State(app_state): State<AppState>,
	req: Request<AxumBody>,
) -> Response {
	let handler = render_route_with_context(
		app_state.leptos_options.clone(),
		app_state.routes.clone(),
		move || {
			provide_context(auth_session.clone());
			provide_context(app_state.pool.clone());
		},
		App,
	);
	handler(req).await.into_response()
}

#[allow(clippy::needless_return)]
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
	use crate::db::ssr::{get_db, init_db};

	// Init the Postgres pool into static
	init_db().await.expect("Initialization of database failed");

	static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

	if let Err(e) = MIGRATOR.run(get_db()).await {
		eprintln!("{e:?}");
	}

	// Auth section
	let session_config = SessionConfig::default().with_table_name("axum_sessions");
	let auth_config = AuthConfig::<i32>::default();
	let session_store =
		SessionStore::<SessionPgPool>::new(Some(SessionPgPool::from(get_db().clone())), session_config).await.unwrap();

	// Setting get_configuration(None) means we'll be using cargo-leptos's env values
	// For deployment these variables are:
	// https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain
	// Alternately a file can be specified such as Some("Cargo.toml")
	// The file would need to be included with the executable when moved to deployment
	let conf = get_configuration(None).await.unwrap();
	let leptos_options = conf.leptos_options;
	let addr = leptos_options.site_addr;
	let routes = generate_route_list(App);

	let app_state = AppState {
		leptos_options,
		routes: routes.clone(),
		pool: get_db().clone(),
	};

	// build our application with a route
	let app = Router::new()
		.route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
		.leptos_routes_with_handler(routes, get(leptos_routes_handler))
		.fallback(file_and_error_handler)
		.layer(AuthSessionLayer::<User, i32, SessionPgPool, PgPool>::new(Some(get_db().clone())).with_config(auth_config))
		.layer(SessionLayer::new(session_store))
		.with_state(app_state);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
	// no client-side main function
	// unless we want this to work with e.g., Trunk for a purely client-side app
	// see lib.rs for hydration function instead
}
