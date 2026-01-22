use crate::log_schema_drift;
use crate::models::{FloatingIp, ListResponse};
use serde::Deserialize;

pub type FloatingIpsResponse = ListResponse<FipWrapper>;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct FipWrapper {
    #[serde(default)]
    pub item_id: uuid::Uuid,
    #[serde(default)]
    pub data: FipDataWrapper,
    #[serde(default)]
    pub parent: String,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FipDataWrapper {
    pub state: String,
    pub config: FipConfigWrapper,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FipConfigWrapper {
    #[serde(rename = "floating_ip_address")]
    pub floating_ip_address: Option<String>,
    pub bandwidth: Option<String>,
}

impl From<FipWrapper> for FloatingIp {
    fn from(wrapper: FipWrapper) -> Self {
        log_schema_drift!(
            wrapper,
            "/vpc/api/v1/projects/{project}/public-ip-addresses"
        );

        FloatingIp {
            id: wrapper.item_id,
            floating_ip_address: wrapper.data.config.floating_ip_address.unwrap_or_default(),
            bandwidth: wrapper
                .data
                .config
                .bandwidth
                .unwrap_or_default()
                .parse()
                .unwrap_or(0),
            status: wrapper.data.state,
            parent_item_id: None,
            parent_item_type: None,
        }
    }
}
