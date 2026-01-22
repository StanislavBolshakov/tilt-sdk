#[macro_use]
pub mod client;
pub mod error;
#[macro_use]
pub mod models;
pub mod operations;

pub use client::{ComputeClient, Resource};
pub use error::{ComputeError, Result, Service};
pub use models::{
    AvailabilityZone, ExtraSpec, Flavors, ImageStatus, ImageUpload, ImageUploadStatus, Images,
    InstanceStatus, Instances, ListMeta, ListResponse, NestedEntity, Network, NetworkInterface,
    Networks, PlacementPolicy, Ports, Regions, SecurityGroup, SecurityGroupRule, SecurityGroups,
    Snapshot, Snapshots, SshKeys, StatusEnum, Subnets, TaskStatus, Tasks, VolumeStatus, VolumeType,
    Volumes, parse_datetime,
};
