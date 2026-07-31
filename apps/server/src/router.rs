use axum::{http::Method, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::{auth_handler, curah_hujan_handler};
use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers(Any);

    Router::new()
        .route("/health", axum::routing::get(health_check))
        .nest("/api/auth", auth_routes())
        .nest("/api/curah-hujan", curah_hujan_routes())
        .layer(cors)
        .with_state(state)
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", axum::routing::post(auth_handler::register))
        .route("/login", axum::routing::post(auth_handler::login))
}

fn curah_hujan_routes() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(curah_hujan_handler::list))
        .route("/", axum::routing::post(curah_hujan_handler::create))
        .route("/{id}", axum::routing::get(curah_hujan_handler::detail))
        .route("/{id}", axum::routing::put(curah_hujan_handler::update))
        .route("/{id}", axum::routing::delete(curah_hujan_handler::delete))
        .route("/{id}/status", axum::routing::patch(curah_hujan_handler::update_status))
        .route("/validation-queue", axum::routing::get(curah_hujan_handler::validation_queue))
        .route("/analisis", axum::routing::get(curah_hujan_handler::analisis))
        .route("/export/excel", axum::routing::get(curah_hujan_handler::export_excel))
        .route("/export/pdf", axum::routing::get(curah_hujan_handler::export_pdf))
}

async fn health_check() -> &'static str {
    "OK"
}