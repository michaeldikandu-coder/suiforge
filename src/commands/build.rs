use crate::config::find_project_root;
use crate::error::{Result, SuiForgeError};
use crate::sui::SuiCli;
use crate::utils;

pub async fn execute(release: bool, watch: bool) -> Result<()> {
    let _root = find_project_root()?;

    if watch {
        utils::info("Watch mode not yet implemented. Building once...");
    }

    utils::info(&format!(
        "Building Move contracts{}...",
        if release { " (release mode)" } else { "" }
    ));

    let spinner = utils::create_spinner("Compiling Move code...");

    let output = SuiCli::build(release)?;

    if output.status.success() {
        spinner.finish_with_message("Build completed");
        utils::success("Move contracts built successfully!");

        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            println!("\n{}", stdout);
        }
    } else {
        spinner.finish_with_message("Build failed");
        let stderr = String::from_utf8_lossy(&output.stderr);
        utils::error("Build failed with errors:");
        println!("\n{}", stderr);
        return Err(SuiForgeError::BuildFailed(stderr.to_string()));
    }

    Ok(())
}
