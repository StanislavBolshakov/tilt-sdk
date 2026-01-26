pub mod commands;

use clap::{Args, Subcommand};

use crate::cli::OutputFormat;
use tilt_sdk_cloudengine as cloudengine;

#[derive(Debug, Clone, Subcommand)]
pub enum SshKeyAction {
    #[command(about = "List SSH keys")]
    List {
        #[command(flatten)]
        list_opts: SshKeyListOpts,
    },
    #[command(about = "Create an SSH key")]
    Create {
        #[command(flatten)]
        create_opts: SshKeyCreateOpts,
    },
    #[command(about = "Delete an SSH key")]
    Delete {
        #[command(flatten)]
        delete_opts: SshKeyDeleteOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct SshKeyListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Limit results (max 100)")]
    pub limit: Option<u32>,
    #[arg(short = 'P', long, help = "Page number")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, Args)]
pub struct SshKeyCreateOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(short, long, help = "SSH key name")]
    pub name: String,
    #[arg(short, long, help = "SSH login user")]
    pub login: String,
    #[arg(long, required = true, help = "Public SSH keys (can be specified multiple times)")]
    pub public_key: Vec<String>,
}

#[derive(Debug, Clone, Args)]
pub struct SshKeyDeleteOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    pub id: String,
}

pub async fn handle_ssh_key_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: SshKeyAction,
    format: Option<OutputFormat>,
) {
    match action {
        SshKeyAction::List { list_opts } => {
            match commands::list_ssh_keys(compute, list_opts.limit, list_opts.page).await {
                Ok(keys) => {
                    let table = commands::format_ssh_key_rows(&keys);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(keys.len(), "SSH key", "SSH keys")
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&keys).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
        SshKeyAction::Create { create_opts } => {
            match commands::create_ssh_key(
                compute,
                create_opts.name,
                create_opts.login,
                create_opts.public_key,
            )
            .await
            {
                Ok(key) => {
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("SSH key created successfully:");
                            println!("{}", commands::format_ssh_key_rows(&[key]));
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&key).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
        SshKeyAction::Delete { delete_opts } => {
            match commands::delete_ssh_key(compute, &delete_opts.id).await {
                Ok(json) => {
                    match delete_opts.format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("SSH key {} deleted successfully", delete_opts.id);
                        }
                        OutputFormat::Json => {
                            println!("{}", json);
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
