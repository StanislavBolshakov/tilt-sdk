use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subnets {
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub cidr: String,
    pub description: Option<String>,
    pub gateway_ip: Option<String>,
    pub enable_dhcp: bool,
    pub network_id: Uuid,
    pub network_name: String,
    pub region_id: String,
    pub region_name: String,
    pub shared_from: Option<String>,
}
