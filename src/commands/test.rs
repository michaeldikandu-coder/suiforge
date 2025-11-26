use crate::config::find_project_root;
use crate::error::{Result, SuiForgeError};
use crate::sui::SuiCli;
use crate::utils;

pub async fn execute(filter: Option<String>, coverage: bool) -> Result<()> {
    let _root = find_project_root()?;

    if coverage {
        utils::warning("Coverage mode not yet implemented. Running tests without coverage...");
    }

    utils::info("Running Move tests...");

    let spinner = utils::create_spinner("Executing tests...");

    let output = SuiCli::test(filter)?;

    if output.status.success() {
        spinner.finish_with_message("Tests completed");
        utils::success("All tests passed!");

        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            println!("\n{}", stdout);
        }
    } else {
        spinner.finish_with_message("Tests failed");
        let stderr = String::from_utf8_lossy(&output.stderr);
        utils::error("Tests failed:");
        println!("\n{}", stderr);
        return Err(SuiForgeError::TestFailed(stderr.to_string()));
    }

    Ok(())
}
