use sqlx::PgPool;
use crate::models::{Patient, CreatePatient, UpdatePatient};

/// Retrieves all patients from the database
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// 
/// # Returns
/// 
/// * `Result<Vec<Patient>, sqlx::Error>` - List of all patients
pub async fn get_all_patients(pool: &PgPool) -> Result<Vec<Patient>, sqlx::Error> {
    let patients = sqlx::query_as!(
        Patient,
        "SELECT id, full_name, personal_id, flag FROM patients ORDER BY id"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(patients)
}

/// Retrieves a patient by their database ID
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// * `id` - Patient's database ID
/// 
/// # Returns
/// 
/// * `Result<Option<Patient>, sqlx::Error>` - Patient if found, None otherwise
pub async fn get_patient_by_id(pool: &PgPool, id: i32) -> Result<Option<Patient>, sqlx::Error> {
    let patient = sqlx::query_as!(
        Patient,
        "SELECT id, full_name, personal_id, flag FROM patients WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(patient)
}

/// Retrieves a patient by their Swedish personal ID
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// * `personal_id` - Swedish personal ID (YYYYMMDD-XXXX)
/// 
/// # Returns
/// 
/// * `Result<Option<Patient>, sqlx::Error>` - Patient if found, None otherwise
pub async fn get_patient_by_personal_id(
    pool: &PgPool,
    personal_id: &str,
) -> Result<Option<Patient>, sqlx::Error> {
    let patient = sqlx::query_as!(
        Patient,
        "SELECT id, full_name, personal_id, flag FROM patients WHERE personal_id = $1",
        personal_id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(patient)
}

/// Creates a new patient in the database
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// * `patient` - Patient data to create
/// 
/// # Returns
/// 
/// * `Result<Patient, sqlx::Error>` - Created patient with generated ID
pub async fn create_patient(
    pool: &PgPool,
    patient: CreatePatient,
) -> Result<Patient, sqlx::Error> {
    let created_patient = sqlx::query_as!(
        Patient,
        "INSERT INTO patients (full_name, personal_id, flag) 
         VALUES ($1, $2, $3) 
         RETURNING id, full_name, personal_id, flag",
        patient.full_name,
        patient.personal_id,
        patient.flag
    )
    .fetch_one(pool)
    .await?;
    
    Ok(created_patient)
}

/// Updates an existing patient by ID
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// * `id` - Patient's database ID
/// * `patient` - Updated patient data
/// 
/// # Returns
/// 
/// * `Result<Option<Patient>, sqlx::Error>` - Updated patient if found, None otherwise
pub async fn update_patient(
    pool: &PgPool,
    id: i32,
    patient: UpdatePatient,
) -> Result<Option<Patient>, sqlx::Error> {
    // Check if patient exists first
    let existing = get_patient_by_id(pool, id).await?;
    if existing.is_none() {
        return Ok(None);
    }
    
    // Update fields if provided, otherwise keep existing values
    let updated_patient = sqlx::query_as!(
        Patient,
        "UPDATE patients 
         SET full_name = COALESCE($1, full_name),
             personal_id = COALESCE($2, personal_id),
             flag = COALESCE($3, flag)
         WHERE id = $4
         RETURNING id, full_name, personal_id, flag",
        patient.full_name,
        patient.personal_id,
        patient.flag,
        id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(updated_patient)
}

/// Deletes a patient by ID
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// * `id` - Patient's database ID
/// 
/// # Returns
/// 
/// * `Result<bool, sqlx::Error>` - true if deleted, false if not found
pub async fn delete_patient(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM patients WHERE id = $1",
        id
    )
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected() > 0)
}

/// Retrieves all patients with flag set to true
/// This is particularly useful for identifying patients flagged by the police system
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// 
/// # Returns
/// 
/// * `Result<Vec<Patient>, sqlx::Error>` - List of flagged patients
pub async fn get_flagged_patients(pool: &PgPool) -> Result<Vec<Patient>, sqlx::Error> {
    let flagged_patients = sqlx::query_as!(
        Patient,
        "SELECT id, full_name, personal_id, flag 
         FROM patients 
         WHERE flag = true 
         ORDER BY id"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(flagged_patients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires database to be running
    async fn test_crud_operations() {
        dotenv::dotenv().ok();
        let pool = crate::database::connection::establish_connection()
            .await
            .expect("Failed to connect to database");

        // Test create
        let new_patient = CreatePatient {
            full_name: "Test Patient".to_string(),
            personal_id: "19990101-1234".to_string(),
            flag: false,
        };
        
        let created = create_patient(&pool, new_patient).await.unwrap();
        assert_eq!(created.full_name, "Test Patient");

        // Test get by id
        let retrieved = get_patient_by_id(&pool, created.id).await.unwrap();
        assert!(retrieved.is_some());

        // Test delete
        let deleted = delete_patient(&pool, created.id).await.unwrap();
        assert!(deleted);
    }

    #[tokio::test]
    #[ignore] // Requires database to be running
    async fn test_get_flagged_patients() {
        dotenv::dotenv().ok();
        let pool = crate::database::connection::establish_connection()
            .await
            .expect("Failed to connect to database");

        let flagged = get_flagged_patients(&pool).await.unwrap();
        assert!(flagged.iter().all(|p| p.flag));
    }
}