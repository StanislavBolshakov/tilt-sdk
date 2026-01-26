use crate::error::{ComputeError, Result, Service};
use crate::models::{
    AvailabilityZone, Backups, Flavors, FloatingIps, ImageUpload, Images, Instances,
    NetworkInterface, NetworkItem, NetworkRouter, Networks, PlacementPolicy, Regions, RouteTables,
    Routers, SecurityGroupRule, SecurityGroups, Snapshots, SshKeys, Subnets, Tasks, VirtualIps,
    VolumeType, Volumes,
};
use tilt_sdk::Client;
use uuid::Uuid;

#[macro_use]
pub mod compute;
#[macro_use]
pub mod network;
pub mod paginate;
#[macro_use]
pub mod portal;
#[macro_use]
pub mod responses;
#[macro_use]
pub mod storage;

pub use network::PortFilter;
pub use paginate::paginate;

const MAX_PAGE_LIMIT: u32 = 100;

pub(crate) fn clamp_page_limit(limit: u32) -> u32 {
    if limit > MAX_PAGE_LIMIT {
        tracing::warn!(
            "Requested limit {} exceeds maximum of {}, using {}",
            limit,
            MAX_PAGE_LIMIT,
            MAX_PAGE_LIMIT
        );
        MAX_PAGE_LIMIT
    } else {
        limit
    }
}

#[derive(Debug, Clone)]
pub enum Resource {
    Instance(Instances),
    Flavor(Flavors),
    Image(Images),
    VolumeType(VolumeType),
    Volume(Volumes),
    Network(Networks),
    SecurityGroup(SecurityGroups),
    PlacementPolicy(PlacementPolicy),
    Region(Regions),
    AvailabilityZone(AvailabilityZone),
    Task(Tasks),
}

pub struct ComputeClient<'a> {
    client: &'a Client,
    compute: compute::ComputeClient<'a>,
    storage: storage::StorageClient<'a>,
    network: network::NetworkClient<'a>,
    portal: portal::PortalClient<'a>,
}

