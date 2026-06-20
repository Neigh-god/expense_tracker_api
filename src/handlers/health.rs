use axum::{extract::Extension, Json};
use serde::Serialize;

use crate::routes::api::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
}

pub async fn health_check(Extension(_state): Extension<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    })
}
