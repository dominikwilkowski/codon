#[cfg(feature = "ssr")]
pub mod ssr {
	// use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
	use leptos::server_fn::ServerFnError;
	use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

	pub async fn db() -> Result<Pool<Postgres>, ServerFnError> {
		Ok(
			PgPoolOptions::new()
				.connect("postgres://admin:codon-admin@127.0.0.1:5432/codon")
				.await?,
		)
	}
}
