use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MedicalRecord {
    pub id: i32,
    pub patient_id: i32,
    pub diagnosis: Option<String>,
    pub treatment: Option<String>,
    pub created_at: NaiveDateTime,
}