//! Tests for logging module

use tilt_sdk::redact_secrets;

#[test]
fn test_redact_authorization_header() {
    let input = "Authorization: Bearer eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0";
    let redacted = redact_secrets(input);
    assert!(redacted.contains("[REDACTED]"));
    assert!(!redacted.contains("eyJhbGciOiJIUzI1NiJ9"));
}

#[test]
fn test_redact_bearer_token_case_insensitive() {
    let input = "authorization: bearer mytoken123";
    let redacted = redact_secrets(input);
    assert!(redacted.contains("[REDACTED]"));
    assert!(!redacted.contains("mytoken123"));
}
