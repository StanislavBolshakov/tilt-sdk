use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DhcpDns {
    pub method: String,
    #[serde(default)]
    pub reverse_resolution: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DhcpSettings {
    pub dns: DhcpDns,
    pub domain_name: Option<String>,
    pub ntp_servers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Networks {
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub description: Option<String>,
    pub create_time: Option<String>,
    pub shared_from: Option<String>,
    pub dhcp_settings: Option<DhcpSettings>,
}

pub type Network = Networks;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub id: Uuid,
    pub network_id: Uuid,
    pub network_name: String,
    pub mac_address: String,
    pub ip_addresses: Vec<IpAddress>,
    pub instance_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpAddress {
    pub address: String,
    pub subnet_cidr: Option<String>,
    pub version: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityGroups {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub create_time: String,
    pub update_time: Option<String>,
}

pub type SecurityGroup = SecurityGroups;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityGroupRule {
    pub id: Uuid,
    pub security_group_id: Uuid,
    pub direction: String,
    pub protocol: String,
    pub ethertype: String,
    pub port_range_min: Option<u32>,
    pub port_range_max: Option<u32>,
    pub remote_ip_prefix: Option<String>,
    pub remote_group_id: Option<Uuid>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualIp {
    pub id: Uuid,
    pub name: String,
    pub mac_address: String,
    pub status: String,
    pub floating_ip: Option<FloatingIpInfo>,
    pub fixed_ips: Vec<VirtualIpFixedIp>,
    pub network_id: Option<String>,
    pub network_name: Option<String>,
    pub subnet_id: Option<String>,
    pub subnet_name: Option<String>,
    pub region_id: Option<String>,
    pub region_name: Option<String>,
    pub l2_enabled: bool,
    pub address_mode: String,
    pub created_time: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirtualIpFixedIp {
    pub id: Uuid,
    pub subnet_id: Uuid,
    pub ip_address: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatingIpInfo {
    pub id: Uuid,
    pub bandwidth: u64,
    pub ip_address: String,
}

pub type VirtualIps = VirtualIp;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatingIp {
    pub id: Uuid,
    pub floating_ip_address: String,
    pub bandwidth: u64,
    pub status: String,
    pub parent_item_id: Option<Uuid>,
    pub parent_item_type: Option<String>,
}

pub type FloatingIps = FloatingIp;
