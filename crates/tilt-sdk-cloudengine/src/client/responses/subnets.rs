use crate::models::common::extensible::LogSchemaWarnings;
use crate::models::network::networks::{DhcpDns, DhcpSettings};
use crate::models::{NestedEntity, Subnets};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

pub type SubnetsResponse = Vec<SubnetWrapper>;

#[derive(Debug, Clone, Deserialize)]
pub struct SharedFromObject {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SharedToObject {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DhcpDnsWrapper {
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub reverse_resolution: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DhcpSettingsWrapper {
    #[serde(default)]
    pub dns: DhcpDnsWrapper,
    #[serde(default)]
    pub domain_name: Option<String>,
    #[serde(default)]
    pub ntp_servers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct NetworkData {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub dhcp_settings: Option<DhcpSettingsWrapper>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct SubnetWrapper {
    pub id: uuid::Uuid,
    pub name: String,
    pub status: Option<String>,
    pub cidr: String,
    pub description: Option<String>,
    pub gateway_ip: Option<String>,
    pub enable_dhcp: bool,
    pub network: Option<NetworkData>,
    pub region: Option<NestedEntity<String>>,
    pub shared_from: Option<SharedFromObject>,
    pub shared_to: Vec<SharedToObject>,
    #[serde(flatten)]
    pub _extra: HashMap<String, Value>,
}

impl From<SubnetWrapper> for Subnets {
    fn from(wrapper: SubnetWrapper) -> Self {
        wrapper
            ._extra
            .log_unknown_fields("/vpc/api/v1/projects/{project}/subnets");

        let SubnetWrapper {
            id,
            name,
            status,
            cidr,
            description,
            gateway_ip,
            enable_dhcp,
            network,
            region,
            shared_from,
            shared_to,
            ..
        } = wrapper;

        let dhcp_settings = network
            .as_ref()
            .and_then(|n| n.dhcp_settings.as_ref())
            .map(|d| DhcpSettings {
                dns: DhcpDns {
                    method: d.dns.method.clone(),
                    reverse_resolution: d.dns.reverse_resolution,
                },
                domain_name: d.domain_name.clone(),
                ntp_servers: d.ntp_servers.clone(),
            });

        Subnets {
            id,
            name,
            status: status.unwrap_or_default(),
            cidr,
            description,
            gateway_ip,
            enable_dhcp,
            dhcp_settings,
            network_id: network.as_ref().map(|n| n.id).unwrap_or_default(),
            network_name: network.map(|n| n.name).unwrap_or_default(),
            region_id: region.as_ref().map(|r| r.id.clone()).unwrap_or_default(),
            region_name: region.map(|r| r.name).unwrap_or_default(),
            shared_from: shared_from.map(|s| s.id),
            shared_to: shared_to.into_iter().map(|s| s.id).collect(),
        }
    }
}
