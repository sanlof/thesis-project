use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    // Swedish personal ID pattern: YYYYMMDD-XXXX
    static ref PERSONAL_ID_REGEX: Regex = Regex::new(r"\d{8}-\d{4}").unwrap();
}

/// Sanitizes sensitive data from log messages
/// Redacts Swedish personal IDs (YYYYMMDD-XXXX) to YYYYMMDD-****
pub fn sanitize_personal_id(message: &str) -> String {
    PERSONAL_ID_REGEX.replace_all(message, |caps: &regex::Captures| {
        let full_match = &caps[0];
        let date_part = &full_match[..8];
        format!("{}-****", date_part)
    }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_personal_id() {
        let input = "Patient 19850312-2398 checked in";
        let output = sanitize_personal_id(input);
        assert_eq!(output, "Patient 19850312-**** checked in");
    }

    #[test]
    fn test_sanitize_multiple_ids() {
        let input = "Patients 19850312-2398 and 19900204-1457";
        let output = sanitize_personal_id(input);
        assert_eq!(output, "Patients 19850312-**** and 19900204-****");
    }
}