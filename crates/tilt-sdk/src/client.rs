use crate::{error::Result, http::ReqwestClient};
use std::time::Duration;
use url::Url;

#[derive(Clone)]
pub struct Client {
    http: ReqwestClient,
    project: String,
}

impl Client {
    pub fn http(&self) -> &ReqwestClient {
        &self.http
    }

    pub fn project(&self) -> &str {
        &self.project
    }

    pub fn base_url(&self) -> &Url {
        self.http.base_url()
    }

    pub fn token(&self) -> Option<&str> {
        self.http.token()
    }
}

pub struct ClientBuilder {
    base_url: Option<Url>,
    token: Option<String>,
    project: Option<String>,
    timeout: Option<Duration>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            base_url: None,
            token: None,
            project: None,
            timeout: None,
        }
    }

    pub fn base_url(mut self, base_url: &str) -> Self {
        self.base_url = Some(Url::parse(base_url).expect("Invalid base URL"));
        self
    }

    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn project(mut self, project: &str) -> Self {
        self.project = Some(project.to_string());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Result<Client> {
        let base_url = self.base_url.unwrap_or_else(|| {
            Url::parse("https://api.t1.cloud").expect("Default base URL should be valid")
        });

        let mut http = ReqwestClient::new(base_url, self.token);

        if let Some(timeout) = self.timeout {
            http = http.with_timeout(timeout);
        }

        let project = self
            .project
            .ok_or_else(|| crate::error::SdkError::Validation {
                message: "project is required".to_string(),
            })?;

        Ok(Client { http, project })
    }

    pub fn build_from_env(self) -> Result<Client> {
        let base_url = std::env::var("TILT_API_URL")
            .unwrap_or_else(|_| "https://api.t1.cloud/order-service/".to_string());
        let token =
            std::env::var("TILT_TOKEN").map_err(|_| crate::error::SdkError::PermissionDenied {
                message: "TILT_TOKEN environment variable not set".to_string(),
            })?;
        let project =
            std::env::var("TILT_PROJECT").map_err(|_| crate::error::SdkError::Validation {
                message: "TILT_PROJECT environment variable not set".to_string(),
            })?;

        let mut client = self.base_url(&base_url).token(&token).project(&project);

        if let Ok(timeout_str) = std::env::var("TILT_TIMEOUT")
            && let Ok(timeout) = timeout_str.parse()
        {
            client = client.timeout(Duration::from_secs(timeout));
        }

        client.build()
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
