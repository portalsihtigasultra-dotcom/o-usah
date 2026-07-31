use sqlx::migrate::MigrateError;
use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}