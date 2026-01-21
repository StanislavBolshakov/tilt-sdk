use crate::models::ListResponse;
use serde::Deserialize;

pub type PlacementPoliciesResponse = ListResponse<PlacementPolicyWrapper>;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PlacementPolicyWrapper {
    #[serde(rename = "item_id")]
    pub id: uuid::Uuid,
    #[serde(rename = "created_row_dt")]
    pub created_time: String,
    pub data: PlacementPolicyDataWrapper,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PlacementPolicyDataWrapper {
    pub state: String,
    pub config: PlacementPolicyConfigWrapper,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PlacementPolicyConfigWrapper {
    pub name: String,
    pub policy_type: Option<String>,
    #[serde(rename = "availability_zone")]
    pub availability_zone: Option<PlacementAzWrapper>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PlacementAzWrapper {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl From<PlacementPolicyWrapper> for crate::models::PlacementPolicy {
    fn from(wrapper: PlacementPolicyWrapper) -> Self {
        crate::models::PlacementPolicy {
            id: wrapper.id,
            name: wrapper.data.config.name,
            policy_type: wrapper.data.config.policy_type,
            availability_zone: wrapper.data.config.availability_zone.map(|az| az.name),
        }
    }
}
