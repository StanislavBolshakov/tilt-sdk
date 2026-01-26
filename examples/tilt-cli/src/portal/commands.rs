use cloudengine::client::portal::{CreateSshKeyRequest, SshKeyDetails};
use cloudengine::{ComputeError, SshKeys};
use tilt_sdk_cloudengine as cloudengine;

use crate::output::SshKeyRow;
use crate::output::format_table;

pub async fn list_ssh_keys(
    client: &cloudengine::ComputeClient<'_>,
    limit: Option<u32>,
    page: Option<u32>,
) -> Result<Vec<SshKeys>, ComputeError> {
    client.list_ssh_keys(limit, page).await
}

pub async fn create_ssh_key(
    client: &cloudengine::ComputeClient<'_>,
    name: String,
    login: String,
    public_keys: Vec<String>,
) -> Result<SshKeys, ComputeError> {
    let request = CreateSshKeyRequest {
        ssh_key: SshKeyDetails {
            public_keys,
            login,
            name,
        },
    };
    client.create_ssh_key(request).await
}

pub async fn delete_ssh_key(
    client: &cloudengine::ComputeClient<'_>,
    ssh_key_id: &str,
) -> Result<serde_json::Value, ComputeError> {
    let id = ssh_key_id.parse().map_err(|_| {
        ComputeError::validation(
            cloudengine::Service::PortalApi,
            None,
            format!("Invalid SSH key ID: {}", ssh_key_id),
        )
    })?;
    client.delete_ssh_key(id).await
}

pub fn format_ssh_key_rows(keys: &[SshKeys]) -> String {
    let rows: Vec<SshKeyRow> = keys
        .iter()
        .map(|k| SshKeyRow {
            id: k.id.to_string(),
            name: k.name.clone(),
            login: k.login.clone(),
            created: k.created_at.format("%Y-%m-%d").to_string(),
        })
        .collect();
    format_table(&rows)
}
