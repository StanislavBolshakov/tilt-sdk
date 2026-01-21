use crate::models::NestedEntity;
use serde::Deserialize;
use std::collections::HashMap;

fn parse_uuid_or_warn(s: &str, field: &str) -> uuid::Uuid {
    match s.parse() {
        Ok(uuid) => uuid,
        Err(e) => {
            tracing::warn!(target: "tilt-sdk-cloudengine", "Invalid UUID for {}: {} ({})", field, s, e);
            uuid::Uuid::nil()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PortsResponse {
    pub meta: PortsMeta,
    pub list: Vec<PortWrapper>,
}

#[derive(Debug, Deserialize)]
pub struct PortsMeta {
    pub total_count: usize,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PortWrapper {
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub data: PortData,
    #[serde(default, rename = "state")]
    pub state: String,
    #[serde(default, rename = "type")]
    pub type_field: String,
    #[serde(default, rename = "created_row_dt")]
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PortData {
    #[serde(default)]
    pub config: PortConfig,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub parent_item: Option<ParentItem>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PortConfig {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub mac_address: String,
    #[serde(default)]
    pub fixed_ips: Vec<FixedIp>,
    #[serde(default)]
    pub security_groups: Vec<SecurityGroup>,
    #[serde(default, rename = "network")]
    pub network: Option<NestedEntity<uuid::Uuid>>,
    #[serde(default, rename = "subnet")]
    pub subnet: Option<NestedEntity<uuid::Uuid>>,
    #[serde(default)]
    pub addresses: HashMap<String, Vec<NestedAddress>>,
    #[serde(default)]
    pub flavor: NestedEntity<uuid::Uuid>,
    #[serde(default)]
    pub source_image: NestedEntity<uuid::Uuid>,
    #[serde(default)]
    pub availability_zone: NestedEntity<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FixedIp {
    #[serde(default)]
    pub ip_address: Option<String>,
    #[serde(default)]
    pub id: Option<uuid::Uuid>,
    #[serde(default)]
    pub subnet_id: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SecurityGroup {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
}

pub type NestedNetwork = NestedEntity<uuid::Uuid>;
pub type NestedSubnet = NestedEntity<uuid::Uuid>;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ParentItem {
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub data: ParentData,
    #[serde(default, rename = "type")]
    pub type_field: String,
}

impl ParentItem {
    pub fn into_instance(self, nic_created_at: &str) -> crate::models::InstanceItem {
        let data_state = if !self.data.state.is_empty() {
            self.data.state.clone()
        } else {
            String::new()
        };

        let config = &self.data.config;

        let (ip_address, network_name) = config
            .addresses
            .iter()
            .find(|(_, addrs)| addrs.iter().any(|addr| addr.version == 4))
            .map(|(name, addrs)| {
                let ip = addrs
                    .iter()
                    .find(|addr| addr.version == 4)
                    .map(|addr| addr.addr.clone());
                (ip, Some(name.clone()))
            })
            .unwrap_or((None, None));

        let flavor = &config.flavor;
        let image = &config.source_image;
        let az = &config.availability_zone;

        crate::models::InstanceItem {
            id: parse_uuid_or_warn(&self.item_id, "port.id"),
            name: config.name.clone(),
            status: data_state,
            flavor_id: flavor.id,
            flavor_name: flavor.name.clone(),
            image_id: image.id,
            image_name: image.name.clone(),
            ip_address,
            network_name,
            availability_zone: if !az.name.is_empty() {
                az.name.clone()
            } else {
                az.id.clone()
            },
            created_at: nic_created_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ParentData {
    #[serde(default)]
    pub config: ParentConfig,
    #[serde(default)]
    pub state: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ParentConfig {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub addresses: HashMap<String, Vec<NestedAddress>>,
    #[serde(default)]
    pub flavor: NestedEntity<uuid::Uuid>,
    #[serde(default)]
    pub source_image: NestedEntity<uuid::Uuid>,
    #[serde(default)]
    pub availability_zone: NestedEntity<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct InstanceConfig {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub flavor: NestedEntity<uuid::Uuid>,
    #[serde(default)]
    pub source_image: NestedEntity<uuid::Uuid>,
    #[serde(default)]
    pub addresses: HashMap<String, Vec<NestedAddress>>,
    #[serde(default)]
    pub availability_zone: NestedEntity<String>,
}

pub type NestedFlavor = NestedEntity<uuid::Uuid>;
pub type NestedImage = NestedEntity<uuid::Uuid>;
pub type NestedAz = NestedEntity<String>;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct NestedAddress {
    #[serde(default)]
    pub addr: String,
    #[serde(default)]
    pub version: i32,
}

impl PortWrapper {
    pub fn into_network_item(self) -> crate::models::NetworkItem {
        crate::models::NetworkItem::Nic(self.into_nic())
    }

    pub fn into_nic(self) -> crate::models::NicPort {
        let config = &self.data.config;
        let first_ip = config.fixed_ips.first();
        let parent_config = self
            .data
            .parent_item
            .as_ref()
            .map(|p| p.data.config.name.clone());

        let state = if !self.state.is_empty() {
            self.state.clone()
        } else {
            self.data.state.clone()
        };

        let ip_address = first_ip.as_ref().and_then(|ip| ip.ip_address.clone());

        crate::models::NicPort {
            id: parse_uuid_or_warn(&self.item_id, "nic.id"),
            name: config.name.clone(),
            mac_address: config.mac_address.clone(),
            ip_address,
            subnet_id: first_ip.as_ref().and_then(|ip| ip.subnet_id),
            subnet_name: config.subnet.as_ref().map(|s| s.name.clone()),
            network_id: config.network.as_ref().map(|n| n.id).unwrap_or_default(),
            network_name: config
                .network
                .as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default(),
            security_group_ids: config.security_groups.iter().map(|sg| sg.id).collect(),
            security_group_names: config
                .security_groups
                .iter()
                .map(|sg| sg.name.clone())
                .collect(),
            instance_id: self.data.parent.and_then(|p| p.parse().ok()),
            instance_name: parent_config,
            state,
            availability_zone: if !config.availability_zone.name.is_empty() {
                Some(config.availability_zone.name.clone())
            } else if !config.availability_zone.id.is_empty() {
                Some(config.availability_zone.id.clone())
            } else {
                None
            },
        }
    }

    pub fn into_instance(self) -> crate::models::InstanceItem {
        let data_state = if !self.state.is_empty() {
            self.state.clone()
        } else {
            self.data.state.clone()
        };

        let config = &self.data.config;

        let (ip_address, network_name) = config
            .addresses
            .iter()
            .find(|(_, addrs)| addrs.iter().any(|addr| addr.version == 4))
            .map(|(name, addrs)| {
                let ip = addrs
                    .iter()
                    .find(|addr| addr.version == 4)
                    .map(|addr| addr.addr.clone());
                (ip, Some(name.clone()))
            })
            .unwrap_or((None, None));

        let flavor = &config.flavor;
        let image = &config.source_image;
        let az = &config.availability_zone;

        crate::models::InstanceItem {
            id: parse_uuid_or_warn(&self.item_id, "port.id"),
            name: config.name.clone(),
            status: data_state,
            flavor_id: flavor.id,
            flavor_name: flavor.name.clone(),
            image_id: image.id,
            image_name: image.name.clone(),
            ip_address,
            network_name,
            availability_zone: if !az.name.is_empty() {
                az.name.clone()
            } else {
                az.id.clone()
            },
            created_at: self.created_at.clone(),
        }
    }
}

impl From<PortWrapper> for crate::models::Ports {
    fn from(wrapper: PortWrapper) -> Self {
        let config = &wrapper.data.config;
        let first_ip = config.fixed_ips.first();
        let parent_config = wrapper
            .data
            .parent_item
            .as_ref()
            .map(|p| p.data.config.name.clone());

        let state = if !wrapper.state.is_empty() {
            wrapper.state.clone()
        } else {
            wrapper.data.state.clone()
        };

        crate::models::Ports {
            id: parse_uuid_or_warn(&wrapper.item_id, "port.id"),
            name: config.name.clone(),
            mac_address: config.mac_address.clone(),
            ip_address: first_ip.as_ref().and_then(|ip| ip.ip_address.clone()),
            subnet_id: first_ip.as_ref().and_then(|ip| ip.subnet_id),
            subnet_name: config.subnet.as_ref().map(|s| s.name.clone()),
            network_id: config.network.as_ref().map(|n| n.id).unwrap_or_default(),
            network_name: config
                .network
                .as_ref()
                .map(|n| n.name.clone())
                .unwrap_or_default(),
            security_group_ids: config.security_groups.iter().map(|sg| sg.id).collect(),
            security_group_names: config
                .security_groups
                .iter()
                .map(|sg| sg.name.clone())
                .collect(),
            instance_id: wrapper.data.parent.and_then(|p| p.parse().ok()),
            instance_name: parent_config,
            state,
        }
    }
}
