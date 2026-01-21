use serde::Deserialize;

pub type NetworksResponse = Vec<NetworkWrapper>;

#[derive(Debug, Clone, Deserialize)]
pub struct NetworkWrapper {
    pub id: uuid::Uuid,
    pub name: String,
    pub status: Option<String>,
    pub description: Option<String>,
    pub create_time: Option<String>,
    pub shared_from: Option<String>,
}

impl Default for NetworkWrapper {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::nil(),
            name: String::new(),
            status: None,
            description: None,
            create_time: None,
            shared_from: None,
        }
    }
}

impl From<NetworkWrapper> for crate::models::Networks {
    fn from(wrapper: NetworkWrapper) -> Self {
        crate::models::Networks {
            id: wrapper.id,
            name: wrapper.name,
            status: wrapper.status.unwrap_or_default(),
            description: wrapper.description,
            create_time: wrapper.create_time,
            shared_from: wrapper.shared_from,
        }
    }
}
