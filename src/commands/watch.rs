use crate::commands;
use crate::error::Result;
use crate::utils;
use colored::Colorize;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub async fn execute(test: bool, deploy: bool) -> Result<()> {
    utils::info("Starting watch mode...");
    println!("  Watching: {}", "sources/".cyan());
    if test {
        println!("  Auto-test: {}", "enabled".green());
    }
    if deploy {
        println!("  Auto-deploy: {}", "enabled".yellow());
    }
    println!();
    println!("Press {} to stop", "Ctrl+C".bold());
    println!();

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: std::result::Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default(),
    )
    .map_err(|e| crate::error::SuiForgeError::Custom(e.to_string()))?;

    watcher
        .watch(Path::new("sources"), RecursiveMode::Recursive)
        .map_err(|e| crate::error::SuiForgeError::Custom(e.to_string()))?;

    let mut last_build = std::time::Instant::now();
    let debounce_duration = Duration::from_millis(500);

    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => {
                if event.kind.is_modify() {
                    let now = std::time::Instant::now();
                    if now.duration_since(last_build) > debounce_duration {
                        last_build = now;

                        println!();
                        utils::info(&format!(
                            "Change detected at {}",
                            chrono::Local::now().format("%H:%M:%S")
                        ));

                        // Build
                        println!();
                        if let Err(e) = commands::build::execute(false, false).await {
                            utils::error(&format!("Build failed: {}", e));
                            continue;
                        }

                        // Test if enabled
                        if test {
                            println!();
                            if let Err(e) = commands::test::execute(None, false).await {
                                utils::error(&format!("Tests failed: {}", e));
                                continue;
                            }
                        }

                        // Deploy if enabled
                        if deploy {
                            println!();
                            utils::warning("Auto-deploy is enabled but requires manual confirmation");
                            println!("Run: suiforge deploy devnet");
                        }

                        println!();
                        utils::success("✓ Ready for changes");
                        println!();
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // Normal timeout, continue watching
                continue;
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }
    }

    Ok(())
}
