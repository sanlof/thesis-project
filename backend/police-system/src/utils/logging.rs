use sha2::{Sha256, Digest};

/// Generate a hash of personal_id for logging purposes
/// 
/// Creates a SHA-256 hash of the personal ID and returns the first 16 hex characters.
/// This allows correlation of log entries without exposing actual PII.
/// 
/// # Arguments
/// 
/// * `personal_id` - The Swedish personal ID to hash
/// 
/// # Returns
/// 
/// * `String` - First 16 characters of SHA-256 hash in hexadecimal
/// 
/// # Example
/// 
/// ```
/// log::info!("Processing request for personal_id hash: {}", 
///     hash_for_logging(&personal_id));
/// ```
pub fn hash_for_logging(personal_id: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(personal_id.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)[..16].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_for_logging() {
        let pid = "19850312-2398";
        let hash = hash_for_logging(pid);
        
        // Should return 16 character hex string
        assert_eq!(hash.len(), 16);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
        
        // Same input should produce same hash
        assert_eq!(hash, hash_for_logging(pid));
        
        // Different input should produce different hash
        let different_hash = hash_for_logging("19900101-1234");
        assert_ne!(hash, different_hash);
    }

    #[test]
    fn test_hash_consistency() {
        let pid = "19850312-2398";
        let hash1 = hash_for_logging(pid);
        let hash2 = hash_for_logging(pid);
        
        assert_eq!(hash1, hash2, "Hash should be deterministic");
    }
}