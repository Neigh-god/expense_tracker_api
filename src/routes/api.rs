use axum::{extract::Request, http::header, middleware::{self, Next}, Extension, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::error::AppError;
use crate::handlers::{
    auth::{login, register},
    expense::{create_expense, delete_expense, get_expense, list_expenses, update_expense},
    health::health_check,
};
use crate::utils::jwt::verify_token;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
}

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
}

pub fn create_api_router(pool: PgPool, config: Config) -> Router {
    let state = AppState { pool, config };

    Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/auth/register", axum::routing::post(register))
        .route("/auth/login", axum::routing::post(login))
        .route("/expenses", axum::routing::get(list_expenses).post(create_expense))
        .route("/expenses/{id}", axum::routing::get(get_expense).put(update_expense).delete(delete_expense))
        .layer(middleware::from_fn({
            let state = state.clone();
            move |mut req: Request, next: Next| {
                let state = state.clone();
                async move {
                    let path = req.uri().path();
                    
                    // Skip auth for public routes
                    if path == "/health" || path == "/auth/register" || path == "/auth/login" {
                        return Ok::<_, AppError>(next.run(req).await);
                    }

                    let auth_header = req
                        .headers()
                        .get(header::AUTHORIZATION)
                        .and_then(|h| h.to_str().ok())
                        .ok_or(AppError::Unauthorized)?;

                    let token = auth_header
                        .strip_prefix("Bearer ")
                        .ok_or(AppError::Unauthorized)?;

                    let claims = verify_token(token, &state.config)
                        .map_err(|_| AppError::Unauthorized)?;

                    let user = sqlx::query!(
                        "SELECT id FROM users WHERE id = $1",
                        claims.sub
                    )
                    .fetch_optional(&state.pool)
                    .await
                    .map_err(|_| AppError::Unauthorized)?
                    .ok_or(AppError::Unauthorized)?;

                    req.extensions_mut().insert(AuthUser { user_id: user.id });

                    Ok(next.run(req).await)
                }
            }
        }))
        .layer(Extension(state))
}
