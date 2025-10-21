use sqlx::PgPool;
use crate::models::{Patient, MedicalRecord};

pub async fn get_all_patients(pool: &PgPool) -> Result<Vec<Patient>, sqlx::Error> {
    let patients = sqlx::query_as!(
        Patient,
        "SELECT id, patient_id, name, personal_id, created_at FROM patients ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(patients)
}

pub async fn get_patient_by_id(pool: &PgPool, id: i32) -> Result<Patient, sqlx::Error> {
    let patient = sqlx::query_as!(
        Patient,
        "SELECT id, patient_id, name, personal_id, created_at FROM patients WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(patient)
}

pub async fn create_patient(
    pool: &PgPool,
    patient_id: String,
    name: String,
    personal_id: Option<String>,
) -> Result<Patient, sqlx::Error> {
    let patient = sqlx::query_as!(
        Patient,
        "INSERT INTO patients (patient_id, name, personal_id) VALUES ($1, $2, $3) RETURNING id, patient_id, name, personal_id, created_at",
        patient_id,
        name,
        personal_id
    )
    .fetch_one(pool)
    .await?;

    Ok(patient)
}

pub async fn get_records_by_patient(pool: &PgPool, patient_id: i32) -> Result<Vec<MedicalRecord>, sqlx::Error> {
    let records = sqlx::query_as!(
        MedicalRecord,
        "SELECT id, patient_id, diagnosis, treatment, created_at FROM medical_records WHERE patient_id = $1 ORDER BY created_at DESC",
        patient_id
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

pub async fn create_record(
    pool: &PgPool,
    patient_id: i32,
    diagnosis: Option<String>,
    treatment: Option<String>,
) -> Result<MedicalRecord, sqlx::Error> {
    let record = sqlx::query_as!(
        MedicalRecord,
        "INSERT INTO medical_records (patient_id, diagnosis, treatment) VALUES ($1, $2, $3) RETURNING id, patient_id, diagnosis, treatment, created_at",
        patient_id,
        diagnosis,
        treatment
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}