use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", "ℹ".blue().bold(), msg);
}

pub fn warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg);
}

pub fn error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg);
}

pub fn create_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

pub fn create_progress_bar(len: u64, msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb.set_message(msg.to_string());
    pb
}
