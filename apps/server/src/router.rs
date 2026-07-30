use axum::{http::Method, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers(Any);

    Router::new()
        .route("/health", axum::routing::get(health_check))
        .layer(cors)
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}