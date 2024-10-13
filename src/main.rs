#[macro_use]
pub mod macros;

pub mod app;
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
pub mod nav;
pub mod qrcode;
pub mod samples;
pub mod utils;

#[cfg(feature = "ssr")]
pub mod fileserv;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
	pub id: i32,
	pub anonymous: bool,
	pub username: String,
	pub permissions: HashSet<String>,
}

impl Default for User {
	fn default() -> Self {
		let mut permissions = HashSet::new();

		permissions.insert("Category::View".to_owned());

		Self {
			id: 1,
			anonymous: true,
			username: "Guest".into(),
			permissions,
		}
	}
}

#[allow(clippy::needless_return)]
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
	use crate::{
		app::App,
		db::ssr::{get_db, init_db},
		fileserv::file_and_error_handler,
	};

	use axum::Router;
	use axum_session::{SessionConfig, SessionLayer, SessionStore};
	use axum_session_auth::*;
	use axum_session_sqlx::SessionPgPool;
	use leptos::*;
	use leptos_axum::{generate_route_list, LeptosRoutes};
	use sqlx::{migrate::Migrator, PgPool};

	// Init the pool into static
	init_db().await.expect("Initialization of database failed");

	let session_config = SessionConfig::default().with_table_name("codon_session");
	let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
	let session_store = SessionStore::<SessionPgPool>::new(Some(get_db().clone().into()), session_config)
		.await
		.expect("Failed to create session store");

	static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
	if let Err(e) = MIGRATOR.run(get_db()).await {
		eprintln!("{e:?}");
	}

	// Setting get_configuration(None) means we'll be using cargo-leptos's env values
	// For deployment these variables are:
	// https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain
	// Alternately a file can be specified such as Some("Cargo.toml")
	// The file would need to be included with the executable when moved to deployment
	let conf = get_configuration(None).await.unwrap();
	let leptos_options = conf.leptos_options;
	let addr = leptos_options.site_addr;
	let routes = generate_route_list(App);

	// build our application with a route
	let app = Router::new()
		.leptos_routes(&leptos_options, routes, App)
		.fallback(file_and_error_handler)
		.with_state(leptos_options)
		.layer(AuthSessionLayer::<User, i64, SessionPgPool, PgPool>::new(Some(get_db().clone())).with_config(auth_config))
		.layer(SessionLayer::new(session_store));

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
	// no client-side main function
	// unless we want this to work with e.g., Trunk for a purely client-side app
	// see lib.rs for hydration function instead
}
