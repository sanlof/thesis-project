use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Suspect {
    pub id: i32,
    pub case_id: i32,
    pub name: String,
    pub personal_id: Option<String>,
    pub created_at: NaiveDateTime,
}