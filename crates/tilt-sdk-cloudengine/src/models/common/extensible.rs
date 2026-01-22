use serde_json::Value;
use std::collections::HashMap;

const SCHEMA_DRIFT_REPORT_URL: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/issues");

pub trait LogSchemaWarnings {
    fn log_unknown_fields(&self, path: &str);
}

impl LogSchemaWarnings for HashMap<String, Value> {
    fn log_unknown_fields(&self, path: &str) {
        if !self.is_empty() {
            tracing::warn!(
                path = path,
                unknown_fields = ?self.keys().collect::<Vec<_>>(),
                "API schema drift. Please report {}",
                SCHEMA_DRIFT_REPORT_URL
            );
        }
    }
}

#[macro_export]
macro_rules! log_schema_drift {
    ($wrapper:expr, $path:expr) => {
        $crate::models::common::LogSchemaWarnings::log_unknown_fields(&$wrapper._extra, $path);
    };
}
