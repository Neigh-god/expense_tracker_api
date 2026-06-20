use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::error::AppError;
use crate::models::expense::{CreateExpenseRequest, Expense, UpdateExpenseRequest};
use crate::routes::api::{AppState, AuthUser};

#[derive(Debug, Deserialize)]
pub struct ListExpensesQuery {
    pub category: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn create_expense(
    Extension(state): Extension<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(payload): Json<CreateExpenseRequest>,
) -> Result<(StatusCode, Json<Expense>), AppError> {
    payload.validate()?;

    let expense = sqlx::query_as!(
        Expense,
        r#"
        INSERT INTO expenses (id, user_id, amount, category, description, date)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, amount, category, description, date, created_at, updated_at
        "#,
        Uuid::new_v4(),
        auth_user.user_id,
        payload.amount,
        payload.category,
        payload.description,
        payload.date,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| AppError::Database(e))?;

    Ok((StatusCode::CREATED, Json(expense)))
}

pub async fn list_expenses(
    Extension(state): Extension<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Query(params): Query<ListExpensesQuery>,
) -> Result<Json<Vec<Expense>>, AppError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);

    let expenses = sqlx::query_as!(
        Expense,
        r#"
        SELECT id, user_id, amount, category, description, date, created_at, updated_at
        FROM expenses
        WHERE user_id = $1
        ORDER BY date DESC
        LIMIT $2 OFFSET $3
        "#,
        auth_user.user_id,
        limit,
        offset,
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(expenses))
}

pub async fn get_expense(
    Extension(state): Extension<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<Expense>, AppError> {
    let expense = sqlx::query_as!(
        Expense,
        r#"
        SELECT id, user_id, amount, category, description, date, created_at, updated_at
        FROM expenses
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        auth_user.user_id,
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound(format!("Expense with id {} not found", id)))?;

    Ok(Json(expense))
}

pub async fn update_expense(
    Extension(state): Extension<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateExpenseRequest>,
) -> Result<Json<Expense>, AppError> {
    payload.validate()?;

    let expense = sqlx::query_as!(
        Expense,
        r#"
        UPDATE expenses
        SET
            amount = COALESCE($1, amount),
            category = COALESCE($2, category),
            description = COALESCE($3, description),
            date = COALESCE($4, date),
            updated_at = NOW()
        WHERE id = $5 AND user_id = $6
        RETURNING id, user_id, amount, category, description, date, created_at, updated_at
        "#,
        payload.amount,
        payload.category,
        payload.description,
        payload.date,
        id,
        auth_user.user_id,
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound(format!("Expense with id {} not found", id)))?;

    Ok(Json(expense))
}

pub async fn delete_expense(
    Extension(state): Extension<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query!(
        r#"
        DELETE FROM expenses
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        auth_user.user_id,
    )
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Expense with id {} not found",
            id
        )));
    }

    Ok(StatusCode::NO_CONTENT)
}
