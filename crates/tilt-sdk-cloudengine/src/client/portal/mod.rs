use crate::client::paginate::paginate;
use crate::error::{ComputeError, Result, Service};
use crate::models::SshKeys;
use tilt_sdk::Client;
use tracing::{Instrument, debug, info_span};

const PORTAL_SERVICE: Service = Service::PortalApi;

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
}
