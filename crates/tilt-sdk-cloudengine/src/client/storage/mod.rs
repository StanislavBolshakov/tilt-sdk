use crate::client::paginate::paginate;
use crate::error::{ComputeError, Result, Service};
use crate::models::{Snapshots, VolumeType, Volumes};
use tilt_sdk::Client;
use tracing::{Instrument, debug, info_span};
use uuid::Uuid;

use super::responses::{
    SnapshotsResponse, VolumeTypeWrapper, VolumeTypesResponse, VolumeWrapper, VolumesResponse,
};

const STORAGE_SERVICE: Service = Service::OrderService;

pub struct StorageClient<'a> {
    client: &'a Client,
}

impl<'a> StorageClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let span = info_span!("storage_get", path);
        async move {
            debug!(path, "Fetching resource");
            match self.client.http().get(path).await {
                Ok(response) => {
                    debug!(path, "Successfully fetched resource");
                    Ok(response)
                }
                Err(e) => Err(ComputeError::from_sdk_error(e, STORAGE_SERVICE, Some(path))),
            }
        }
        .instrument(span)
        .await
    }

    async fn get_with_query<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T> {
        let span = info_span!("storage_get_with_query", path);
        async move {
            debug!(path, ?query, "Fetching resources with query");
            match self.client.http().get_with_query(path, query).await {
                Ok(response) => {
                    debug!(path, ?query, "Successfully fetched resources");
                    Ok(response)
                }
                Err(e) => Err(ComputeError::from_sdk_error(e, STORAGE_SERVICE, Some(path))),
            }
        }
        .instrument(span)
        .await
    }

    pub async fn list_volume_types(&self) -> Result<Vec<VolumeType>> {
        let path = "/api/v1/volume-types";
        let response: VolumeTypesResponse = self.get(path).await?;
        Ok(response.0.into_iter().map(Into::into).collect())
    }

    pub async fn get_volume_type(&self, volume_type_id: Uuid) -> Result<VolumeType> {
        let path = format!("/api/v1/volume-types/{}", volume_type_id);
        let response: VolumeTypeWrapper = self.get(&path).await?;
        Ok(response.into())
    }

    pub async fn list_volumes(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Volumes>> {
        let path = format!(
            "/order-service/api/v1/projects/{}/compute/volumes",
            self.client.project()
        );

        let this = &self;
        paginate(
            limit,
            page,
            STORAGE_SERVICE,
            &path,
            |page: u32, limit: u32| {
                let path = path.clone();
                async move {
                    let response: VolumesResponse = this
                        .get_with_query(
                            &path,
                            &[
                                ("include", "total_count"),
                                ("page", &page.to_string()),
                                ("per_page", &limit.to_string()),
                            ],
                        )
                        .await?;
                    let items: Vec<Volumes> = response.list.into_iter().map(|w| w.into()).collect();
                    Ok((items, response.meta.total_count))
                }
            },
        )
        .await
    }

    pub async fn get_volume(&self, volume_id: Uuid) -> Result<Volumes> {
        let path = format!(
            "/order-service/api/v1/projects/{}/compute/volumes/{}",
            self.client.project(),
            volume_id
        );
        let response: VolumeWrapper = self.get(&path).await?;
        Ok(response.into())
    }

    pub async fn list_snapshots(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Snapshots>> {
        let path = format!(
            "/order-service/api/v1/projects/{}/compute/snapshots",
            self.client.project()
        );

        let this = &self;
        paginate(
            limit,
            page,
            STORAGE_SERVICE,
            &path,
            |page: u32, limit: u32| {
                let path = path.clone();
                async move {
                    let response: SnapshotsResponse = this
                        .get_with_query(
                            &path,
                            &[
                                ("include", "total_count"),
                                ("page", &page.to_string()),
                                ("per_page", &limit.to_string()),
                            ],
                        )
                        .await?;
                    let items: Vec<Snapshots> =
                        response.list.into_iter().map(|w| w.into()).collect();
                    Ok((items, response.meta.total_count))
                }
            },
        )
        .await
    }
}
