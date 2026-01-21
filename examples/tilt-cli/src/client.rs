use anyhow::{Context, Result};
use std::time::Duration;
use tilt_sdk::Client;
use tilt_sdk::ClientBuilder;

pub fn build_client(
    token: Option<String>,
    project: Option<String>,
    url: Option<String>,
    timeout: Option<Duration>,
) -> Result<Client> {
    let token = token
        .or_else(|| std::env::var("TILT_TOKEN").ok())
        .context("TILT_TOKEN not set")?;

    let project = project
        .or_else(|| std::env::var("TILT_PROJECT").ok())
        .context("TILT_PROJECT not set")?;

    let url = url
        .or_else(|| std::env::var("TILT_API_URL").ok())
        .unwrap_or_else(|| "https://api.t1.cloud".to_string());

    let mut builder = ClientBuilder::new()
        .base_url(&url)
        .token(&token)
        .project(&project);

    if let Some(timeout) = timeout {
        builder = builder.timeout(timeout);
    }

    builder.build().context("Failed to build client")
}
