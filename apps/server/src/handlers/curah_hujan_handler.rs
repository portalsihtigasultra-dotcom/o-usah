use axum::{extract::State, Json};
use o_usah_core::ApiResponse;
use o_usah_models::CurahHujanResponse;

use crate::state::AppState;

pub async fn list(
    State(_state): State<AppState>,
) -> Json<ApiResponse<Vec<CurahHujanResponse>>> {
    Json(ApiResponse::error("Not implemented"))
}

pub async fn create(
    State(_state): State<AppState>,
    Json(_body): Json<serde_json::Value>,
) -> Json<ApiResponse<CurahHujanResponse>> {
    Json(ApiResponse::error("Not implemented"))
}

pub async fn detail(
    State(_state): State<AppState>,
) -> Json<ApiResponse<CurahHujanResponse>> {
    Json(ApiResponse::error("Not implemented"))
}

pub async fn update(
    State(_state): State<AppState>,
    Json(_body): Json<serde_json::Value>,
) -> Json<ApiResponse<CurahHujanResponse>> {
    Json(ApiResponse::error("Not implemented"))
}

pub async fn delete(
    State(_state): State<AppState>,
) -> Json<ApiResponse<()>> {
    Json(ApiResponse::error("Not implemented"))
}

pub async fn update_status(
    State(_state): State<AppState>,
    Json(_body): Json<serde_json::Value>,
) -> Json<ApiResponse<CurahHujanResponse>> {
    Json(ApiResponse::error("Not implemented"))
}

pub async fn validation_queue(
    State(_state): State<AppState>,
) -> Json<ApiResponse<Vec<CurahHujanResponse>>> {
    Json(ApiResponse::error("Not implemented"))
}

pub async fn analisis(
    State(_state): State<AppState>,
) -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::error("Not implemented"))
}

pub async fn export_excel(
    State(_state): State<AppState>,
) -> Json<ApiResponse<()>> {
    Json(ApiResponse::error("Not implemented"))
}

pub async fn export_pdf(
    State(_state): State<AppState>,
) -> Json<ApiResponse<()>> {
    Json(ApiResponse::error("Not implemented"))
}