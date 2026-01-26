use crate::client::paginate::paginate;
use crate::error::{ComputeError, Result, Service};
use crate::models::SshKeys;
use serde::{Deserialize, Serialize};
use tilt_sdk::Client;
use tracing::{Instrument, debug, info_span};

const PORTAL_SERVICE: Service = Service::PortalApi;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSshKeyRequest {
    pub ssh_key: SshKeyDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKeyDetails {
    pub public_keys: Vec<String>,
    pub login: String,
    pub name: String,
}

pub struct PortalClient<'a> {
    client: &'a Client,
}

impl<'a> PortalClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    async fn get_with_query<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T> {
        let span = info_span!("portal_get_with_query", path);
        async move {
            debug!(path, ?query, "Fetching resources with query");
            match self.client.http().get_with_query(path, query).await {
                Ok(response) => {
                    debug!(path, ?query, "Successfully fetched resources");
                    Ok(response)
                }
                Err(e) => Err(ComputeError::from_sdk_error(e, PORTAL_SERVICE, Some(path))),
            }
        }
        .instrument(span)
        .await
    }

    async fn post<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let span = info_span!("portal_post", path);
        async move {
            debug!(path, "Creating resource");
            match self.client.http().post(path, body).await {
                Ok(response) => {
                    debug!(path, "Successfully created resource");
                    Ok(response)
                }
                Err(e) => Err(ComputeError::from_sdk_error(e, PORTAL_SERVICE, Some(path))),
            }
        }
        .instrument(span)
        .await
    }

    async fn delete(&self, path: &str) -> Result<serde_json::Value> {
        let span = info_span!("portal_delete", path);
        async move {
            debug!(path, "Deleting resource");
            self.client.http().delete::<serde_json::Value>(path).await
                .map_err(|e| ComputeError::from_sdk_error(e, PORTAL_SERVICE, Some(path)))
        }
        .instrument(span)
        .await
    }

    pub async fn list_ssh_keys(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<SshKeys>> {
        let path = format!("/portal/api/v2/projects/{}/ssh_keys", self.client.project());

        let this = &self;
        paginate(
            limit,
            page,
            PORTAL_SERVICE,
            &path,
            |page: u32, limit: u32| {
                let path = path.clone();
                async move {
                    use super::responses::SshKeysResponse;
                    let response: SshKeysResponse = this
                        .get_with_query(
                            &path,
                            &[
                                ("include", "total_count"),
                                ("page", &page.to_string()),
                                ("per_page", &limit.to_string()),
                            ],
                        )
                        .await?;
                    let items: Vec<SshKeys> = response.list.into_iter().map(Into::into).collect();
                    Ok((items, response.meta.total_count))
                }
            },
        )
        .await
    }

    pub async fn create_ssh_key(&self, request: CreateSshKeyRequest) -> Result<SshKeys> {
        let path = format!("/portal/api/v2/projects/{}/ssh_keys", self.client.project());
        let response: super::responses::SshKeyWrapper = self.post(&path, &request).await?;
        Ok(response.into())
    }

    pub async fn delete_ssh_key(&self, ssh_key_id: uuid::Uuid) -> Result<serde_json::Value> {
        let path = format!("/portal/api/v2/projects/{}/ssh_keys/{}", self.client.project(), ssh_key_id);
        self.delete(&path).await
    }
}
