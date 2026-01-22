use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::models::common::LogSchemaWarnings;

pub type NetworksResponse = Vec<NetworkWrapper>;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct NetworkWrapper {
    pub id: uuid::Uuid,
    pub name: String,
    pub status: Option<String>,
    pub description: Option<String>,
    pub create_time: Option<String>,
    #[serde(default)]
    pub shared_from: Option<SharedFromObject>,
    #[serde(flatten)]
    pub _extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SharedFromObject {
    id: String,
}

impl From<NetworkWrapper> for crate::models::Networks {
    fn from(wrapper: NetworkWrapper) -> Self {
        wrapper
            ._extra
            .log_unknown_fields("/vpc/api/v1/projects/{project}/networks");

        crate::models::Networks {
            id: wrapper.id,
            name: wrapper.name,
            status: wrapper.status.unwrap_or_default(),
            description: wrapper.description,
            create_time: wrapper.create_time,
            shared_from: wrapper.shared_from.map(|s| s.id),
        }
    }
}
