use crate::models::{FloatingIp, ListResponse};
use serde::Deserialize;

pub type FloatingIpsResponse = ListResponse<FipWrapper>;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FipWrapper {
    #[serde(default)]
    pub item_id: uuid::Uuid,
    #[serde(default)]
    pub data: FipDataWrapper,
    #[serde(default)]
    pub parent: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FipDataWrapper {
    #[serde(default)]
    pub config: FipConfigWrapper,
    #[serde(default)]
    pub state: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FipConfigWrapper {
    #[serde(default)]
    pub region: Option<FipRegionWrapper>,
    #[serde(default)]
    #[serde(rename = "floating_ip_address")]
    pub floating_ip_address: String,
    #[serde(default)]
    pub bandwidth: u64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FipRegionWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
}

impl From<FipWrapper> for FloatingIp {
    fn from(wrapper: FipWrapper) -> Self {
        FloatingIp {
            id: wrapper.item_id,
            floating_ip_address: wrapper.data.config.floating_ip_address,
            bandwidth: wrapper.data.config.bandwidth,
            status: wrapper.data.state,
            parent_item_id: None,
            parent_item_type: None,
        }
    }
}
