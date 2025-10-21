use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Case {
    pub id: i32,
    pub case_number: String,
    pub status: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}