use crate::codegen;
use crate::config::{find_project_root, load_config};
use crate::error::{Result, SuiForgeError};
use crate::utils;
use colored::Colorize;

pub async fn execute(target: String, output: Option<String>) -> Result<()> {
    let root = find_project_root()?;
    let config = load_config()?;

    let output_dir = output.unwrap_or_else(|| {
        match target.as_str() {
            "ts" => config.codegen.typescript.as_ref().map(|c| c.output_dir.clone()),
            "rust" => config.codegen.rust.as_ref().map(|c| c.output_dir.clone()),
            "swift" => config.codegen.swift.as_ref().map(|c| c.output_dir.clone()),
            _ => None,
        }
        .unwrap_or_else(|| format!("./sdk/{}", target))
    });

    utils::info(&format!(
        "Generating {} SDK to {}...",
        target.cyan().bold(),
        output_dir.yellow()
    ));

    let spinner = utils::create_spinner("Analyzing Move modules...");

    match target.as_str() {
        "ts" | "typescript" => {
            codegen::typescript::generate(&root, &output_dir)?;
            spinner.finish_with_message("TypeScript SDK generated");
        }
        "rust" => {
            spinner.finish_with_message("Rust SDK generation not yet implemented");
            utils::warning("Rust SDK generation coming soon!");
        }
        "swift" => {
            spinner.finish_with_message("Swift SDK generation not yet implemented");
            utils::warning("Swift SDK generation coming soon!");
        }
        "python" => {
            spinner.finish_with_message("Python SDK generation not yet implemented");
            utils::warning("Python SDK generation coming soon!");
        }
        _ => {
            return Err(SuiForgeError::CodegenFailed(format!(
                "Unknown target: {}",
                target
            )));
        }
    }

    utils::success(&format!("SDK generated at {}", output_dir.green()));

    Ok(())
}
