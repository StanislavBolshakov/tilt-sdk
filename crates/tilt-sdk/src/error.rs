use http::StatusCode;
use serde::Deserialize;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SdkError {
    #[error(transparent)]
    Http(#[from] HttpError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error("request timeout after {timeout}s")]
    Timeout { timeout: u64 },

    #[error("rate limited, retry after {retry_after}s")]
    RateLimited { retry_after: u64 },

    #[error("resource not found: {resource}")]
    NotFound { resource: String },

    #[error("permission denied: {message}")]
    PermissionDenied { message: String },

    #[error("quota exceeded: {resource}. {hint}")]
    QuotaExceeded { resource: String, hint: String },

    #[error("validation failed: {message}")]
    Validation { message: String },

    #[error(transparent)]
    Unexpected(#[from] UnexpectedError),
}

#[derive(Debug, Error)]
pub struct HttpError {
    pub status: StatusCode,
    pub request_id: Option<String>,
    pub body: Option<String>,
    pub hints: Vec<String>,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HTTP {} {}",
            self.status,
            self.status.canonical_reason().unwrap_or("unknown")
        )?;
        if let Some(ref body) = self.body {
            let canonical = self.status.canonical_reason().unwrap_or("");
            if body != canonical {
                write!(f, " {}", body)?;
            }
        }
        if let Some(ref req_id) = self.request_id {
            write!(f, ", request_id={}", req_id)?;
        }
        if !self.hints.is_empty() {
            write!(f, ", hints: {}", self.hints.join(", "))?;
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
#[error("unexpected error: {message}")]
pub struct UnexpectedError {
    pub message: String,
    pub request_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderError {
    pub message: Option<String>,
    pub error: Option<String>,
    pub code: Option<i32>,
    pub request_id: Option<String>,
}

impl ProviderError {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref().or(self.error.as_deref())
    }
}

pub type Result<T> = std::result::Result<T, SdkError>;
