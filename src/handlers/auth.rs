use axum::{extract::Extension, http::StatusCode, Json};
use validator::Validate;

use crate::error::AppError;
use crate::models::user::{CreateUserRequest, LoginRequest, UserResponse};
use crate::routes::api::AppState;
use crate::utils::jwt::generate_token;
use crate::utils::password::{hash_password, verify_password};

pub async fn register(
    Extension(state): Extension<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    payload.validate()?;

    let password_hash = hash_password(&payload.password)
        .map_err(|_e| AppError::Internal)?;

    let user = sqlx::query_as!(
        UserResponse,
        r#"
        INSERT INTO users (id, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, email
        "#,
        uuid::Uuid::new_v4(),
        payload.email,
        password_hash,
    )
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login(
    Extension(state): Extension<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    payload.validate()?;

    let user = sqlx::query!(
        r#"
        SELECT id, email, password_hash
        FROM users
        WHERE email = $1
        "#,
        payload.email,
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Auth("Invalid credentials".to_string()))?;

    let valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|_| AppError::Auth("Invalid credentials".to_string()))?;

    if !valid {
        return Err(AppError::Auth("Invalid credentials".to_string()));
    }

    let token = generate_token(user.id, &state.config)
        .map_err(|_| AppError::Internal)?;

    Ok(Json(serde_json::json!({
        "success": true,
        "token": token,
        "user": {
            "id": user.id,
            "email": user.email
        }
    })))
}
