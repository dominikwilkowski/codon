#[cfg(feature = "ssr")]
pub mod ssr {
	use dotenvy::dotenv;
	use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

	static DB: std::sync::OnceLock<PgPool> = std::sync::OnceLock::new();

	async fn create_pool() -> PgPool {
		dotenv().ok();

		let database_url = std::env::var("DATABASE_URL").expect("No database url found in environment");
		let pool = PgPoolOptions::new()
			.max_connections(5)
			.connect(database_url.as_str())
			.await
			.expect("Unable to connect to database");

		pool
	}

	pub async fn init_db() -> Result<(), Pool<Postgres>> {
		DB.set(create_pool().await)
	}

	pub fn get_db<'a>() -> &'a PgPool {
		DB.get().expect("Database has not been initialized")
	}
}
