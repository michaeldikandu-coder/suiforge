use crate::error::{Result, SuiForgeError};
use crate::utils;
use colored::Colorize;
use std::process::Command;

pub async fn execute(action: String) -> Result<()> {
    match action.as_str() {
        "start" => start_node().await,
        "stop" => stop_node().await,
        "status" => check_status().await,
        _ => Err(SuiForgeError::Custom(format!(
            "Unknown action: {}. Use start, stop, or status",
            action
        ))),
    }
}

async fn start_node() -> Result<()> {
    utils::info("Starting local Sui node...");

    let spinner = utils::create_spinner("Initializing node...");

    let output = Command::new("sui").args(["start", "--with-faucet"]).spawn();

    match output {
        Ok(_) => {
            spinner.finish_with_message("Node started");
            utils::success("Local Sui node is running!");
            println!("\n{}", "Node Details:".bold());
            println!("  RPC: {}", "http://127.0.0.1:9000".cyan());
            println!("  Faucet: {}", "http://127.0.0.1:9123".cyan());
            println!("\nTo stop the node, run: {}", "suiforge node stop".yellow());
        }
        Err(e) => {
            spinner.finish_with_message("Failed to start node");
            return Err(SuiForgeError::Custom(format!(
                "Failed to start node: {}",
                e
            )));
        }
    }

    Ok(())
}

async fn stop_node() -> Result<()> {
    utils::info("Stopping local Sui node...");

    // This is simplified - actual implementation would track the process
    utils::warning("Node management not fully implemented yet.");
    utils::info("Please manually stop the Sui node process.");

    Ok(())
}

async fn check_status() -> Result<()> {
    utils::info("Checking node status...");

    // Check if node is running by trying to connect
    let result = reqwest::get("http://127.0.0.1:9000").await;

    match result {
        Ok(_) => {
            utils::success("Local Sui node is running");
            println!("  RPC: {}", "http://127.0.0.1:9000".cyan());
        }
        Err(_) => {
            utils::warning("Local Sui node is not running");
            println!("Start it with: {}", "suiforge node start".yellow());
        }
    }

    Ok(())
}
