use o_usah_db::{create_pool, run_migrations};

use crate::router::build_router;
use crate::state::AppState;

mod handlers;
mod middleware;
mod router;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "o_usah_server=debug,tower_http=debug".into()),
        )
        .init();

    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let jwt_secret =
        std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
    let host =
        std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port =
        std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".into());

    let pool = create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database connected and migrations applied");

    let state = AppState::new(pool, jwt_secret);
    let app = build_router(state);

    let addr = format!("{host}:{port}");
    tracing::info!("O'USAH server starting on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}