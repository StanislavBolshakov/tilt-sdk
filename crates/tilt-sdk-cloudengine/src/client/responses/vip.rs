use crate::log_schema_drift;
use crate::models::network::VirtualIp;
use crate::models::{FloatingIpInfo, ListResponse, NestedEntity, VirtualIpFixedIp};
use serde::Deserialize;

pub type VipsResponse = ListResponse<VipWrapper>;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct VipWrapper {
    #[serde(default)]
    pub item_id: uuid::Uuid,
    #[serde(default)]
    pub data: VipDataWrapper,
    #[serde(default)]
    #[serde(rename = "created_row_dt")]
    pub created_row_dt: String,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct VipDataWrapper {
    pub state: String,
    pub config: VipConfigWrapper,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct VipConfigWrapper {
    pub name: String,
    #[serde(rename = "mac_address")]
    pub mac_address: String,
    pub fixed_ips: Vec<VipFixedIpWrapper>,
    pub network: Option<NestedEntity<String>>,
    pub subnet: Option<NestedEntity<String>>,
    pub region: Option<NestedEntity<String>>,
    #[serde(rename = "floating_ip")]
    pub floating_ip: Option<VipFloatingIpWrapper>,
    #[serde(rename = "address_mode")]
    pub address_mode: Option<String>,
    #[serde(rename = "l2_enabled")]
    pub l2_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct VipFixedIpWrapper {
    pub id: uuid::Uuid,
    #[serde(rename = "subnet_id")]
    pub subnet_id: uuid::Uuid,
    #[serde(rename = "ip_address")]
    pub ip_address: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct VipFloatingIpWrapper {
    pub id: uuid::Uuid,
    pub bandwidth: u64,
    #[serde(rename = "ip_address")]
    pub ip_address: String,
}

impl From<VipWrapper> for VirtualIp {
    fn from(wrapper: VipWrapper) -> Self {
        log_schema_drift!(
            wrapper,
            "/vpc/api/v1/projects/{project}/virtual-ip-addresses"
        );

        let fixed_ips: Vec<VirtualIpFixedIp> = wrapper
            .data
            .config
            .fixed_ips
            .into_iter()
            .map(|ip| VirtualIpFixedIp {
                id: ip.id,
                subnet_id: ip.subnet_id,
                ip_address: ip.ip_address,
            })
            .collect();

        let network = wrapper.data.config.network.as_ref();
        let subnet = wrapper.data.config.subnet.as_ref();
        let region = wrapper.data.config.region.as_ref();

        VirtualIp {
            id: wrapper.item_id,
            name: wrapper.data.config.name,
            mac_address: wrapper.data.config.mac_address,
            status: wrapper.data.state,
            floating_ip: wrapper.data.config.floating_ip.map(|fip| FloatingIpInfo {
                id: fip.id,
                bandwidth: fip.bandwidth,
                ip_address: fip.ip_address,
            }),
            fixed_ips,
            network_id: network.map(|n| n.id.clone()),
            network_name: network.as_ref().map(|n| n.name.clone()),
            subnet_id: subnet.map(|s| s.id.clone()),
            subnet_name: subnet.as_ref().map(|s| s.name.clone()),
            region_id: region.map(|r| r.id.clone()),
            region_name: region.as_ref().map(|r| r.name.clone()),
            l2_enabled: wrapper.data.config.l2_enabled,
            address_mode: wrapper.data.config.address_mode.unwrap_or_default(),
            created_time: Some(wrapper.created_row_dt),
        }
    }
}
