use crate::log_schema_drift;
use crate::models::common::status::StatusEnum;
use crate::models::{
    ListResponse, RouteTableNetwork, RouteTableRegion, RouteTableRoute, RouteTableStatus,
    RouteTables,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RouteTableRouteWrapper {
    pub prefix: String,
    pub next_hop: String,
    pub next_hop_type: String,
    #[serde(default)]
    pub raw_id: Option<String>,
}

impl From<RouteTableRouteWrapper> for RouteTableRoute {
    fn from(wrapper: RouteTableRouteWrapper) -> Self {
        RouteTableRoute {
            prefix: wrapper.prefix,
            next_hop: wrapper.next_hop,
            next_hop_type: wrapper.next_hop_type,
            raw_id: wrapper.raw_id.and_then(|s| uuid::Uuid::parse_str(&s).ok()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RouteTableRegionWrapper {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl From<RouteTableRegionWrapper> for RouteTableRegion {
    fn from(wrapper: RouteTableRegionWrapper) -> Self {
        RouteTableRegion {
            id: uuid::Uuid::parse_str(&wrapper.id).unwrap_or_else(|_| uuid::Uuid::nil()),
            name: wrapper.name,
            description: wrapper.description,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RouteTableNetworkWrapper {
    pub name: String,
    pub status: String,
    pub id: String,
    #[serde(default)]
    pub description: Option<String>,
    pub create_time: String,
    #[serde(default)]
    pub shared_from: Option<String>,
}

impl From<RouteTableNetworkWrapper> for RouteTableNetwork {
    fn from(wrapper: RouteTableNetworkWrapper) -> Self {
        RouteTableNetwork {
            name: wrapper.name,
            status: wrapper.status,
            id: uuid::Uuid::parse_str(&wrapper.id).unwrap_or_else(|_| uuid::Uuid::nil()),
            description: wrapper.description,
            create_time: wrapper.create_time,
            shared_from: wrapper.shared_from,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct RouteTableWrapper {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub routes: Vec<RouteTableRouteWrapper>,
    pub status: String,
    pub id: String,
    pub create_date: String,
    pub region: RouteTableRegionWrapper,
    #[serde(default)]
    pub networks: Vec<RouteTableNetworkWrapper>,
    #[serde(default, flatten)]
    pub _extra: std::collections::HashMap<String, serde_json::Value>,
}

impl From<RouteTableWrapper> for RouteTables {
    fn from(wrapper: RouteTableWrapper) -> Self {
        log_schema_drift!(
            wrapper,
            "/vpc/api/v1/projects/{project}/network_route_tables"
        );

        RouteTables {
            id: uuid::Uuid::parse_str(&wrapper.id).unwrap_or_else(|_| uuid::Uuid::nil()),
            name: wrapper.name,
            description: wrapper.description,
            routes: wrapper.routes.into_iter().map(Into::into).collect(),
            status: RouteTableStatus::from_string(&wrapper.status),
            create_date: wrapper.create_date,
            region: wrapper.region.into(),
            networks: wrapper.networks.into_iter().map(Into::into).collect(),
        }
    }
}

pub type RouteTablesResponse = ListResponse<RouteTableWrapper>;
