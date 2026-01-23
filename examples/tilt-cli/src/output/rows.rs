use cloudengine::models::NetworkItem;
use comfy_table::Cell;
use tilt_sdk_cloudengine as cloudengine;

pub trait ToStringRow {
    fn to_headers(&self) -> Vec<String>;
    fn to_row(&self) -> Vec<Cell>;
}

macro_rules! impl_table_row {
    ($struct_name:ident, $($field:ident),+) => {
        impl ToStringRow for $struct_name {
            fn to_headers(&self) -> Vec<String> {
                vec![$(stringify!($field).to_string()),+]
            }
            fn to_row(&self) -> Vec<Cell> {
                vec![$(Cell::new(&self.$field.to_string())),+]
            }
        }
    };
}

#[derive(Debug)]
pub struct NetworkRow {
    pub id: String,
    pub name: String,
    pub status: String,
    pub description: String,
    pub created: String,
}

impl_table_row!(NetworkRow, id, name, status, description, created);

#[derive(Debug)]
pub struct NetworkRowLong {
    pub id: String,
    pub name: String,
    pub status: String,
    pub dhcp: String,
    pub description: String,
    pub shared: String,
    pub created: String,
}

impl_table_row!(
    NetworkRowLong,
    id,
    name,
    status,
    dhcp,
    description,
    shared,
    created
);

#[derive(Debug)]
pub struct SubnetRowLong {
    pub id: String,
    pub name: String,
    pub ipam: String,
    pub network: String,
    pub region: String,
    pub status: String,
    pub dhcp: String,
    pub description: String,
    pub shared: String,
}

impl_table_row!(
    SubnetRowLong,
    id,
    name,
    ipam,
    network,
    region,
    status,
    dhcp,
    description,
    shared
);

#[derive(Debug)]
pub struct SshKeyRow {
    pub id: String,
    pub name: String,
    pub login: String,
    pub created: String,
}

impl_table_row!(SshKeyRow, id, name, login, created);

#[derive(Debug)]
pub struct SecurityGroupRow {
    pub id: String,
    pub name: String,
    pub status: String,
    pub description: String,
    pub created: String,
}

impl_table_row!(SecurityGroupRow, id, name, status, description, created);

#[derive(Debug)]
pub struct SecurityGroupRowLong {
    pub id: String,
    pub name: String,
    pub status: String,
    pub description: String,
    pub created: String,
    pub updated: String,
}

impl_table_row!(
    SecurityGroupRowLong,
    id,
    name,
    status,
    description,
    created,
    updated
);

#[derive(Debug)]
pub struct SubnetRow {
    pub id: String,
    pub name: String,
    pub ipam: String,
    pub network: String,
    pub status: String,
}

impl_table_row!(SubnetRow, id, name, ipam, network, status);

#[derive(Debug)]
pub struct NetworkItemRow {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub status: String,
    pub ip: String,
    pub network: String,
    pub az: String,
}

impl_table_row!(NetworkItemRow, id, name, type_, status, ip, network, az);

#[derive(Debug)]
pub struct NicRow {
    pub id: String,
    pub status: String,
    pub ip: String,
    pub network: String,
    pub security_groups: String,
    pub state: String,
}

impl_table_row!(NicRow, id, status, ip, network, security_groups, state);

#[derive(Debug)]
pub struct InstanceRow {
    pub id: String,
    pub name: String,
    pub status: String,
    pub flavor: String,
    pub image: String,
    pub ip: String,
    pub network: String,
    pub availability_zone: String,
}

impl_table_row!(
    InstanceRow,
    id,
    name,
    status,
    flavor,
    image,
    ip,
    network,
    availability_zone
);

impl From<&NetworkItem> for NicRow {
    fn from(item: &NetworkItem) -> Self {
        match item {
            NetworkItem::Nic(nic) => NicRow {
                id: nic.id.to_string(),
                status: nic.state.clone(),
                ip: nic.ip_address.clone().unwrap_or_else(|| "-".to_string()),
                network: nic.network_name.clone(),
                security_groups: if nic.security_group_names.is_empty() {
                    "-".to_string()
                } else {
                    nic.security_group_names.join(", ")
                },
                state: nic.state.clone(),
            },
            NetworkItem::Instance(_) => panic!("Cannot convert Instance to NicRow"),
        }
    }
}

