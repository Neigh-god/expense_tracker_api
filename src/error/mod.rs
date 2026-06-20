use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Auth error: {0}")]
    Auth(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal,
}

pub type Result<T> = std::result::Result<T, AppError>;

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        let message = errors
            .field_errors()
            .iter()
            .map(|(field, errs)| {
                let msgs: Vec<String> = errs.iter().map(|e| {
                    e.message.clone().unwrap_or_else(|| "invalid value".into()).to_string()
                }).collect();
                format!("{}: {}", field, msgs.join(", "))
            })
            .collect::<Vec<_>>()
            .join("; ");
        AppError::Validation(message)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "success": false,
            "error": message
        }));

        (status, body).into_response()
    }
}
