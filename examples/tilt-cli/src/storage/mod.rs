pub mod commands;

use clap::{Args, Subcommand};

use crate::cli::OutputFormat;
use tilt_sdk_cloudengine as cloudengine;

#[derive(Debug, Clone, Subcommand)]
pub enum VolumeAction {
    #[command(about = "List volumes")]
    List {
        #[command(flatten)]
        list_opts: VolumeListOpts,
    },
    #[command(about = "Show volume details")]
    Show {
        #[command(flatten)]
        show_opts: VolumeShowOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct VolumeListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Show detailed info")]
    pub long: bool,
    #[arg(long, help = "Limit results (max 100)")]
    pub limit: Option<u32>,
    #[arg(short = 'P', long, help = "Page number")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, Args)]
pub struct VolumeShowOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    pub volume_id: String,
}

#[derive(Debug, Clone, Subcommand)]
pub enum SnapshotAction {
    #[command(about = "List snapshots")]
    List {
        #[command(flatten)]
        list_opts: SnapshotListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct SnapshotListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Limit results (max 100)")]
    pub limit: Option<u32>,
    #[arg(short = 'P', long, help = "Page number")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum BackupAction {
    #[command(about = "List backups")]
    List {
        #[command(flatten)]
        list_opts: BackupListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct BackupListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Show detailed info")]
    pub long: bool,
    #[arg(long, help = "Limit results (max 100)")]
    pub limit: Option<u32>,
    #[arg(short = 'P', long, help = "Page number")]
    pub page: Option<u32>,
}

pub async fn handle_volume_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: VolumeAction,
    format: Option<OutputFormat>,
) {
    match action {
        VolumeAction::List { list_opts } => {
            match commands::list_volumes(compute, list_opts.limit, list_opts.page).await {
                Ok(volumes) => {
                    let table = commands::format_volume_rows(&volumes, list_opts.long);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(volumes.len(), "volume", "volumes")
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&volumes).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
        VolumeAction::Show { show_opts } => {
            match commands::show_volume(compute, &show_opts.volume_id).await {
                Ok(volume) => {
                    let table = commands::format_volume_show_row(&volume);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => println!("{}", table),
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&volume).unwrap())
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

pub async fn handle_snapshot_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: SnapshotAction,
    format: Option<OutputFormat>,
) {
    match action {
        SnapshotAction::List { list_opts } => {
            match commands::list_snapshots(compute, list_opts.limit, list_opts.page).await {
                Ok(snapshots) => {
                    let table = commands::format_snapshot_rows(&snapshots);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(
                                    snapshots.len(),
                                    "snapshot",
                                    "snapshots"
                                )
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&snapshots).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

pub async fn handle_backup_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: BackupAction,
    format: Option<OutputFormat>,
) {
    match action {
        BackupAction::List { list_opts } => {
            match commands::list_backups(compute, list_opts.limit, list_opts.page).await {
                Ok(backups) => {
                    let table = commands::format_backup_rows(&backups, list_opts.long);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(backups.len(), "backup", "backups")
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&backups).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
