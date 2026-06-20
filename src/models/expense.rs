use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: String,
    pub category: String,
    pub description: Option<String>,
    pub date: NaiveDate,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateExpenseRequest {
    #[validate(length(min = 1, message = "Amount is required"))]
    pub amount: String,
    
    #[validate(length(min = 1, max = 50, message = "Category must be 1-50 characters"))]
    pub category: String,
    
    pub description: Option<String>,
    
    pub date: NaiveDate,
}

#[derive(Debug, Deserialize, Validate, Default)]
pub struct UpdateExpenseRequest {
    #[validate(length(min = 1, message = "Amount cannot be empty if provided"))]
    pub amount: Option<String>,
    
    #[validate(length(min = 1, max = 50, message = "Category must be 1-50 characters"))]
    pub category: Option<String>,
    
    pub description: Option<String>,
    
    pub date: Option<NaiveDate>,
}
