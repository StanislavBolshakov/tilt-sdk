use crate::models::common::extensible::LogSchemaWarnings;
use crate::models::{
    ListResponse, NestedEntity, NetworkRouter, RouterNic, RouterStatus, Routers, StatusEnum,
};
use serde::Deserialize;

pub type RoutersResponse = ListResponse<RouterWrapper>;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct RouterWrapper {
    #[serde(default)]
    pub item_id: uuid::Uuid,
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub data: RouterDataWrapper,
    #[serde(default)]
    #[serde(rename = "created_row_dt")]
    pub created_row_dt: String,
    #[serde(default)]
    pub children_list: Vec<RouterNicWrapper>,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RouterDataWrapper {
    #[serde(default)]
    pub config: RouterConfigWrapper,
    #[serde(default)]
    pub state: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RouterConfigWrapper {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub region: Option<NestedEntity<String>>,
    #[serde(default)]
    pub bandwidth: u64,
    #[serde(default)]
    #[serde(rename = "ip_address")]
    pub ip_address: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RouterNicWrapper {
    #[serde(default)]
    pub item_id: uuid::Uuid,
    #[serde(default)]
    pub data: RouterNicDataWrapper,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RouterNicDataWrapper {
    #[serde(default)]
    pub config: RouterNicConfigWrapper,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RouterNicConfigWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "fixed_ips")]
    pub fixed_ips: Vec<RouterNicIpWrapper>,
    #[serde(default)]
    pub subnet: Option<NestedEntity<String>>,
    #[serde(default)]
    pub network: Option<NestedEntity<String>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RouterNicIpWrapper {
    #[serde(default)]
    #[serde(rename = "ip_address")]
    pub ip_address: String,
}

impl From<RouterWrapper> for Routers {
    fn from(wrapper: RouterWrapper) -> Self {
        wrapper
            ._extra
            .log_unknown_fields("/vpc/api/v1/projects/{project}/snat-routers");

        Routers {
            id: wrapper.item_id,
            name: wrapper.data.config.name,
            status: RouterStatus::from_string(&wrapper.data.state),
            bandwidth: wrapper.data.config.bandwidth,
            ip_address: wrapper.data.config.ip_address,
            availability_zone: wrapper.data.config.region.as_ref().map(|r| {
                if !r.name.is_empty() {
                    r.name.clone()
                } else {
                    r.id.to_string()
                }
            }),
            created_at: wrapper.created_row_dt,
            attached_nics: wrapper
                .children_list
                .into_iter()
                .filter(|nic| nic.data.config.id != uuid::Uuid::nil())
                .map(|nic| RouterNic {
                    id: nic.data.config.id,
                    name: nic.data.config.name,
                    ip_address: nic
                        .data
                        .config
                        .fixed_ips
                        .first()
                        .map(|ip| ip.ip_address.clone())
                        .unwrap_or_default(),
                    network_name: nic.data.config.network.as_ref().map(|n| n.name.clone()),
                    subnet_name: nic.data.config.subnet.as_ref().map(|s| s.name.clone()),
                })
                .collect(),
        }
    }
}

pub type NetworkRoutersResponse = ListResponse<NetworkRouterWrapper>;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct NetworkRouterWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    #[serde(rename = "create_time")]
    pub create_time: String,
    #[serde(default)]
    #[serde(rename = "update_time")]
    pub update_time: Option<String>,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

impl From<NetworkRouterWrapper> for NetworkRouter {
    fn from(wrapper: NetworkRouterWrapper) -> Self {
        NetworkRouter {
            id: wrapper.id,
            name: wrapper.name,
            description: wrapper.description.into(),
            status: wrapper.status,
            create_time: wrapper.create_time,
            update_time: wrapper.update_time,
        }
    }
}
