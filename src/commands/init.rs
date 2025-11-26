use crate::config::SuiForgeConfig;
use crate::error::{Result, SuiForgeError};
use crate::templates::Template;
use crate::utils;
use colored::Colorize;
use std::fs;
use std::path::Path;
use std::process::Command;

pub async fn execute(name: String, template: String, no_git: bool) -> Result<()> {
    utils::info(&format!("Creating new SuiForge project: {}", name.bold()));

    let project_path = Path::new(&name);
    if project_path.exists() {
        return Err(SuiForgeError::ProjectExists(name));
    }

    let template = Template::from_str(&template)?;

    // Create project structure
    let spinner = utils::create_spinner("Setting up project structure...");
    create_project_structure(project_path, &name, &template)?;
    spinner.finish_with_message("Project structure created");

    // Initialize git
    if !no_git {
        let spinner = utils::create_spinner("Initializing git repository...");
        init_git(project_path)?;
        spinner.finish_with_message("Git repository initialized");
    }

    utils::success(&format!("Project {} created successfully!", name.green().bold()));
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  suiforge build");
    println!("  suiforge test");
    println!("  suiforge deploy devnet");

    Ok(())
}

fn create_project_structure(path: &Path, name: &str, template: &Template) -> Result<()> {
    // Create directories
    fs::create_dir_all(path)?;
    fs::create_dir_all(path.join("sources"))?;
    fs::create_dir_all(path.join("tests"))?;
    fs::create_dir_all(path.join("scripts"))?;

    // Create Move.toml
    let move_toml = template.get_move_toml(name);
    fs::write(path.join("Move.toml"), move_toml)?;

    // Create main.move
    let main_move = template.get_main_move(name);
    fs::write(path.join("sources").join("main.move"), main_move)?;

    // Create test file
    let test_content = format!(
        r#"#[test_only]
module {}::main_tests {{
    use {}::main;

    #[test]
    fun test_basic() {{
        // Add your tests here
    }}
}}
"#,
        name, name
    );
    fs::write(path.join("tests").join("main_tests.move"), test_content)?;

    // Create suiforge.config.json
    let config = SuiForgeConfig::default();
    config.save(&path.join("suiforge.config.json"))?;

    // Create .gitignore
    let gitignore = r#"build/
suiforge.lock.json
.sui/
node_modules/
*.log
.DS_Store
"#;
    fs::write(path.join(".gitignore"), gitignore)?;

    // Create README
    let readme = format!(
        r#"# {}

A SuiForge project.

## Getting Started

```bash
# Build the project
suiforge build

# Run tests
suiforge test

# Deploy to devnet
suiforge deploy devnet
```

## Project Structure

- `sources/` - Move source files
- `tests/` - Move test files
- `scripts/` - Deployment and utility scripts
- `build/` - Build artifacts (auto-generated)

## Learn More

- [SuiForge Documentation](https://suiforge.dev)
- [Sui Documentation](https://docs.sui.io)
"#,
        name
    );
    fs::write(path.join("README.md"), readme)?;

    Ok(())
}

fn init_git(path: &Path) -> Result<()> {
    Command::new("git")
        .arg("init")
        .current_dir(path)
        .output()?;

    Command::new("git")
        .args(["add", "."])
        .current_dir(path)
        .output()?;

    Command::new("git")
        .args(["commit", "-m", "Initial commit from SuiForge"])
        .current_dir(path)
        .output()?;

    Ok(())
}
