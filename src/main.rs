mod config;
mod store;

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

    if let Err(e) = store::migrator().run(&pool).await {
        eprintln!("the migrations failed: {}", store::migration_failure(&e));
        std::process::exit(1);
    }
    match store::schema_version(&pool).await {
        Ok(version) => tracing::info!("migrations checked, the database is at version {version}"),
        Err(e) => tracing::warn!("cannot read the version of the database: {e}"),
    }
}
