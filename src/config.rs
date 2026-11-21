use axum::http::HeaderValue;

pub struct Config {
    pub port: u16,
    /// The PostgreSQL of the course stack, by default.
    pub database_url: String,
    /// Number of events in a page when the request sends no `limit`.
    pub page_size: u32,
    /// How long a hold is meant to last, used to compute `expiresAt`.
    pub hold_ttl_seconds: i64,
    /// The Active holds that one customer can have at the same time.
    pub max_holds_per_customer: usize,
    /// The exact origins whose pages may call this service, with credentials.
    pub cors_origins: Vec<HeaderValue>,
}

impl Config {
    pub fn from_env() -> Result<Config, String> {
        let port = match std::env::var("INVENTORYORDERS_PORT") {
            Ok(raw) => raw
                .parse::<u16>()
                .map_err(|_| format!("INVENTORYORDERS_PORT is not a valid port: {raw:?}"))?,
            Err(_) => 3000,
        };
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://INVENTORYORDERS:INVENTORYORDERS-dev-secret@localhost:5433/INVENTORYORDERS".to_string());
        let page_size = match std::env::var("INVENTORYORDERS_PAGE_SIZE") {
            Ok(raw) => {
                let size = raw
                    .parse::<u32>()
                    .map_err(|_| format!("INVENTORYORDERS_PAGE_SIZE is not a number: {raw:?}"))?;
                if !(1..=100).contains(&size) {
                    return Err(format!(
                        "INVENTORYORDERS_PAGE_SIZE must be within 1..=100, got {size}"
                    ));
                }
                size
            }
            Err(_) => 50,
        };
        let hold_ttl_seconds = match std::env::var("INVENTORYORDERS_HOLD_TTL_SECONDS") {
            Ok(raw) => {
                let ttl = raw.parse::<i64>().map_err(|_| {
                    format!("INVENTORYORDERS_HOLD_TTL_SECONDS is not a number: {raw:?}")
                })?;
                if !(10..=3600).contains(&ttl) {
                    return Err(format!(
                        "INVENTORYORDERS_HOLD_TTL_SECONDS must be within 10..=3600, got {ttl}"
                    ));
                }
                ttl
            }
            Err(_) => 120,
        };
        let max_holds_per_customer = match std::env::var("INVENTORYORDERS_HOLD_MAX_PER_CUSTOMER") {
            Ok(raw) => {
                let max = raw.parse::<usize>().map_err(|_| {
                    format!("INVENTORYORDERS_HOLD_MAX_PER_CUSTOMER is not a number: {raw:?}")
                })?;
                if !(1..=20).contains(&max) {
                    return Err(format!(
                        "INVENTORYORDERS_HOLD_MAX_PER_CUSTOMER must be within 1..=20, got {max}"
                    ));
                }
                max
            }
            Err(_) => 4,
        };
        let raw = std::env::var("INVENTORYORDERS_CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173".to_string());
        let mut cors_origins = Vec::new();
        for origin in raw.split(',').map(str::trim).filter(|o| !o.is_empty()) {
            if origin == "*" {
                return Err("INVENTORYORDERS_CORS_ORIGINS cannot be * with credentials".to_string());
            }
            let value = origin.parse::<HeaderValue>().map_err(|_| {
                format!("INVENTORYORDERS_CORS_ORIGINS has an invalid origin: {origin:?}")
            })?;
            cors_origins.push(value);
        }
        if cors_origins.is_empty() {
            return Err("INVENTORYORDERS_CORS_ORIGINS is empty".to_string());
        }
        Ok(Config {
            port,
            database_url,
            page_size,
            hold_ttl_seconds,
            max_holds_per_customer,
            cors_origins,
        })
    }
}
