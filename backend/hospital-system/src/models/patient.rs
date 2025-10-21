use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Patient {
    pub id: i32,
    pub patient_id: String,
    pub name: String,
    pub personal_id: Option<String>,
    pub created_at: NaiveDateTime,
}