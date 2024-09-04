pub mod db;
pub mod error_template;
pub mod home {
	pub mod home;
}
pub mod samples {
	pub mod samples;
}
pub mod nav {
	pub mod nav;
}
pub mod app {
	pub mod app;
}
pub mod equipment {
	pub mod equipment;
	pub mod equipment_detail;
	pub mod row;
	pub mod schema;
}
pub mod icons {
	pub mod culture;
	pub mod equipment;
	pub mod experiment;
	pub mod flask;
	pub mod incubation_cabinet;
	pub mod people;
	pub mod vessel;
}
pub mod components {
	pub mod pagination {
		pub mod pagination;
	}
	pub mod table {
		pub mod table;
	}
}

#[cfg(feature = "ssr")]
pub mod fileserv;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
	use sqlx::migrate::Migrator;
	static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

	use crate::{
		app::app::App,
		db::ssr::{get_db, init_db},
		fileserv::file_and_error_handler,
	};

	use axum::Router;
	use leptos::*;
	use leptos_axum::{generate_route_list, LeptosRoutes};

	// Init the pool into static
	init_db().await.expect("Initialization of database failed");

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
		.with_state(leptos_options);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
	// no client-side main function
	// unless we want this to work with e.g., Trunk for a purely client-side app
	// see lib.rs for hydration function instead
}
