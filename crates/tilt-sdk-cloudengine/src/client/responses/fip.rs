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
    pub parent: Option<String>,
    #[serde(default)]
    pub action_id: Option<String>,
    #[serde(default)]
    pub graph_id: Option<String>,
    #[serde(default)]
    pub graph_name: Option<String>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub src_order_id: Option<String>,
    #[serde(default)]
    pub external_provider_id: Option<String>,
    #[serde(default)]
    pub created_row_dt: Option<String>,
    #[serde(default)]
    pub update_dt: Option<String>,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct FipParentItemWrapper {
    pub item_id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct FipDataWrapper {
    pub state: Option<String>,
    pub config: FipConfigWrapper,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub provider: Option<String>,
    #[serde(default)]
    pub src_order_id: Option<String>,
    #[serde(default)]
    pub parent_item: Option<FipParentItemWrapper>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct FipConfigWrapper {
    #[serde(rename = "floating_ip_address")]
    pub floating_ip_address: Option<String>,
    pub bandwidth: Option<u64>,
    pub region: Option<FipRegionWrapper>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FipRegionWrapper {
    pub id: Option<String>,
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
            bandwidth: wrapper.data.config.bandwidth.unwrap_or(0),
            status: wrapper.data.state.unwrap_or_else(|| "unknown".to_string()),
            parent_item_id: wrapper
                .data
                .parent_item
                .as_ref()
                .and_then(|p| p.item_id.as_ref().and_then(|id| id.parse().ok())),
            parent_item_type: wrapper
                .data
                .parent_item
                .as_ref()
                .and_then(|p| p.r#type.clone()),
            created_time: wrapper.created_row_dt,
            region_id: wrapper
                .data
                .config
                .region
                .as_ref()
                .and_then(|r| r.id.clone()),
        }
    }
}