impl<'a> ComputeClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        let compute = compute::ComputeClient::new(client);
        let storage = storage::StorageClient::new(client);
        let network = network::NetworkClient::new(client);
        let portal = portal::PortalClient::new(client);
        Self {
            client,
            compute,
            storage,
            network,
            portal,
        }
    }

    pub fn client(&self) -> &Client {
        self.client
    }

    pub async fn list_instances(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Instances>> {
        self.compute.list_instances(limit, page).await
    }

    pub async fn get_instance(&self, instance_id: Uuid) -> Result<Instances> {
        self.compute.get_instance(instance_id).await
    }

    pub async fn list_flavors(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
        az_id: Option<&str>,
    ) -> Result<Vec<Flavors>> {
        self.compute.list_flavors(limit, page, az_id).await
    }

    pub async fn get_flavor(&self, flavor_id: Uuid) -> Result<Flavors> {
        self.compute.get_flavor(flavor_id).await
    }

    pub async fn list_project_images(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Images>> {
        self.compute.list_project_images(limit, page).await
    }

    pub async fn list_shared_images(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Images>> {
        self.compute.list_shared_images(limit, page).await
    }

    pub async fn list_image_uploads(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<ImageUpload>> {
        self.compute.list_image_uploads(limit, page).await
    }

    pub async fn list_images(&self, include_shared: bool) -> Result<Vec<Images>> {
        self.compute.list_images(include_shared).await
    }

    pub async fn get_image(&self, image_id: Uuid) -> Result<Images> {
        self.compute.get_image(image_id).await
    }

    pub async fn list_volume_types(&self) -> Result<Vec<VolumeType>> {
        self.storage.list_volume_types().await
    }

    pub async fn get_volume_type(&self, volume_type_id: Uuid) -> Result<VolumeType> {
        self.storage.get_volume_type(volume_type_id).await
    }

    pub async fn list_volumes(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Volumes>> {
        self.storage.list_volumes(limit, page).await
    }

    pub async fn get_volume(&self, volume_id: Uuid) -> Result<Volumes> {
        self.storage.get_volume(volume_id).await
    }

    pub async fn list_snapshots(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Snapshots>> {
        self.storage.list_snapshots(limit, page).await
    }

    pub async fn list_regions(&self) -> Result<Vec<Regions>> {
        self.compute.list_regions().await
    }

    pub async fn list_availability_zones(&self) -> Result<Vec<AvailabilityZone>> {
        self.compute.list_availability_zones().await
    }

    pub async fn list_networks(&self) -> Result<Vec<Networks>> {
        self.network.list_networks().await
    }

    pub async fn list_network_interfaces(
        &self,
        instance_id: Uuid,
    ) -> Result<Vec<NetworkInterface>> {
        self.network.list_network_interfaces(instance_id).await
    }

    pub async fn list_security_groups(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<SecurityGroups>> {
        self.network.list_security_groups(limit, page).await
    }

    pub async fn list_security_group_rules(
        &self,
        security_group_id: Uuid,
    ) -> Result<Vec<SecurityGroupRule>> {
        self.network
            .list_security_group_rules(security_group_id)
            .await
    }

    pub async fn list_vips(&self) -> Result<Vec<VirtualIps>> {
        self.network.list_vips().await
    }

    pub async fn list_floating_ips(&self) -> Result<Vec<FloatingIps>> {
        self.network.list_floating_ips().await
    }

    pub async fn list_placement_policies(&self) -> Result<Vec<PlacementPolicy>> {
        self.compute.list_placement_policies().await
    }

    pub async fn list_backups(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<Backups>> {
        self.compute.list_backups(limit, page).await
    }

    pub async fn list_ssh_keys(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<SshKeys>> {
        self.portal.list_ssh_keys(limit, page).await
    }

    pub async fn create_ssh_key(&self, request: crate::client::portal::CreateSshKeyRequest) -> Result<SshKeys> {
        self.portal.create_ssh_key(request).await
    }

    pub async fn delete_ssh_key(&self, ssh_key_id: uuid::Uuid) -> Result<serde_json::Value> {
        self.portal.delete_ssh_key(ssh_key_id).await
    }

    pub async fn list_subnets(&self, network_id: Option<&str>) -> Result<Vec<Subnets>> {
        self.network.list_subnets(network_id).await
    }

    pub async fn delete_subnet(&self, subnet_id: Uuid) -> Result<serde_json::Value> {
        self.network.delete_subnet(subnet_id).await
    }

    pub async fn delete_network(&self, network_id: Uuid) -> Result<serde_json::Value> {
        self.network.delete_network(network_id).await
    }

    pub async fn delete_fip(&self, fip_id: Uuid) -> Result<serde_json::Value> {
        self.network.delete_fip(fip_id).await
    }

    pub async fn delete_security_group(&self, security_group_id: Uuid) -> Result<serde_json::Value> {
        self.network.delete_security_group(security_group_id).await
    }

    pub async fn delete_route_table(&self, route_table_id: Uuid) -> Result<serde_json::Value> {
        self.network.delete_route_table(route_table_id).await
    }

    pub async fn list_ports(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
        filter: PortFilter,
    ) -> Result<Vec<NetworkItem>> {
        self.network.list_ports(limit, page, filter).await
    }

    pub async fn list_routers(&self) -> Result<Vec<Routers>> {
        self.network.list_routers().await
    }

    pub async fn list_network_routers(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<NetworkRouter>> {
        self.network.list_network_routers(limit, page).await
    }

    pub async fn delete_router(&self, router_id: Uuid) -> Result<serde_json::Value> {
        self.network.delete_router(router_id).await
    }

    pub async fn delete_network_router(&self, router_id: Uuid) -> Result<serde_json::Value> {
        self.network.delete_network_router(router_id).await
    }

    pub async fn delete_vip(&self, vip_id: Uuid) -> Result<serde_json::Value> {
        self.network.delete_vip(vip_id).await
    }

    pub async fn delete_port(&self, port_id: Uuid) -> Result<serde_json::Value> {
        self.network.delete_port(port_id).await
    }

    pub async fn list_route_tables(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<RouteTables>> {
        self.network.list_route_tables(limit, page).await
    }

    pub async fn list_tasks(&self) -> Result<Vec<Tasks>> {
        self.compute.list_tasks().await
    }

    pub async fn get_task(&self, task_id: Uuid) -> Result<Tasks> {
        self.compute.get_task(task_id).await
    }

    pub async fn get_resource(&self, object_type: &str, object_id: Uuid) -> Result<Resource> {
        match object_type {
            "instance" | "Instance" => {
                let instance = self.get_instance(object_id).await?;
                Ok(Resource::Instance(instance))
            }
            "flavor" | "Flavor" => {
                let flavor = self.get_flavor(object_id).await?;
                Ok(Resource::Flavor(flavor))
            }
            "image" | "Image" | "shared_image" | "SharedImage" => {
                let image = self.get_image(object_id).await?;
                Ok(Resource::Image(image))
            }
            "volume_type" | "VolumeType" => {
                let volume_type = self.get_volume_type(object_id).await?;
                Ok(Resource::VolumeType(volume_type))
            }
            "volume" | "Volume" => {
                let volume = self.get_volume(object_id).await?;
                Ok(Resource::Volume(volume))
            }
            "network" | "Network" => {
                let network = self.network.get_network(object_id).await?;
                Ok(Resource::Network(network))
            }
            "security_group" | "SecurityGroup" => {
                let sg = self.network.get_security_group(object_id).await?;
                Ok(Resource::SecurityGroup(sg))
            }
            "placement_policy" | "PlacementPolicy" => {
                let pp = self.compute.get_placement_policy(object_id).await?;
                Ok(Resource::PlacementPolicy(pp))
            }
            "region" | "Region" => {
                let region = self.compute.get_region(object_id).await?;
                Ok(Resource::Region(region))
            }
            "availability_zone" | "AvailabilityZone" => {
                let az = self.compute.get_availability_zone(object_id).await?;
                Ok(Resource::AvailabilityZone(az))
            }
            "task" | "Task" => {
                let task = self.get_task(object_id).await?;
                Ok(Resource::Task(task))
            }
            _ => Err(ComputeError::validation(
                Service::Global,
                None,
                format!("Unknown resource type: {}", object_type),
            )),
        }
    }
}
