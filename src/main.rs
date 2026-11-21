mod config;

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let cfg = match config::Config::from_env() {
        Ok(cfg) => cfg,
        Err(msg) => {
            eprintln!("configuration error: {msg}");
            std::process::exit(1);
        }
    };

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .min_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&cfg.database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("cannot connect to the database: {e}");
            eprintln!("is the course stack running? (handout, section 2)");
            std::process::exit(1);
        }
    };

    match sqlx::query_scalar::<_, String>("SELECT version()")
        .fetch_one(&pool)
        .await
    {
        Ok(version) => tracing::info!(%version, "connected to the database"),
        Err(e) => {
            eprintln!("database connection check failed: {e}");
            std::process::exit(1);
        }
    }
}
