use crate::models::common::extensible::LogSchemaWarnings;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NetworkInterfacesResponse {
    pub interfaces: Vec<NetworkInterfaceWrapper>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct NetworkInterfaceWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub network_id: uuid::Uuid,
    #[serde(default)]
    pub network_name: String,
    #[serde(default)]
    pub mac_address: String,
    #[serde(default)]
    pub ip_addresses: Vec<IpAddress>,
    #[serde(default)]
    pub instance_id: Option<uuid::Uuid>,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

impl From<NetworkInterfaceWrapper> for crate::models::NetworkInterface {
    fn from(wrapper: NetworkInterfaceWrapper) -> Self {
        wrapper._extra.log_unknown_fields("/order-service/api/v1/projects/{project}/compute/instances/{instance_id}/network-interfaces");

        crate::models::NetworkInterface {
            id: wrapper.id,
            network_id: wrapper.network_id,
            network_name: wrapper.network_name,
            mac_address: wrapper.mac_address,
            ip_addresses: wrapper
                .ip_addresses
                .into_iter()
                .map(|ip| crate::models::IpAddress {
                    address: ip.address,
                    subnet_cidr: ip.subnet_cidr,
                    version: ip.version,
                })
                .collect(),
            instance_id: wrapper.instance_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct IpAddress {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub subnet_cidr: Option<String>,
    #[serde(default)]
    pub version: u8,
}
