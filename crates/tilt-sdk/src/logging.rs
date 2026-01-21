use regex::Regex;
use std::collections::HashMap;
use tracing::field;

lazy_static::lazy_static! {
    static ref BEARER_TOKEN_PATTERN: Regex =
        Regex::new(r#"(?i)(Authorization:\s*Bearer\s+)([a-zA-Z0-9_\-\.]+)"#).unwrap();
}

pub fn redact_secrets(input: &str) -> String {
    BEARER_TOKEN_PATTERN
        .replace_all(input, "${1}[REDACTED]")
        .to_string()
}

pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .json()
        .init();
}

pub fn init_tracing_with_level(level: tracing::Level) {
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .json()
        .init();
}

#[derive(Default)]
pub struct KvVisitor(HashMap<String, String>);

impl tracing::field::Visit for KvVisitor {
    fn record_str(&mut self, field: &field::Field, value: &str) {
        self.0.insert(field.name().to_string(), value.to_string());
    }

    fn record_debug(&mut self, field: &field::Field, value: &dyn std::fmt::Debug) {
        self.0
            .insert(field.name().to_string(), format!("{:?}", value));
    }
}
