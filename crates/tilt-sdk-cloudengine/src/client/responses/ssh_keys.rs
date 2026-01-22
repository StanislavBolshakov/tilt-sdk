use crate::models::common::extensible::LogSchemaWarnings;
use crate::models::ListResponse;
use chrono::Utc;
use serde::Deserialize;

pub type SshKeysResponse = ListResponse<SshKeyWrapper>;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct SshKeyWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub login: String,
    #[serde(default)]
    pub public_keys: Vec<String>,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

impl From<SshKeyWrapper> for crate::models::SshKeys {
    fn from(wrapper: SshKeyWrapper) -> Self {
        wrapper
            ._extra
            .log_unknown_fields("/portal/api/v1/projects/{project}/ssh-keys");

        crate::models::SshKeys {
            id: wrapper.id,
            name: wrapper.name,
            login: wrapper.login,
            public_keys: wrapper.public_keys,
            created_at: wrapper.created_at.parse().unwrap_or_else(|_| Utc::now()),
            updated_at: wrapper
                .updated_at
                .map(|s| s.parse().unwrap_or_else(|_| Utc::now())),
        }
    }
}
