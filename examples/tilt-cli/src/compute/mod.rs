pub mod commands;

use clap::{Args, Subcommand};

use crate::cli::OutputFormat;
use tilt_sdk_cloudengine as cloudengine;

#[derive(Debug, Clone, Subcommand)]
pub enum ServerAction {
    #[command(about = "List compute instances")]
    List {
        #[command(flatten)]
        list_opts: ServerListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct ServerListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Show detailed info")]
    pub long: bool,
    #[arg(long, help = "Limit results (max 100)")]
    pub limit: Option<u32>,
    #[arg(short = 'P', long, help = "Page number")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum FlavorAction {
    #[command(about = "List VM flavors")]
    List {
        #[command(flatten)]
        list_opts: FlavorListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct FlavorListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(short, long, help = "Filter by AZ")]
    pub az: Option<String>,
    #[arg(long, help = "Limit results (max 100)")]
    pub limit: Option<u32>,
    #[arg(short = 'P', long, help = "Page number")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ImageAction {
    #[command(about = "List disk images")]
    List {
        #[command(flatten)]
        list_opts: ImageListOpts,
    },
    #[command(about = "Show image details")]
    Show {
        #[command(flatten)]
        show_opts: ImageShowOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct ImageListOpts {
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
pub struct ImageShowOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    pub image_id: String,
}

#[derive(Debug, Clone, Subcommand)]
pub enum TaskAction {
    #[command(about = "List async tasks")]
    List {
        #[command(flatten)]
        list_opts: TaskListOpts,
    },
    #[command(about = "Show task details")]
    Show {
        #[command(flatten)]
        task_opts: TaskOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct TaskListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    #[arg(long, help = "Show detailed status")]
    pub long: bool,
    #[arg(long, help = "Limit results (max 100)")]
    pub limit: Option<u32>,
    #[arg(short = 'P', long, help = "Page number")]
    pub page: Option<u32>,
}

#[derive(Debug, Clone, Args)]
pub struct TaskOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
    pub task_id: String,
}

#[derive(Debug, Clone, Subcommand)]
pub enum PlacementAction {
    #[command(about = "List placement policies")]
    List {
        #[command(flatten)]
        list_opts: PlacementListOpts,
    },
}

#[derive(Debug, Clone, Args)]
pub struct PlacementListOpts {
    #[arg(short, long, help = "Output format [table]")]
    pub format: Option<OutputFormat>,
}

pub async fn handle_server_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: ServerAction,
    format: Option<OutputFormat>,
) {
    match action {
        ServerAction::List { list_opts } => {
            match commands::list_servers(compute, list_opts.limit, list_opts.page, list_opts.long)
                .await
            {
                Ok(instances) => {
                    let table = commands::format_servers_rows(&instances, list_opts.long);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(instances.len(), "server", "servers")
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&instances).unwrap());
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

pub async fn handle_flavor_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: FlavorAction,
    format: Option<OutputFormat>,
) {
    match action {
        FlavorAction::List { list_opts } => {
            match commands::list_flavors(
                compute,
                list_opts.limit,
                list_opts.page,
                list_opts.az.as_deref(),
            )
            .await
            {
                Ok(items) => {
                    let table = commands::format_flavors_rows(&items);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(items.len(), "flavor", "flavors")
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&items).unwrap());
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

pub async fn handle_image_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: ImageAction,
    format: Option<OutputFormat>,
) {
    match action {
        ImageAction::List { list_opts } => {
            match commands::list_project_images(compute, list_opts.limit, list_opts.page).await {
                Ok(project) => {
                    let table = commands::format_image_rows(&project, list_opts.long);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            println!(
                                "{}",
                                crate::output::format_count(project.len(), "image", "images")
                            );
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&project).unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "tilt-cli", "{}", e);
                    std::process::exit(1);
                }
            }
        }
        ImageAction::Show { show_opts } => {
            match commands::show_image(compute, &show_opts.image_id).await {
                Ok(image) => {
                    let table = commands::format_image_show_row(&image);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => println!("{}", table),
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&image).unwrap())
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

pub async fn handle_task_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: TaskAction,
    format: Option<OutputFormat>,
) {
    match action {
        TaskAction::List { list_opts } => match commands::list_tasks(compute).await {
            Ok(tasks) => {
                let table = commands::format_task_rows(&tasks, list_opts.long);
                match format.unwrap_or(OutputFormat::Table) {
                    OutputFormat::Table => {
                        println!("{}", table);
                        println!(
                            "{}",
                            crate::output::format_count(tasks.len(), "task", "tasks")
                        );
                    }
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&tasks).unwrap());
                    }
                }
            }
            Err(e) => {
                tracing::error!(target: "tilt-cli", "{}", e);
                std::process::exit(1);
            }
        },
        TaskAction::Show { task_opts } => {
            match commands::show_task(compute, &task_opts.task_id).await {
                Ok(task) => {
                    let table = commands::format_task_rows(std::slice::from_ref(&task), false);
                    match format.unwrap_or(OutputFormat::Table) {
                        OutputFormat::Table => {
                            println!("{}", table);
                            if task.status == cloudengine::TaskStatus::Failed {
                                std::process::exit(1);
                            }
                        }
                        OutputFormat::Json => {
                            println!("{}", serde_json::to_string_pretty(&task).unwrap())
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

pub async fn handle_placement_action(
    compute: &cloudengine::ComputeClient<'_>,
    action: PlacementAction,
    format: Option<OutputFormat>,
) {
    match action {
        PlacementAction::List { .. } => match commands::list_placement_policies(compute).await {
            Ok(policies) => {
                let table = commands::format_placement_rows(&policies, false);
                match format.unwrap_or(OutputFormat::Table) {
                    OutputFormat::Table => {
                        println!("{}", table);
                        println!(
                            "{}",
                            crate::output::format_count(policies.len(), "policy", "policies")
                        );
                    }
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&policies).unwrap());
                    }
                }
            }
            Err(e) => {
                tracing::error!(target: "tilt-cli", "{}", e);
                std::process::exit(1);
            }
        },
    }
}
