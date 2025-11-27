mod cli;
mod codegen;
mod commands;
mod config;
mod error;
mod sui;
mod templates;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use error::Result;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            name,
            template,
            no_git,
        } => {
            commands::init::execute(name, template, no_git).await?;
        }
        Commands::Build { release, watch } => {
            commands::build::execute(release, watch).await?;
        }
        Commands::Test { filter, coverage } => {
            commands::test::execute(filter, coverage).await?;
        }
        Commands::Deploy {
            network,
            gas_budget,
            skip_verify,
        } => {
            commands::deploy::execute(network, gas_budget, skip_verify).await?;
        }
        Commands::Generate { target, output } => {
            commands::generate::execute(target, output).await?;
        }
        Commands::Node { action } => {
            commands::node::execute(action).await?;
        }
        Commands::Install { plugin } => {
            commands::install::execute(plugin).await?;
        }
        Commands::Profile { action, name, rpc } => {
            commands::profile::execute(action, name, rpc).await?;
        }
        Commands::Verify {
            package_id,
            network,
        } => {
            commands::verify::execute(package_id, network).await?;
        }
        Commands::Gas { action, function } => {
            commands::gas::execute(action, function).await?;
        }
        Commands::Scan { level, format } => {
            commands::scan::execute(level, format).await?;
        }
        Commands::Watch { test, deploy } => {
            commands::watch::execute(test, deploy).await?;
        }
        Commands::Dashboard { port } => {
            commands::dashboard::execute(port).await?;
        }
        Commands::Inspect {
            object_id,
            network,
            format,
        } => {
            commands::inspect::execute(object_id, network, format).await?;
        }
        Commands::Coverage { format, output } => {
            commands::coverage::execute(format, output).await?;
        }
    }

    Ok(())
}