impl From<&NetworkItem> for InstanceRow {
    fn from(item: &NetworkItem) -> Self {
        match item {
            NetworkItem::Instance(inst) => InstanceRow {
                id: inst.id.to_string(),
                name: inst.name.clone(),
                status: inst.status.clone(),
                flavor: inst.flavor_name.clone(),
                image: inst.image_name.clone(),
                ip: inst.ip_address.clone().unwrap_or_else(|| "-".to_string()),
                network: inst.network_name.clone().unwrap_or_else(|| "-".to_string()),
                availability_zone: inst.availability_zone.clone(),
            },
            NetworkItem::Nic(_) => panic!("Cannot convert Nic to InstanceRow"),
        }
    }
}

impl From<&NetworkItem> for NetworkItemRow {
    fn from(item: &NetworkItem) -> Self {
        match item {
            NetworkItem::Nic(nic) => NetworkItemRow {
                id: nic.id.to_string(),
                name: nic.id.to_string(),
                type_: "nic".to_string(),
                status: nic.state.clone(),
                ip: nic.ip_address.clone().unwrap_or_else(|| "-".to_string()),
                network: nic.network_name.clone(),
                az: nic
                    .availability_zone
                    .clone()
                    .unwrap_or_else(|| "-".to_string()),
            },
            NetworkItem::Instance(inst) => NetworkItemRow {
                id: inst.id.to_string(),
                name: inst.name.clone(),
                type_: "instance".to_string(),
                status: inst.status.clone(),
                ip: inst.ip_address.clone().unwrap_or_else(|| "-".to_string()),
                network: inst.network_name.clone().unwrap_or_else(|| "-".to_string()),
                az: inst.availability_zone.clone(),
            },
        }
    }
}

#[derive(Debug)]
pub struct PortTreeRow {
    pub instance: String,
    pub instance_id: String,
    pub instance_status: String,
    pub network: String,
    pub az: String,
    pub attached_nics: String,
}

impl_table_row!(
    PortTreeRow,
    instance,
    instance_id,
    instance_status,
    network,
    az,
    attached_nics
);

#[derive(Debug)]
pub struct RouterRow {
    pub id: String,
    pub name: String,
    pub status: String,
    pub bandwidth: String,
    pub ip: String,
}

impl_table_row!(RouterRow, id, name, status, bandwidth, ip);

#[derive(Debug)]
pub struct RouterRowLong {
    pub id: String,
    pub name: String,
    pub status: String,
    pub bandwidth: String,
    pub ip: String,
    pub attached_nics: String,
    pub created_at: String,
}

impl_table_row!(
    RouterRowLong,
    id,
    name,
    status,
    bandwidth,
    ip,
    attached_nics,
    created_at
);

#[derive(Debug)]
pub struct RouteTableRow {
    pub id: String,
    pub name: String,
    pub status: String,
    pub routes: String,
    pub networks: String,
    pub region: String,
    pub created: String,
}

impl_table_row!(
    RouteTableRow,
    id,
    name,
    status,
    routes,
    networks,
    region,
    created
);

#[derive(Debug)]
pub struct SecurityGroupRuleRow {
    pub id: String,
    pub direction: String,
    pub protocol: String,
    pub ports: String,
    pub remote_ip: String,
    pub ethertype: String,
}

impl_table_row!(
    SecurityGroupRuleRow,
    id,
    direction,
    protocol,
    ports,
    remote_ip,
    ethertype
);

#[derive(Debug)]
pub struct VipRow {
    pub id: String,
    pub name: String,
    pub status: String,
    pub floating_ip: String,
    pub fixed_ip: String,
    pub address_mode: String,
}

impl_table_row!(
    VipRow,
    id,
    name,
    status,
    floating_ip,
    fixed_ip,
    address_mode
);

#[derive(Debug)]
pub struct VipRowLong {
    pub id: String,
    pub name: String,
    pub status: String,
    pub mac_address: String,
    pub floating_ip: String,
    pub fixed_ip: String,
    pub l2_enabled: String,
    pub address_mode: String,
    pub created: String,
}

impl_table_row!(
    VipRowLong,
    id,
    name,
    status,
    mac_address,
    floating_ip,
    fixed_ip,
    l2_enabled,
    address_mode,
    created
);

#[derive(Debug)]
pub struct FipRow {
    pub id: String,
    pub ip_address: String,
    pub bandwidth: String,
    pub status: String,
    pub created: String,
}

impl_table_row!(FipRow, id, ip_address, bandwidth, status, created);

#[derive(Debug)]
pub struct FipRowLong {
    pub id: String,
    pub ip_address: String,
    pub bandwidth: String,
    pub status: String,
    pub parent: String,
    pub created: String,
}

impl_table_row!(FipRowLong, id, ip_address, bandwidth, status, parent, created);
