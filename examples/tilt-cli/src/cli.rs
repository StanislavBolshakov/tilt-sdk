use clap::{Parser, Subcommand, ValueEnum};

pub use crate::compute::{FlavorAction, ImageAction, PlacementAction, ServerAction, TaskAction};
pub use crate::storage::{BackupAction, SnapshotAction, VolumeAction};
pub use crate::vpc::{
    FipAction, NetworkAction, PortAction, RegionAction, RouteTableAction, RouterAction,
    SecurityGroupAction, SshKeyAction, SubnetAction, VipAction,
};

#[derive(Debug, Clone, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Powershell,
}

#[derive(Parser)]
#[command(name = "tilt-cli")]
#[command(author = "T1LT T1 Cloud <st.bolshakov@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "T1LT T1 Cloud CLI")]
#[command(long_about = "T1LT T1 Cloud CLI

Manage compute instances, volumes, networks, and more.

Authentication:
  Set TILT_TOKEN environment variable or use --token flag
  Set TILT_PROJECT environment variable or use --project flag

Examples:
  tilt-cli server list                          # List compute instances
  tilt-cli volume list --long                   # Detailed volume list
  tilt-cli network list --limit 10              # List 10 networks
  tilt-cli --debug server list                  # Debug mode")]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOpts,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, clap::Args)]
pub struct GlobalOpts {
    #[arg(
        short,
        long,
        help = "API authentication token (or set TILT_TOKEN env var)"
    )]
    pub token: Option<String>,

    #[arg(short, long, help = "Project ID (or set TILT_PROJECT env var)")]
    pub project: Option<String>,

    #[arg(short = 'U', long, help = "API base URL (or set TILT_URL env var)")]
    pub url: Option<String>,

    #[arg(short, long, help = "Enable debug logging")]
    pub debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(about = "Compute instance operations")]
    Server {
        #[command(subcommand)]
        action: ServerAction,
    },
    #[command(about = "VM flavor/instance type operations")]
    Flavor {
        #[command(subcommand)]
        action: FlavorAction,
    },
    #[command(about = "Disk image operations")]
    Image {
        #[command(subcommand)]
        action: ImageAction,
    },
    #[command(about = "Virtual network operations")]
    Network {
        #[command(subcommand)]
        action: NetworkAction,
    },
    #[command(about = "Network subnet operations")]
    Subnet {
        #[command(subcommand)]
        action: SubnetAction,
    },
    #[command(about = "Network port/interface operations")]
    Port {
        #[command(subcommand)]
        action: PortAction,
    },
    #[command(about = "Block storage volume operations")]
    Volume {
        #[command(subcommand)]
        action: VolumeAction,
    },
    #[command(about = "Volume snapshot operations")]
    Snapshot {
        #[command(subcommand)]
        action: SnapshotAction,
    },
    #[command(about = "Async task/operation status")]
    Task {
        #[command(subcommand)]
        action: TaskAction,
    },
    #[command(about = "SSH public key operations")]
    SshKey {
        #[command(subcommand)]
        action: SshKeyAction,
    },
    #[command(about = "Firewall rule operations")]
    SecurityGroup {
        #[command(subcommand)]
        action: SecurityGroupAction,
    },
    #[command(about = "Region/availability zone operations")]
    Region {
        #[command(subcommand)]
        action: RegionAction,
    },
    #[command(about = "Availability zone operations")]
    Az {
        #[command(subcommand)]
        action: RegionAction,
    },
    #[command(about = "Placement policy operations")]
    Placement {
        #[command(subcommand)]
        action: PlacementAction,
    },
    #[command(about = "Backup operations")]
    Backup {
        #[command(subcommand)]
        action: BackupAction,
    },
    #[command(about = "Router operations (SNAT or network)")]
    Router {
        #[command(subcommand)]
        action: RouterAction,
    },
    #[command(about = "Route table operations")]
    RouteTable {
        #[command(subcommand)]
        action: RouteTableAction,
    },
    #[command(about = "Virtual IP operations")]
    Vip {
        #[command(subcommand)]
        action: VipAction,
    },
    #[command(about = "Floating IP operations")]
    Fip {
        #[command(subcommand)]
        action: FipAction,
    },
    #[command(about = "Generate shell completion scripts")]
    Completions {
        #[arg(value_enum)]
        shell: Shell,
        #[arg(long, help = "Install completions to shell profile")]
        install: bool,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}
