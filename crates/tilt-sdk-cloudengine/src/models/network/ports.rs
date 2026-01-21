use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NetworkItem {
    Nic(NicPort),
    Instance(InstanceItem),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NicPort {
    pub id: Uuid,
    pub name: String,
    pub ip_address: Option<String>,
    pub mac_address: String,
    pub network_id: Uuid,
    pub network_name: String,
    pub subnet_id: Option<Uuid>,
    pub subnet_name: Option<String>,
    pub security_group_ids: Vec<Uuid>,
    pub security_group_names: Vec<String>,
    pub instance_id: Option<Uuid>,
    pub instance_name: Option<String>,
    pub state: String,
    pub availability_zone: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceItem {
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub flavor_id: Uuid,
    pub flavor_name: String,
    pub image_id: Uuid,
    pub image_name: String,
    pub ip_address: Option<String>,
    pub network_name: Option<String>,
    pub availability_zone: String,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ports {
    pub id: Uuid,
    pub name: String,
    pub mac_address: String,
    pub ip_address: Option<String>,
    pub subnet_id: Option<Uuid>,
    pub subnet_name: Option<String>,
    pub network_id: Uuid,
    pub network_name: String,
    pub security_group_ids: Vec<Uuid>,
    pub security_group_names: Vec<String>,
    pub instance_id: Option<Uuid>,
    pub instance_name: Option<String>,
    pub state: String,
}

impl NetworkItem {
    pub fn id(&self) -> Uuid {
        match self {
            NetworkItem::Nic(nic) => nic.id,
            NetworkItem::Instance(inst) => inst.id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            NetworkItem::Nic(nic) => &nic.name,
            NetworkItem::Instance(inst) => &inst.name,
        }
    }

    pub fn item_type(&self) -> &str {
        match self {
            NetworkItem::Nic(_) => "nic",
            NetworkItem::Instance(_) => "instance",
        }
    }

    pub fn is_nic(&self) -> bool {
        matches!(self, NetworkItem::Nic(_))
    }

    pub fn into_nic(self) -> Option<NicPort> {
        match self {
            NetworkItem::Nic(nic) => Some(nic),
            NetworkItem::Instance(_) => None,
        }
    }

    pub fn instance_id(&self) -> Option<uuid::Uuid> {
        match self {
            NetworkItem::Nic(nic) => nic.instance_id,
            NetworkItem::Instance(inst) => Some(inst.id),
        }
    }

    pub fn ip_address(&self) -> Option<&String> {
        match self {
            NetworkItem::Nic(nic) => nic.ip_address.as_ref(),
            NetworkItem::Instance(inst) => inst.ip_address.as_ref(),
        }
    }
}
