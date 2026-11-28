mod config;
mod error;
mod holds;
mod store;

use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::cors::CorsLayer;

use crate::config::Config;

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
            eprintln!("Cannot connect to the database: {e}");
            eprintln!("Is the database running?");
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

    let app = create_router(cfg.cors_origins.clone());

    listen(app, cfg).await;
}

fn create_router(cors_origins: Vec<HeaderValue>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(cors_origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .expose_headers([header::LOCATION, header::ETAG])
        .allow_credentials(true);

    Router::new()
        .route("/api/v1/events/{id}/holds", post(holds::create_hold))
        .fallback(error::not_found_route)
        .method_not_allowed_fallback(error::wrong_method)
        .layer(cors)
}

async fn listen(app: Router, cfg: Config) {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], cfg.port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind failed");

    tracing::info!("listening on http://{addr}");
    axum::serve(listener, app).await.expect("server failed");
}
