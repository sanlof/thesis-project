use sqlx::PgPool;
use crate::models::{Case, Suspect};

pub async fn get_all_cases(pool: &PgPool) -> Result<Vec<Case>, sqlx::Error> {
    let cases = sqlx::query_as!(
        Case,
        "SELECT id, case_number, status, description, created_at FROM cases ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(cases)
}

pub async fn get_case_by_id(pool: &PgPool, id: i32) -> Result<Case, sqlx::Error> {
    let case = sqlx::query_as!(
        Case,
        "SELECT id, case_number, status, description, created_at FROM cases WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(case)
}

pub async fn create_case(
    pool: &PgPool,
    case_number: String,
    status: String,
    description: Option<String>,
) -> Result<Case, sqlx::Error> {
    let case = sqlx::query_as!(
        Case,
        "INSERT INTO cases (case_number, status, description) VALUES ($1, $2, $3) RETURNING id, case_number, status, description, created_at",
        case_number,
        status,
        description
    )
    .fetch_one(pool)
    .await?;

    Ok(case)
}

pub async fn get_suspects_by_case(pool: &PgPool, case_id: i32) -> Result<Vec<Suspect>, sqlx::Error> {
    let suspects = sqlx::query_as!(
        Suspect,
        "SELECT id, case_id, name, personal_id, created_at FROM suspects WHERE case_id = $1 ORDER BY created_at DESC",
        case_id
    )
    .fetch_all(pool)
    .await?;

    Ok(suspects)
}

pub async fn create_suspect(
    pool: &PgPool,
    case_id: i32,
    name: String,
    personal_id: Option<String>,
) -> Result<Suspect, sqlx::Error> {
    let suspect = sqlx::query_as!(
        Suspect,
        "INSERT INTO suspects (case_id, name, personal_id) VALUES ($1, $2, $3) RETURNING id, case_id, name, personal_id, created_at",
        case_id,
        name,
        personal_id
    )
    .fetch_one(pool)
    .await?;

    Ok(suspect)
}