use crate::client::paginate::paginate;
use crate::client::responses::RouteTablesResponse;
use crate::error::{ComputeError, Result, Service};
use crate::models::{
    FloatingIps, NetworkInterface, NetworkItem, Networks, RouteTables, Routers, SecurityGroupRule,
    SecurityGroups, Subnets, VirtualIps,
};
use tilt_sdk::Client;
use tracing::{Instrument, debug, info_span};
use uuid::Uuid;

use super::responses::{
    FloatingIpsResponse, NetworkInterfacesResponse, NetworksResponse, PortsResponse,
    RoutersResponse, SecurityGroupRulesResponse, SecurityGroupsResponse, SubnetsResponse,
    VipsResponse,
};

const VPC_SERVICE: Service = Service::VpcApi;

#[derive(Debug, PartialEq, Clone)]
pub enum PortFilter {
    All,
    Nic,
    Instance,
}

impl PortFilter {
    pub fn matches(&self, type_field: &str) -> bool {
        match self {
            PortFilter::All => true,
            PortFilter::Nic => type_field == "nic",
            PortFilter::Instance => type_field == "instance",
        }
    }
}

pub struct NetworkClient<'a> {
    client: &'a Client,
}

impl<'a> NetworkClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let span = info_span!("network_get", path);
        async move {
            debug!(path, "Fetching resource");
            match self.client.http().get(path).await {
                Ok(response) => {
                    debug!(path, "Successfully fetched resource");
                    Ok(response)
                }
                Err(e) => Err(ComputeError::from_sdk_error(e, VPC_SERVICE, Some(path))),
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
        let span = info_span!("network_get_with_query", path);
        async move {
            debug!(path, ?query, "Fetching resources with query");
            match self.client.http().get_with_query(path, query).await {
                Ok(response) => {
                    debug!(path, ?query, "Successfully fetched resources");
                    Ok(response)
                }
                Err(e) => Err(ComputeError::from_sdk_error(e, VPC_SERVICE, Some(path))),
            }
        }
        .instrument(span)
        .await
    }

    pub async fn list_networks(&self) -> Result<Vec<Networks>> {
        let path = format!("/vpc/api/v1/projects/{}/networks", self.client.project());
        let response: NetworksResponse = self.get_with_query(&path, &[]).await?;
        Ok(response.into_iter().map(Into::into).collect())
    }

    pub async fn list_network_interfaces(
        &self,
        instance_id: Uuid,
    ) -> Result<Vec<NetworkInterface>> {
        let path = format!(
            "/order-service/api/v1/projects/{}/compute/instances/{}/network-interfaces",
            self.client.project(),
            instance_id
        );
        let response: NetworkInterfacesResponse = self.get(&path).await?;
        Ok(response.interfaces.into_iter().map(Into::into).collect())
    }

    pub async fn list_security_groups(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<SecurityGroups>> {
        let path = format!(
            "/vpc/api/v1/projects/{}/security-groups",
            self.client.project()
        );

        let this = &self;
        paginate(
            limit,
            page,
            Service::VpcApi,
            &path,
            |page: u32, limit: u32| {
                let path = path.clone();
                async move {
                    let response: SecurityGroupsResponse = this
                        .get_with_query(
                            &path,
                            &[
                                ("include", "total_count"),
                                ("page", &page.to_string()),
                                ("per_page", &limit.to_string()),
                            ],
                        )
                        .await?;
                    let items: Vec<SecurityGroups> = response.into_iter().map(Into::into).collect();
                    let total_count = items.len() as u32;
                    Ok((items, total_count))
                }
            },
        )
        .await
    }

    pub async fn list_security_group_rules(
        &self,
        security_group_id: Uuid,
    ) -> Result<Vec<SecurityGroupRule>> {
        let path = format!(
            "/vpc/api/v1/projects/{}/security-groups/{}/rules",
            self.client.project(),
            security_group_id
        );
        let response: SecurityGroupRulesResponse = self.get_with_query(&path, &[]).await?;
        Ok(response.into_iter().map(Into::into).collect())
    }

    pub async fn list_vips(&self) -> Result<Vec<VirtualIps>> {
        let path = format!(
            "/vpc/api/v1/projects/{}/virtual-ip-addresses",
            self.client.project()
        );
        let response: VipsResponse = self.get_with_query(&path, &[("entire", "true")]).await?;
        Ok(response.list.into_iter().map(Into::into).collect())
    }

    pub async fn list_floating_ips(&self) -> Result<Vec<FloatingIps>> {
        let path = format!(
            "/vpc/api/v1/projects/{}/public-ip-addresses",
            self.client.project()
        );
        let response: FloatingIpsResponse =
            self.get_with_query(&path, &[("entire", "true")]).await?;
        Ok(response.list.into_iter().map(Into::into).collect())
    }

    pub async fn list_subnets(&self, network_id: Option<&str>) -> Result<Vec<Subnets>> {
        let path = format!("/vpc/api/v1/projects/{}/subnets", self.client.project());

        let query: Vec<(&str, &str)> = if let Some(id) = network_id {
            vec![("network_id", id)]
        } else {
            vec![]
        };

        let response: SubnetsResponse = self.get_with_query(&path, &query).await?;
        Ok(response.into_iter().map(Into::into).collect())
    }

    pub async fn list_ports(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
        filter: PortFilter,
    ) -> Result<Vec<NetworkItem>> {
        let path = format!(
            "/vpc/api/v1/projects/{}/network-interfaces",
            self.client.project()
        );

        match limit {
            Some(input_limit) => {
                let limit = super::clamp_page_limit(input_limit);
                let page = page.unwrap_or(1);
                let response: PortsResponse = self
                    .get_with_query(
                        &path,
                        &[
                            ("projectName", self.client.project()),
                            ("entire", "true"),
                            ("page", &page.to_string()),
                            ("per_page", &limit.to_string()),
                        ],
                    )
                    .await?;

                let mut items = Vec::new();

                for port in response.list {
                    if filter.matches(&port.type_field) {
                        items.push(port.clone().into_network_item());
                    }

                    if let Some(ref parent_item) = port.data.parent_item
                        && parent_item.type_field == "instance"
                        && (filter == PortFilter::All || filter == PortFilter::Instance)
                    {
                        items.push(NetworkItem::Instance(
                            parent_item.clone().into_instance(&port.created_at),
                        ));
                    }
                }

                Ok(items)
            }
            None => {
                let mut all_items: Vec<NetworkItem> = Vec::new();
                let mut page = 1;
                let per_page = 100;
                let mut first_page_total: Option<usize> = None;

                loop {
                    let response: PortsResponse = self
                        .get_with_query(
                            &path,
                            &[
                                ("projectName", self.client.project()),
                                ("entire", "true"),
                                ("page", &page.to_string()),
                                ("per_page", &per_page.to_string()),
                            ],
                        )
                        .await?;

                    if first_page_total.is_none() {
                        first_page_total = Some(response.meta.total_count);
                    }

                    if response.list.is_empty() {
                        break;
                    }

                    for port in &response.list {
                        let has_instance_parent = port.data.parent_item.is_some()
                            && port.data.parent_item.as_ref().unwrap().type_field == "instance";

                        if filter.matches(&port.type_field) || has_instance_parent {
                            all_items.push(port.clone().into_network_item());
                        }

                        if has_instance_parent
                            && (filter == PortFilter::All || filter == PortFilter::Instance)
                        {
                            all_items.push(NetworkItem::Instance(
                                port.data
                                    .parent_item
                                    .as_ref()
                                    .unwrap()
                                    .clone()
                                    .into_instance(&port.created_at),
                            ));
                        }
                    }

                    let total_count = first_page_total.unwrap_or(0);
                    let expected_pages = (total_count as f64 / per_page as f64).ceil() as u32;
                    if all_items.len() >= total_count || page >= expected_pages {
                        break;
                    }

                    page += 1;

                    if page > 100 {
                        return Err(ComputeError::validation(
                            Service::VpcApi,
                            Some(&path),
                            format!(
                                "Pagination limit reached. Fetched {} of {} records",
                                all_items.len(),
                                total_count
                            ),
                        ));
                    }
                }

                Ok(all_items)
            }
        }
    }

    pub async fn list_routers(&self) -> Result<Vec<Routers>> {
        let path = format!(
            "/vpc/api/v1/projects/{}/snat-routers",
            self.client.project()
        );
        let response: RoutersResponse = self.get_with_query(&path, &[("entire", "true")]).await?;
        Ok(response.list.into_iter().map(Into::into).collect())
    }

    pub async fn list_route_tables(
        &self,
        limit: Option<u32>,
        page: Option<u32>,
    ) -> Result<Vec<RouteTables>> {
        let path = format!(
            "/vpc/api/v1/projects/{}/network_route_tables",
            self.client.project()
        );

        let this = &self;
        paginate(
            limit,
            page,
            Service::VpcApi,
            &path,
            |page: u32, limit: u32| {
                let path = path.clone();
                async move {
                    let response: RouteTablesResponse = this
                        .get_with_query(
                            &path,
                            &[
                                ("include", "total_count"),
                                ("page", &page.to_string()),
                                ("per_page", &limit.to_string()),
                            ],
                        )
                        .await?;
                    let items: Vec<RouteTables> =
                        response.list.into_iter().map(Into::into).collect();
                    Ok((items, response.meta.total_count))
                }
            },
        )
        .await
    }

    pub async fn get_network(&self, network_id: Uuid) -> Result<Networks> {
        let path = format!(
            "/vpc/api/v1/projects/{}/networks/{}",
            self.client.project(),
            network_id
        );
        let response: super::responses::NetworkWrapper = self.get(&path).await?;
        Ok(response.into())
    }

    pub async fn get_security_group(&self, security_group_id: Uuid) -> Result<SecurityGroups> {
        let path = format!(
            "/api/v1/projects/{}/security-groups/{}",
            self.client.project(),
            security_group_id
        );
        let response: super::responses::SecurityGroupWrapper = self.get(&path).await?;
        Ok(response.into())
    }
}
