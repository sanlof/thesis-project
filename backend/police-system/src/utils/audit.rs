use serde::Serialize;
use chrono::{DateTime, Utc};
use actix_web::dev::ServiceRequest;
use std::net::IpAddr;

/// Audit event types for different operations
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    FlagUpdate,
    SuspectAccess,
    SuspectCreate,
    SuspectUpdate,
    SuspectDelete,
    SharedApiAccess,
}

/// Audit action types
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Action {
    Read,
    Create,
    Update,
    Delete,
}

/// Audit result status
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditResult {
    Success,
    Failure,
}

/// Structured audit log entry
#[derive(Debug, Serialize)]
pub struct AuditLog {
    timestamp: DateTime<Utc>,
    event_type: EventType,
    actor: String,
    action: Action,
    resource: String,
    result: AuditResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl AuditLog {
    /// Create a new audit log entry
    pub fn new(
        event_type: EventType,
        actor: String,
        action: Action,
        resource: String,
        result: AuditResult,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            event_type,
            actor,
            action,
            resource,
            result,
            ip_address: None,
            details: None,
        }
    }

    /// Add IP address to audit log
    pub fn with_ip(mut self, ip: Option<IpAddr>) -> Self {
        self.ip_address = ip.map(|addr| addr.to_string());
        self
    }

    /// Add additional details to audit log
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    /// Write audit log entry to the audit log target
    pub fn write(self) {
        match serde_json::to_string(&self) {
            Ok(json) => log::info!(target: "audit", "{}", json),
            Err(e) => log::error!("Failed to serialize audit log: {}", e),
        }
    }
}

/// Extract actor from API key in request
/// Returns a hash of the API key for privacy
pub fn extract_actor_from_request(req: &ServiceRequest) -> String {
    req.headers()
        .get("X-API-Key")
        .and_then(|h| h.to_str().ok())
        .map(|key| {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(key.as_bytes());
            format!("api_key:{:x}", hasher.finalize())[..24].to_string()
        })
        .unwrap_or_else(|| "internal".to_string())
}

/// Extract IP address from request
pub fn extract_ip_from_request(req: &ServiceRequest) -> Option<IpAddr> {
    req.peer_addr().map(|addr| addr.ip())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log_serialization() {
        let log = AuditLog::new(
            EventType::FlagUpdate,
            "test_actor".to_string(),
            Action::Update,
            "suspect:test123".to_string(),
            AuditResult::Success,
        );

        let json = serde_json::to_string(&log).unwrap();
        assert!(json.contains("FLAG_UPDATE"));
        assert!(json.contains("UPDATE"));
        assert!(json.contains("SUCCESS"));
    }

    #[test]
    fn test_audit_log_with_ip() {
        let log = AuditLog::new(
            EventType::SuspectAccess,
            "actor".to_string(),
            Action::Read,
            "suspect:hash".to_string(),
            AuditResult::Success,
        )
        .with_ip(Some("127.0.0.1".parse().unwrap()));

        let json = serde_json::to_string(&log).unwrap();
        assert!(json.contains("127.0.0.1"));
    }

    #[test]
    fn test_audit_log_with_details() {
        let log = AuditLog::new(
            EventType::FlagUpdate,
            "actor".to_string(),
            Action::Update,
            "suspect:hash".to_string(),
            AuditResult::Failure,
        )
        .with_details("Invalid flag value".to_string());

        let json = serde_json::to_string(&log).unwrap();
        assert!(json.contains("Invalid flag value"));
    }
}