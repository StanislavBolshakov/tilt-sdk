pub mod common;
pub mod compute;
pub mod network;
pub mod routers;
pub mod storage;

#[doc(hidden)]
pub use crate::log_schema_drift;

pub use common::{
    parse_datetime, AvailabilityZone, ListMeta, ListResponse, NestedEntity, PlacementPolicy,
    Regions, SshKeys, StatusEnum, TaskStatus, Tasks,
};

pub use compute::{
    Backup, Backups, ExtraSpec, Flavors, ImageStatus, ImageUpload, ImageUploadStatus, Images,
    InstanceStatus, Instances,
};

pub use network::{
    DhcpDns, DhcpSettings, FloatingIp, FloatingIpInfo, FloatingIps, InstanceItem, IpAddress,
    Network, NetworkInterface, NetworkItem, Networks, NicPort, Ports, RouteTableNetwork,
    RouteTableRegion, RouteTableRoute, RouteTableStatus, RouteTables, SecurityGroup,
    SecurityGroupRule, SecurityGroups, Subnets, VirtualIp, VirtualIpFixedIp, VirtualIps,
};

pub use routers::{RouterNic, RouterStatus, Routers};

pub use storage::{Snapshot, Snapshots, VolumeStatus, VolumeType, Volumes};
