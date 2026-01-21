use crate::models::network::VirtualIp;
use crate::models::{FloatingIpInfo, ListResponse, NestedEntity, VirtualIpFixedIp};
use serde::Deserialize;

pub type VipsResponse = ListResponse<VipWrapper>;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct VipWrapper {
    #[serde(default)]
    pub item_id: uuid::Uuid,
    #[serde(default)]
    pub data: VipDataWrapper,
    #[serde(default)]
    #[serde(rename = "created_row_dt")]
    pub created_row_dt: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct VipDataWrapper {
    #[serde(default)]
    pub config: VipConfigWrapper,
    #[serde(default)]
    pub state: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct VipConfigWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub mac_address: String,
    #[serde(default)]
    #[serde(rename = "l2_enabled")]
    pub l2_enabled: bool,
    #[serde(default)]
    #[serde(rename = "floating_ip")]
    pub floating_ip: Option<FloatingIpInfoWrapper>,
    #[serde(default)]
    #[serde(rename = "fixed_ips")]
    pub fixed_ips: Vec<VipFixedIpWrapper>,
    #[serde(default)]
    pub network: Option<NestedEntity>,
    #[serde(default)]
    pub subnet: Option<NestedEntity>,
    #[serde(default)]
    #[serde(rename = "address_mode")]
    pub address_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct VipFixedIpWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    #[serde(rename = "subnet_id")]
    pub subnet_id: uuid::Uuid,
    #[serde(default)]
    #[serde(rename = "ip_address")]
    pub ip_address: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FloatingIpInfoWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub bandwidth: u64,
    #[serde(default)]
    #[serde(rename = "ip_address")]
    pub ip_address: String,
}

impl From<VipWrapper> for VirtualIp {
    fn from(wrapper: VipWrapper) -> Self {
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
            address_mode: wrapper.data.config.address_mode,
        }
    }
}
