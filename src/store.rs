use sqlx::{
    PgPool,
    migrate::{MigrateError, Migrator},
    postgres::PgDatabaseError,
};

pub fn migrator() -> Migrator {
    let mut migrator = sqlx::migrate!();
    migrator.set_ignore_missing(true);
    migrator
}

pub fn migration_failure(e: &MigrateError) -> String {
    let MigrateError::ExecuteMigration(sqlx::Error::Database(db), version) = e else {
        return e.to_string();
    };
    let detail = db
        .try_downcast_ref::<PgDatabaseError>()
        .and_then(|pg| pg.detail())
        .unwrap_or("");
    format!("migration {version}: {}\n{detail}", db.message())
}

pub async fn schema_version(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT coalesce(max(version), 0) FROM _sqlx_migrations WHERE success")
        .fetch_one(pool)
        .await
}
