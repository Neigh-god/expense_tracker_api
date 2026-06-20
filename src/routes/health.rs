use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    message: String,
}

pub async fn health_check() -> Json<HealthResponse> {

    Json(HealthResponse {
        message: "Expense Tracker API Running".to_string(),
    })
}