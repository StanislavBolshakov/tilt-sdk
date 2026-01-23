pub mod common;
pub mod compute;
pub mod network_tree;
pub mod router_tree;
pub mod storage;
pub mod table;
pub mod vpc;

pub use common::{format_bytes, format_count, format_date, format_opt, format_opt_ref};
pub use compute::{
    FlavorsRow, ImageRow, ImageRowLong, ImageShowRow, PlacementRow, RegionRow, ServersRow,
    ServersRowLong, TaskRow, TaskRowDetail,
};
pub use network_tree::format_port_tree;
pub use router_tree::format_router_tree;
pub use storage::{BackupRow, BackupRowLong, SnapshotRow, VolumeRow, VolumeRowLong, VolumeShowRow};
pub use table::{format_table, ToStringRow};
pub use vpc::{
    FipRow, FipRowLong, InstanceRow, NetworkItemRow, NetworkRow, NetworkRowLong, NicRow,
    RouteTableRow, RouterRow, SecurityGroupRow, SecurityGroupRowLong, SecurityGroupRuleRow,
    SshKeyRow, SubnetRow, SubnetRowLong, VipRow, VipRowLong,
};

#[macro_export]
macro_rules! impl_table_row {
    ($struct_name:ident, $($field:ident),+) => {
        impl $crate::output::ToStringRow for $struct_name {
            fn to_headers(&self) -> Vec<String> {
                vec![
                    $(stringify!($field).to_uppercase()),+
                ]
            }

            fn to_row(&self) -> Vec<comfy_table::Cell> {
                vec![
                    $(comfy_table::Cell::new(&self.$field)),+
                ]
            }
        }
    };
}
