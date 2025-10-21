use sqlx::PgPool;
use crate::models::{Suspect, CreateSuspect, UpdateSuspect};

/// Retrieves all suspects from the database
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// 
/// # Returns
/// 
/// * `Result<Vec<Suspect>, sqlx::Error>` - List of all suspects
pub async fn get_all_suspects(pool: &PgPool) -> Result<Vec<Suspect>, sqlx::Error> {
    let suspects = sqlx::query_as!(
        Suspect,
        "SELECT id, full_name, personal_id, flag FROM suspects ORDER BY id"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(suspects)
}

/// Retrieves a suspect by their database ID
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// * `id` - Suspect's database ID
/// 
/// # Returns
/// 
/// * `Result<Option<Suspect>, sqlx::Error>` - Suspect if found, None otherwise
pub async fn get_suspect_by_id(pool: &PgPool, id: i32) -> Result<Option<Suspect>, sqlx::Error> {
    let suspect = sqlx::query_as!(
        Suspect,
        "SELECT id, full_name, personal_id, flag FROM suspects WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(suspect)
}

/// Retrieves a suspect by their Swedish personal ID
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// * `personal_id` - Swedish personal ID (YYYYMMDD-XXXX)
/// 
/// # Returns
/// 
/// * `Result<Option<Suspect>, sqlx::Error>` - Suspect if found, None otherwise
pub async fn get_suspect_by_personal_id(
    pool: &PgPool,
    personal_id: &str,
) -> Result<Option<Suspect>, sqlx::Error> {
    let suspect = sqlx::query_as!(
        Suspect,
        "SELECT id, full_name, personal_id, flag FROM suspects WHERE personal_id = $1",
        personal_id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(suspect)
}

/// Creates a new suspect in the database
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// * `suspect` - Suspect data to create
/// 
/// # Returns
/// 
/// * `Result<Suspect, sqlx::Error>` - Created suspect with generated ID
pub async fn create_suspect(
    pool: &PgPool,
    suspect: CreateSuspect,
) -> Result<Suspect, sqlx::Error> {
    let created_suspect = sqlx::query_as!(
        Suspect,
        "INSERT INTO suspects (full_name, personal_id, flag) 
         VALUES ($1, $2, $3) 
         RETURNING id, full_name, personal_id, flag",
        suspect.full_name,
        suspect.personal_id,
        suspect.flag
    )
    .fetch_one(pool)
    .await?;
    
    Ok(created_suspect)
}

/// Updates an existing suspect by ID
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// * `id` - Suspect's database ID
/// * `suspect` - Updated suspect data
/// 
/// # Returns
/// 
/// * `Result<Option<Suspect>, sqlx::Error>` - Updated suspect if found, None otherwise
pub async fn update_suspect(
    pool: &PgPool,
    id: i32,
    suspect: UpdateSuspect,
) -> Result<Option<Suspect>, sqlx::Error> {
    // Check if suspect exists first
    let existing = get_suspect_by_id(pool, id).await?;
    if existing.is_none() {
        return Ok(None);
    }
    
    // Update fields if provided, otherwise keep existing values
    let updated_suspect = sqlx::query_as!(
        Suspect,
        "UPDATE suspects 
         SET full_name = COALESCE($1, full_name),
             personal_id = COALESCE($2, personal_id),
             flag = COALESCE($3, flag)
         WHERE id = $4
         RETURNING id, full_name, personal_id, flag",
        suspect.full_name,
        suspect.personal_id,
        suspect.flag,
        id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(updated_suspect)
}

/// Deletes a suspect by ID
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// * `id` - Suspect's database ID
/// 
/// # Returns
/// 
/// * `Result<bool, sqlx::Error>` - true if deleted, false if not found
pub async fn delete_suspect(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM suspects WHERE id = $1",
        id
    )
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected() > 0)
}

/// Updates the flag status of a suspect by personal ID
/// This function is particularly important for cross-system synchronization
/// 
/// # Arguments
/// 
/// * `pool` - Database connection pool
/// * `personal_id` - Swedish personal ID (YYYYMMDD-XXXX)
/// * `flag` - New flag status
/// 
/// # Returns
/// 
/// * `Result<Option<Suspect>, sqlx::Error>` - Updated suspect if found, None otherwise
pub async fn update_flag(
    pool: &PgPool,
    personal_id: &str,
    flag: bool,
) -> Result<Option<Suspect>, sqlx::Error> {
    let updated_suspect = sqlx::query_as!(
        Suspect,
        "UPDATE suspects 
         SET flag = $1 
         WHERE personal_id = $2
         RETURNING id, full_name, personal_id, flag",
        flag,
        personal_id
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(updated_suspect)
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
        let new_suspect = CreateSuspect {
            full_name: "Test Person".to_string(),
            personal_id: "19990101-1234".to_string(),
            flag: false,
        };
        
        let created = create_suspect(&pool, new_suspect).await.unwrap();
        assert_eq!(created.full_name, "Test Person");

        // Test get by id
        let retrieved = get_suspect_by_id(&pool, created.id).await.unwrap();
        assert!(retrieved.is_some());

        // Test delete
        let deleted = delete_suspect(&pool, created.id).await.unwrap();
        assert!(deleted);
    }
}