use crate::models::{InstanceStatus, ListResponse, NestedEntity, StatusEnum};
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

pub type InstancesResponse = ListResponse<InstanceWrapper>;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct InstanceWrapper {
    #[serde(default)]
    #[serde(rename = "created_row_dt")]
    pub created_row_dt: String,
    #[serde(default)]
    #[serde(rename = "data")]
    pub data: InstanceDataWrapper,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct InstanceDataWrapper {
    #[serde(default)]
    #[serde(rename = "state")]
    pub state: String,
    #[serde(default)]
    #[serde(rename = "config")]
    pub config: InstanceConfigWrapper,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct InstanceConfigWrapper {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "flavor")]
    pub flavor: FlavorWrapper,
    #[serde(default)]
    #[serde(rename = "source_image")]
    pub source_image: Option<SourceImageWrapper>,
    #[serde(default)]
    #[serde(rename = "availability_zone")]
    pub availability_zone: Option<NestedEntity<String>>,
    #[serde(default)]
    #[serde(rename = "accessIPv4")]
    pub access_ipv4: String,
    #[serde(default)]
    #[serde(rename = "addresses")]
    pub addresses: Option<HashMap<String, Vec<Address>>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FlavorWrapper {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "vcpus")]
    pub vcpus: u64,
    #[serde(default)]
    #[serde(rename = "ram")]
    pub ram: u64,
    #[serde(default)]
    #[serde(rename = "gpus")]
    pub gpus: u64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SourceImageWrapper {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
}

pub type AvailabilityZoneWrapper = NestedEntity<String>;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Address {
    #[serde(default)]
    #[serde(rename = "addr")]
    pub addr: String,
}

impl From<InstanceWrapper> for crate::models::Instances {
    fn from(wrapper: InstanceWrapper) -> Self {
        let config = wrapper.data.config;
        let addresses = config.addresses.unwrap_or_default();

        let all_addresses: Vec<String> = addresses
            .values()
            .flat_map(|ips| ips.iter().map(|ip| ip.addr.clone()))
            .collect();

        crate::models::Instances {
            id: parse_uuid_or_warn(&config.id, "instance.id"),
            name: config.name,
            status: InstanceStatus::from_string(&wrapper.data.state),
            flavor_id: parse_uuid_or_warn(&config.flavor.id, "flavor.id"),
            flavor_name: Some(config.flavor.name),
            flavor_vcpus: Some(config.flavor.vcpus),
            flavor_ram_mb: Some(config.flavor.ram),
            flavor_gpus: Some(config.flavor.gpus),
            image_id: config.source_image.as_ref().and_then(|i| i.id.parse().ok()),
            image_name: config.source_image.as_ref().map(|i| i.name.clone()),
            availability_zone: config.availability_zone.as_ref().map(|az| az.name.clone()),
            all_addresses,
            created_row_dt: wrapper.created_row_dt,
        }
    }
}
