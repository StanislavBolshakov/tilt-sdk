use crate::client::paginate::paginate;
use crate::models::{
    AvailabilityZone, Backups, Flavors, ImageUpload, Images, Instances, PlacementPolicy, Regions,
    Tasks,
};
use tilt_sdk::Client;
use tracing::{Instrument, debug, info_span};
use uuid::Uuid;

use super::responses::{
    AvailabilityZoneWrapper, AvailabilityZonesResponse, BackupsResponse, FlavorWrapper,
    FlavorsResponse, ImageUploadsResponse, ImageWrapper, InstanceWrapper, InstancesResponse,
    PlacementPoliciesResponse, PlacementPolicyWrapper, ProjectImagesResponse, RegionsResponse,
    SharedImagesResponse, TaskResponse, TasksResponse,
};
use crate::error::{ComputeError, Result, Service};

const COMPUTE_SERVICE: Service = Service::ComputeApi;

pub struct ComputeClient<'a> {
    client: &'a Client,
}

impl<'a> ComputeClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let span = info_span!("compute_get", path);
        async move {
            debug!(path, "Fetching resource");
            match self.client.http().get(path).await {
                Ok(response) => {
                    debug!(path, "Successfully fetched resource");
                    Ok(response)
                }
                Err(e) => Err(ComputeError::from_sdk_error(e, COMPUTE_SERVICE, Some(path))),
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
        let span = info_span!("compute_get_with_query", path);
        async move {
            debug!(path, ?query, "Fetching resources with query");
            match self.client.http().get_with_query(path, query).await {
                Ok(response) => {
                    debug!(path, ?query, "Successfully fetched resources");
                    Ok(response)
                }
                Err(e) => Err(ComputeError::from_sdk_error(e, COMPUTE_SERVICE, Some(path))),
            }
        }
        .instrument(span)
        .await
    }

    pub async fn list_instances(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Instances>> {
        let path = format!(
            "/order-service/api/v1/projects/{}/compute/instances",
            self.client.project()
        );

        let this = &self;
        paginate(
            limit,
            page,
            Service::OrderService,
            &path,
            |page: u32, limit: u32| {
                let path = path.clone();
                async move {
                    let response: InstancesResponse = this
                        .get_with_query(
                            &path,
                            &[
                                ("include", "total_count"),
                                ("page", &page.to_string()),
                                ("per_page", &limit.to_string()),
                            ],
                        )
                        .await?;
                    let items: Vec<Instances> =
                        response.list.into_iter().map(|w| w.into()).collect();
                    Ok((items, response.meta.total_count))
                }
            },
        )
        .await
    }

    pub async fn get_instance(&self, instance_id: Uuid) -> Result<Instances> {
        let path = format!(
            "/order-service/api/v1/projects/{}/compute/instances/{}",
            self.client.project(),
            instance_id
        );
        let response: InstanceWrapper = self.get(&path).await?;
        Ok(response.into())
    }

    pub async fn list_flavors(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
        az_id: Option<&str>,
    ) -> Result<Vec<Flavors>> {
        let path = "/api/v1/flavors";
        let az_id = az_id.map(|s| s.to_string());

        let this = &self;
        paginate(
            limit,
            page,
            Service::ComputeApi,
            path,
            |page: u32, limit: u32| {
                let path = path.to_string();
                let az_id = az_id.clone();
                async move {
                    let page_str = page.to_string();
                    let per_page_str = limit.to_string();
                    let mut query: Vec<(&str, &str)> = vec![
                        ("include", "total_count"),
                        ("page", &page_str),
                        ("per_page", &per_page_str),
                    ];
                    if let Some(az) = &az_id {
                        query.push(("availability_zone_id", az));
                    }

                    let response: FlavorsResponse = this.get_with_query(&path, &query).await?;
                    let items: Vec<Flavors> = response.list.into_iter().map(Into::into).collect();
                    Ok((items, response.meta.total_count))
                }
            },
        )
        .await
    }

    pub async fn get_flavor(&self, flavor_id: Uuid) -> Result<Flavors> {
        let path = format!("/api/v1/flavors/{}", flavor_id);
        let response: FlavorWrapper = self.get(&path).await?;
        Ok(response.into())
    }

    pub async fn list_project_images(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Images>> {
        let path = format!("/compute/api/v1/projects/{}/images", self.client.project());

        let this = &self;
        paginate(
            limit,
            page,
            Service::ComputeApi,
            &path,
            |page: u32, limit: u32| {
                let path = path.clone();
                async move {
                    let response: ProjectImagesResponse = this
                        .get_with_query(
                            &path,
                            &[
                                ("include", "total_count"),
                                ("page", &page.to_string()),
                                ("per_page", &limit.to_string()),
                            ],
                        )
                        .await?;
                    let items: Vec<Images> = response.list.into_iter().map(|w| w.into()).collect();
                    Ok((items, response.meta.total_count))
                }
            },
        )
        .await
    }

    pub async fn list_shared_images(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Images>> {
        let path = format!(
            "/compute/api/v1/projects/{}/shared-images",
            self.client.project()
        );

        let this = &self;
        paginate(
            limit,
            page,
            Service::ComputeApi,
            &path,
            |page: u32, limit: u32| {
                let path = path.clone();
                async move {
                    let response: SharedImagesResponse = this
                        .get_with_query(
                            &path,
                            &[
                                ("include", "total_count"),
                                ("page", &page.to_string()),
                                ("per_page", &limit.to_string()),
                            ],
                        )
                        .await?;
                    let items: Vec<Images> = response.list.into_iter().map(|w| w.into()).collect();
                    Ok((items, response.meta.total_count))
                }
            },
        )
        .await
    }

    pub async fn list_image_uploads(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<ImageUpload>> {
        let path = format!(
            "/image-uploader/api/v1/projects/{}/image-uploads",
            self.client.project()
        );

        let this = &self;
        paginate(
            limit,
            page,
            Service::ComputeApi,
            &path,
            |page: u32, limit: u32| {
                let path = path.clone();
                async move {
                    let response: ImageUploadsResponse = this
                        .get_with_query(
                            &path,
                            &[
                                ("include", "total_count"),
                                ("page", &page.to_string()),
                                ("per_page", &limit.to_string()),
                            ],
                        )
                        .await?;
                    let items: Vec<ImageUpload> =
                        response.list.into_iter().map(|w| w.into()).collect();
                    Ok((items, response.meta.total_count))
                }
            },
        )
        .await
    }

    pub async fn list_images(&self, include_shared: bool) -> Result<Vec<Images>> {
        let project = self.list_project_images(None, None).await?;
        if !include_shared {
            return Ok(project);
        }
        let shared = self.list_shared_images(None, None).await?;
        Ok(project.into_iter().chain(shared.into_iter()).collect())
    }

    pub async fn get_image(&self, image_id: Uuid) -> Result<Images> {
        let path = format!(
            "/compute/api/v1/projects/{}/images/{}",
            self.client.project(),
            image_id
        );
        let response: ImageWrapper = self.get(&path).await?;
        Ok(response.into())
    }

    pub async fn list_tasks(&self) -> Result<Vec<Tasks>> {
        let path = format!(
            "/order-service/api/v1/projects/{}/tasks",
            self.client.project()
        );
        let response: TasksResponse = self
            .get_with_query(&path, &[("include", "total_count")])
            .await?;
        Ok(response.list.into_iter().map(Into::into).collect())
    }

    pub async fn get_task(&self, task_id: Uuid) -> Result<Tasks> {
        let path = format!(
            "/order-service/api/v1/projects/{}/tasks/{}",
            self.client.project(),
            task_id
        );
        let response: TaskResponse = self.get(&path).await?;
        Ok(response.data.into())
    }

    pub async fn get_region(&self, region_id: Uuid) -> Result<Regions> {
        let path = format!("/api/v1/regions/{}", region_id);
        let response: super::responses::RegionWrapper = self.get(&path).await?;
        Ok(response.into())
    }

    pub async fn get_availability_zone(&self, az_id: Uuid) -> Result<AvailabilityZone> {
        let path = format!(
            "/compute/api/v1/projects/{}/availability-zones/{}",
            self.client.project(),
            az_id
        );
        let response: AvailabilityZoneWrapper = self.get(&path).await?;
        Ok(response.into())
    }

    pub async fn list_regions(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Regions>> {
        let path = format!("/compute/api/v1/projects/{}/regions", self.client.project());

        let this = &self;
        paginate(
            limit,
            page,
            Service::ComputeApi,
            &path,
            |page: u32, limit: u32| {
                let path = path.clone();
                async move {
                    let response: RegionsResponse = this
                        .get_with_query(
                            &path,
                            &[
                                ("include", "total_count"),
                                ("page", &page.to_string()),
                                ("per_page", &limit.to_string()),
                            ],
                        )
                        .await?;
                    let items: Vec<Regions> = response.iter().cloned().map(Into::into).collect();
                    let total_count = response.len() as u32;
                    Ok((items, total_count))
                }
            },
        )
        .await
    }

    pub async fn list_availability_zones(&self) -> Result<Vec<AvailabilityZone>> {
        let path = format!(
            "/compute/api/v1/projects/{}/availability-zones",
            self.client.project()
        );
        let response: AvailabilityZonesResponse = self.get_with_query(&path, &[]).await?;
        Ok(response.0.into_iter().map(Into::into).collect())
    }

    pub async fn list_placement_policies(&self) -> Result<Vec<PlacementPolicy>> {
        let path = format!(
            "/compute/api/v1/projects/{}/placement-policies",
            self.client.project()
        );
        let response: PlacementPoliciesResponse = self.get(&path).await?;
        Ok(response.list.into_iter().map(Into::into).collect())
    }

    pub async fn get_placement_policy(&self, placement_policy_id: Uuid) -> Result<PlacementPolicy> {
        let path = format!(
            "/api/v1/projects/{}/placement-policies/{}",
            self.client.project(),
            placement_policy_id
        );
        let response: PlacementPolicyWrapper = self.get(&path).await?;
        Ok(response.into())
    }

    pub async fn list_backups(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Backups>> {
        let path = format!(
            "/order-service/api/v1/projects/{}/compute/backup_containers",
            self.client.project()
        );

        let this = &self;
        paginate(
            limit,
            page,
            Service::OrderService,
            &path,
            |page: u32, limit: u32| {
                let path = path.clone();
                async move {
                    let response: BackupsResponse = this
                        .get_with_query(
                            &path,
                            &[
                                ("with_children", "false"),
                                ("with_all_children", "false"),
                                ("include", "total_count"),
                                ("page", &page.to_string()),
                                ("per_page", &limit.to_string()),
                            ],
                        )
                        .await?;
                    let items: Vec<Backups> = response.list.into_iter().map(Into::into).collect();
                    Ok((items, response.meta.total_count))
                }
            },
        )
        .await
    }
}
