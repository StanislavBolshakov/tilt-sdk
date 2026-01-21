use serde::Deserialize;

pub type SecurityGroupsResponse = Vec<SecurityGroupWrapper>;
pub type SecurityGroupRulesResponse = Vec<SecurityGroupRuleWrapper>;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SecurityGroupWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SecurityGroupRuleWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
    #[serde(default)]
    #[serde(rename = "security_group_id")]
    pub security_group_id: uuid::Uuid,
    #[serde(default)]
    pub direction: String,
    #[serde(default)]
    pub protocol: String,
    #[serde(default)]
    pub ethertype: String,
    #[serde(default)]
    #[serde(rename = "port_range_min")]
    pub port_range_min: Option<u32>,
    #[serde(default)]
    #[serde(rename = "port_range_max")]
    pub port_range_max: Option<u32>,
    #[serde(default)]
    #[serde(rename = "remote_ip_prefix")]
    pub remote_ip_prefix: Option<String>,
    #[serde(default)]
    #[serde(rename = "remote_group")]
    pub remote_group: Option<RemoteGroupWrapper>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RemoteGroupWrapper {
    #[serde(default)]
    pub id: uuid::Uuid,
}

impl From<SecurityGroupWrapper> for crate::models::SecurityGroups {
    fn from(wrapper: SecurityGroupWrapper) -> Self {
        crate::models::SecurityGroups {
            id: wrapper.id,
            name: wrapper.name,
            description: wrapper.description,
            status: wrapper.status.unwrap_or_default(),
            create_time: wrapper.create_time,
            update_time: wrapper.update_time,
        }
    }
}

impl From<SecurityGroupRuleWrapper> for crate::models::SecurityGroupRule {
    fn from(wrapper: SecurityGroupRuleWrapper) -> Self {
        crate::models::SecurityGroupRule {
            id: wrapper.id,
            security_group_id: wrapper.security_group_id,
            direction: wrapper.direction,
            protocol: wrapper.protocol,
            ethertype: wrapper.ethertype,
            port_range_min: wrapper.port_range_min,
            port_range_max: wrapper.port_range_max,
            remote_ip_prefix: wrapper.remote_ip_prefix,
            remote_group_id: wrapper.remote_group.map(|g| g.id),
            description: wrapper.description,
            status: wrapper.status,
        }
    }
}
