use crate::models::StatusEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum RouteTableStatus {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "error")]
    Error,
}

impl std::fmt::Display for RouteTableStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouteTableStatus::Available => write!(f, "available"),
            RouteTableStatus::Pending => write!(f, "pending"),
            RouteTableStatus::Deleted => write!(f, "deleted"),
            RouteTableStatus::Error => write!(f, "error"),
        }
    }
}

impl StatusEnum for RouteTableStatus {
    fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "available" => RouteTableStatus::Available,
            "pending" => RouteTableStatus::Pending,
            "deleted" => RouteTableStatus::Deleted,
            _ => RouteTableStatus::Error,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RouteTableRoute {
    pub prefix: String,
    pub next_hop: String,
    pub next_hop_type: String,
    pub raw_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RouteTableRegion {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RouteTableNetwork {
    pub name: String,
    pub status: String,
    pub id: Uuid,
    pub description: Option<String>,
    pub create_time: String,
    pub shared_from: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RouteTables {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub routes: Vec<RouteTableRoute>,
    pub status: RouteTableStatus,
    pub create_date: String,
    pub region: RouteTableRegion,
    pub networks: Vec<RouteTableNetwork>,
}
