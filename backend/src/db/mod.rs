// Database connection and management
// Uses SQLx with Supabase PostgreSQL

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

const DEFAULT_DATABASE_URL: &str = "postgres://user:password@localhost/aevalo_db";
const DEFAULT_MAX_CONNECTIONS: u32 = 5;
const DEFAULT_MIN_CONNECTIONS: u32 = 1;
const DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 10;
const DEFAULT_ACQUIRE_TIMEOUT_SECS: u64 = 5;
const DEFAULT_IDLE_TIMEOUT_SECS: u64 = 300;
const DEFAULT_MAX_LIFETIME_SECS: u64 = 1800;

pub struct Database {
	pool: PgPool,
}

impl Database {
	pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
		let pool = pool_options()
			.connect(database_url)
			.await?;

		Ok(Self { pool })
	}

	pub fn pool(&self) -> &PgPool {
		&self.pool
	}
}

pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
	let database_url = database_url_from_env();

	pool_options()
		.connect(&database_url)
		.await
}

pub async fn health_check(pool: &PgPool) -> Result<(), sqlx::Error> {
	sqlx::query("SELECT 1").execute(pool).await?;
	Ok(())
}

fn database_url_from_env() -> String {
	std::env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_string())
}

fn max_connections_from_env() -> u32 {
	std::env::var("DB_MAX_CONNECTIONS")
		.ok()
		.and_then(|value| value.parse::<u32>().ok())
		.filter(|value| *value > 0)
		.unwrap_or(DEFAULT_MAX_CONNECTIONS)
}

fn pool_options() -> PgPoolOptions {
	PgPoolOptions::new()
		.max_connections(max_connections_from_env())
		.min_connections(min_connections_from_env())
		.acquire_timeout(Duration::from_secs(acquire_timeout_from_env()))
		.idle_timeout(Some(Duration::from_secs(idle_timeout_from_env())))
		.max_lifetime(Some(Duration::from_secs(max_lifetime_from_env())))
}

fn min_connections_from_env() -> u32 {
	std::env::var("DB_MIN_CONNECTIONS")
		.ok()
		.and_then(|value| value.parse::<u32>().ok())
		.unwrap_or(DEFAULT_MIN_CONNECTIONS)
}

fn connect_timeout_from_env() -> u64 {
	std::env::var("DB_CONNECT_TIMEOUT_SECS")
		.ok()
		.and_then(|value| value.parse::<u64>().ok())
		.filter(|value| *value > 0)
		.unwrap_or(DEFAULT_CONNECT_TIMEOUT_SECS)
}

fn acquire_timeout_from_env() -> u64 {
	std::env::var("DB_ACQUIRE_TIMEOUT_SECS")
		.ok()
		.and_then(|value| value.parse::<u64>().ok())
		.filter(|value| *value > 0)
		.unwrap_or(DEFAULT_ACQUIRE_TIMEOUT_SECS)
}

fn idle_timeout_from_env() -> u64 {
	std::env::var("DB_IDLE_TIMEOUT_SECS")
		.ok()
		.and_then(|value| value.parse::<u64>().ok())
		.unwrap_or(DEFAULT_IDLE_TIMEOUT_SECS)
}

fn max_lifetime_from_env() -> u64 {
	std::env::var("DB_MAX_LIFETIME_SECS")
		.ok()
		.and_then(|value| value.parse::<u64>().ok())
		.unwrap_or(DEFAULT_MAX_LIFETIME_SECS)
}
