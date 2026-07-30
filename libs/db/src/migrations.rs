use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(include_str!("../../../migrations/000_init.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../../../migrations/001_users.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../../../migrations/002_pos_pengamatan.sql"))
        .execute(pool)
        .await?;

    sqlx::query(include_str!("../../../migrations/003_curah_hujan.sql"))
        .execute(pool)
        .await?;

    Ok(())
}