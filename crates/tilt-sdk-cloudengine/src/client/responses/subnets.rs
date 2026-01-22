use crate::models::common::extensible::LogSchemaWarnings;
use crate::models::NestedEntity;
use serde::Deserialize;

pub type SubnetsResponse = Vec<SubnetWrapper>;

#[derive(Debug, Clone, Deserialize)]
pub struct SharedFromObject {
    id: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct SubnetWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub cidr: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub gateway_ip: Option<String>,
    #[serde(default)]
    pub enable_dhcp: bool,
    #[serde(default)]
    pub network: Option<NestedEntity<uuid::Uuid>>,
    #[serde(default)]
    pub region: Option<NestedEntity<String>>,
    #[serde(default)]
    pub shared_from: Option<SharedFromObject>,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

pub type NestedNetwork = NestedEntity<uuid::Uuid>;
pub type NestedRegion = NestedEntity<String>;

impl From<SubnetWrapper> for crate::models::Subnets {
    fn from(wrapper: SubnetWrapper) -> Self {
        wrapper
            ._extra
            .log_unknown_fields("/vpc/api/v1/projects/{project}/subnets");

        let network = wrapper.network;
        let region = wrapper.region;

        crate::models::Subnets {
            id: wrapper.id,
            name: wrapper.name,
            status: wrapper.status.unwrap_or_default(),
            cidr: wrapper.cidr,
            description: wrapper.description,
            gateway_ip: wrapper.gateway_ip,
            enable_dhcp: wrapper.enable_dhcp,
            network_id: network.as_ref().map(|n| n.id).unwrap_or_default(),
            network_name: network.as_ref().map(|n| n.name.clone()).unwrap_or_default(),
            region_id: region.as_ref().map(|r| r.id.clone()).unwrap_or_default(),
            region_name: region.as_ref().map(|r| r.name.clone()).unwrap_or_default(),
            shared_from: wrapper.shared_from.map(|s| s.id),
        }
    }
}
