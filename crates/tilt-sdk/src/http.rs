use crate::error::{HttpError, ProviderError, SdkError};
use http::StatusCode;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use std::time::Duration;
use tracing::{Instrument, debug, info_span, warn};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

fn configure_proxy(builder: reqwest::ClientBuilder) -> reqwest::ClientBuilder {
    let mut builder = builder;

    if let Ok(proxy_url) = std::env::var("HTTPS_PROXY")
        && let Ok(proxy) = reqwest::Proxy::https(&proxy_url)
    {
        builder = builder.proxy(proxy);
        warn!(target: "tilt-core", "HTTPS proxy configured via HTTPS_PROXY: {}", proxy_url);
    }

    if let Ok(proxy_url) = std::env::var("HTTP_PROXY")
        && let Ok(proxy) = reqwest::Proxy::http(&proxy_url)
    {
        builder = builder.proxy(proxy);
        warn!(target: "tilt-core", "HTTP proxy configured via HTTP_PROXY: {}", proxy_url);
    }

    builder
}

#[derive(Clone)]
pub struct ReqwestClient {
    base_url: url::Url,
    token: Option<String>,
    inner: reqwest::Client,
    timeout: Duration,
}

impl ReqwestClient {
    pub fn new(base_url: url::Url, _token: Option<String>) -> Self {
        let mut builder = reqwest::Client::builder();

        let user_agent = std::env::var("TILT_USER_AGENT")
            .unwrap_or_else(|_| format!("tilt-sdk/{}", env!("CARGO_PKG_VERSION")));
        builder = builder.user_agent(&user_agent);

        builder = configure_proxy(builder);

        let inner = builder.build().expect("Failed to build reqwest client");

        Self {
            base_url,
            token: _token,
            inner,
            timeout: DEFAULT_TIMEOUT,
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn base_url(&self) -> &url::Url {
        &self.base_url
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    fn build_request(
        &self,
        method: http::Method,
        path: &str,
        query: Option<&[(&str, &str)]>,
    ) -> RequestBuilder {
        let url = {
            let mut url = self.base_url.clone();
            url.set_path(path);
            if let Some(query) = query {
                url.query_pairs_mut().extend_pairs(query.iter().copied());
            }
            url
        };

        let builder = self.inner.request(method.clone(), url);

        if let Some(token) = &self.token {
            builder.bearer_auth(token)
        } else {
            builder
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, SdkError> {
        self.send_request::<(), T>(http::Method::GET, path, None, None)
            .await
    }

    pub async fn get_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T, SdkError> {
        self.send_request::<(), T>(http::Method::GET, path, None, Some(query))
            .await
    }

    pub async fn post<B: serde::Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, SdkError> {
        self.send_request::<B, T>(http::Method::POST, path, Some(body), None)
            .await
    }

    pub async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T, SdkError> {
        self.send_request::<(), T>(http::Method::POST, path, None, None)
            .await
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, SdkError> {
        self.send_request::<(), T>(http::Method::DELETE, path, None, None)
            .await
    }

    async fn send_request<B: serde::Serialize, T: DeserializeOwned>(
        &self,
        method: http::Method,
        path: &str,
        body: Option<&B>,
        query: Option<&[(&str, &str)]>,
    ) -> Result<T, SdkError> {
        let span = info_span!("http_request", method = %method, path = path);
        async move {
            let builder = self.build_request(method.clone(), path, query);

            let builder = if let Some(body) = body {
                builder.json(body)
            } else {
                builder
            };

            match builder.send().await {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        if status == StatusCode::NO_CONTENT {
                            let val: std::result::Result<T, serde_json::Error> =
                                serde_json::from_value(serde_json::Value::Null);
                            return val.map_err(SdkError::Json);
                        }
                        let text = response.text().await.map_err(SdkError::from)?;
                        debug!("Server response ({} bytes): {}", text.len(), text);
                        let value: T = serde_json::from_str(&text).map_err(SdkError::Json)?;
                        return Ok(value);
                    }

                    let request_id = response
                        .headers()
                        .get("x-request-id")
                        .and_then(|h| h.to_str().ok())
                        .map(String::from);

                    let body = response.text().await.ok();

                    debug!("Server error response: {:?}", body);
                    self.handle_error::<T>(status, &body, request_id)
                }
                Err(e) => Err(SdkError::from(e)),
            }
        }
        .instrument(span)
        .await
    }

    fn handle_error<T>(
        &self,
        status: StatusCode,
        body: &Option<String>,
        request_id: Option<String>,
    ) -> Result<T, SdkError> {
        let hints = Self::extract_hints(status, body);

        let error = HttpError {
            status,
            request_id,
            body: body.clone(),
            hints,
        };

        match status {
            StatusCode::NOT_FOUND => Err(SdkError::NotFound {
                resource: "unknown".to_string(),
            }),
            StatusCode::FORBIDDEN => Err(SdkError::PermissionDenied {
                message: body
                    .as_ref()
                    .unwrap_or(&"Access denied".to_string())
                    .clone(),
            }),
            StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = body
                    .as_ref()
                    .and_then(|b| serde_json::from_str::<ProviderError>(b).ok())
                    .and_then(|e| e.message)
                    .and_then(|m| m.parse().ok())
                    .unwrap_or(60);
                Err(SdkError::RateLimited { retry_after })
            }
            StatusCode::UNPROCESSABLE_ENTITY => Err(SdkError::Validation {
                message: body
                    .as_ref()
                    .and_then(|b| serde_json::from_str::<ProviderError>(b).ok())
                    .and_then(|e| e.message)
                    .unwrap_or_else(|| "Validation failed".to_string())
                    .to_string(),
            }),
            _ => Err(SdkError::Http(error)),
        }
    }

    fn extract_hints(status: StatusCode, body: &Option<String>) -> Vec<String> {
        let mut hints = Vec::new();

        if let Some(body) = body
            && let Ok(provider_err) = serde_json::from_str::<ProviderError>(body)
            && let Some(msg) = &provider_err.message
        {
            let lower = msg.to_lowercase();
            if lower.contains("quota") {
                hints.push("Check your resource quotas".to_string());
                hints.push("Consider requesting a quota increase".to_string());
            }
            if lower.contains("limit") {
                hints.push("You may have hit a rate limit".to_string());
            }
        }

        match status {
            StatusCode::BAD_REQUEST => {
                hints.push("Check your request parameters".to_string());
            }
            StatusCode::UNAUTHORIZED => {
                hints.push("Verify your authentication token".to_string());
            }
            _ => {}
        }

        hints
    }
}

impl From<reqwest::Error> for SdkError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            return SdkError::Timeout { timeout: 30 };
        }
        if e.is_status()
            && let Some(status) = e.status()
        {
            return SdkError::Http(HttpError {
                status,
                request_id: None,
                body: None,
                hints: Vec::new(),
            });
        }
        SdkError::Http(HttpError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            request_id: None,
            body: None,
            hints: vec!["An unexpected HTTP error occurred".to_string()],
        })
    }
}
