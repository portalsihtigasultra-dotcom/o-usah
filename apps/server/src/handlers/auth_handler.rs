use axum::{extract::State, Json};
use o_usah_core::ApiResponse;
use o_usah_models::{AuthResponse, LoginDto};

use crate::state::AppState;

pub async fn register(
    State(_state): State<AppState>,
    Json(_body): Json<LoginDto>,
) -> Json<ApiResponse<()>> {
    Json(ApiResponse::error("Not implemented"))
}

pub async fn login(
    State(_state): State<AppState>,
    Json(_body): Json<LoginDto>,
) -> Json<ApiResponse<AuthResponse>> {
    Json(ApiResponse::error("Not implemented"))
}