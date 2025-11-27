use crate::error::Result;
use crate::utils;
use colored::Colorize;
use regex::Regex;
use std::fs;
use std::path::Path;

#[derive(Debug)]
enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    fn color(&self) -> colored::Color {
        match self {
            Severity::Critical => colored::Color::Red,
            Severity::High => colored::Color::BrightRed,
            Severity::Medium => colored::Color::Yellow,
            Severity::Low => colored::Color::Blue,
            Severity::Info => colored::Color::Cyan,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
            Severity::Info => "INFO",
        }
    }
}

#[derive(Debug)]
struct SecurityIssue {
    severity: Severity,
    title: String,
    description: String,
    location: String,
    recommendation: String,
}

pub async fn execute(level: String, format: String) -> Result<()> {
    utils::info(&format!("Running security scan (level: {})...", level.cyan()));

    let spinner = utils::create_spinner("Analyzing Move code...");

    // Scan source files
    let issues = scan_sources("sources", &level)?;

    spinner.finish_with_message(format!("Found {} potential issues", issues.len()));

    if format == "json" {
        print_json_report(&issues)?;
    } else {
        print_text_report(&issues)?;
    }

    Ok(())
}

fn scan_sources(source_dir: &str, level: &str) -> Result<Vec<SecurityIssue>> {
    let mut issues = Vec::new();
    let path = Path::new(source_dir);

    if !path.exists() {
        return Ok(issues);
    }

    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "move"))
    {
        let content = fs::read_to_string(entry.path())?;
        let file_name = entry.path().display().to_string();
        
        // Scan for security issues
        scan_file_for_issues(&content, &file_name, level, &mut issues)?;
    }

    Ok(issues)
}

fn scan_file_for_issues(
    content: &str,
    file_name: &str,
    level: &str,
    issues: &mut Vec<SecurityIssue>,
) -> Result<()> {
    let lines: Vec<&str> = content.lines().collect();

    // Check for unchecked transfers
    for (i, line) in lines.iter().enumerate() {
        if line.contains("transfer::") && !line.contains("assert!") {
            // Look back a few lines for ownership check
            let has_check = lines
                .iter()
                .skip(i.saturating_sub(5))
                .take(5)
                .any(|l| l.contains("assert!") && (l.contains("owner") || l.contains("sender")));

            if !has_check {
                issues.push(SecurityIssue {
                    severity: Severity::High,
                    title: "Unchecked Transfer".to_string(),
                    description: "Transfer operation without ownership verification".to_string(),
                    location: format!("{}:{}", file_name, i + 1),
                    recommendation: "Add ownership check before transfer: assert!(owner == sender)".to_string(),
                });
            }
        }

        // Check for public functions without access control
        if line.contains("public fun") && !line.contains("entry") {
            let function_body_start = i;
            let has_access_control = lines
                .iter()
                .skip(function_body_start)
                .take(10)
                .any(|l| l.contains("assert!") || l.contains("require") || l.contains("capability"));

            if !has_access_control && level != "basic" {
                issues.push(SecurityIssue {
                    severity: Severity::Medium,
                    title: "Missing Access Control".to_string(),
                    description: "Public function without role-based access control".to_string(),
                    location: format!("{}:{}", file_name, i + 1),
                    recommendation: "Implement admin-only modifier or capability pattern".to_string(),
                });
            }
        }

        // Check for potential reentrancy
        if line.contains("transfer::") || line.contains("coin::") {
            // Check if state is updated after external call
            let has_state_update_before = lines
                .iter()
                .skip(i.saturating_sub(3))
                .take(3)
                .any(|l| l.contains("=") && !l.contains("=="));

            if !has_state_update_before {
                issues.push(SecurityIssue {
                    severity: Severity::Critical,
                    title: "Potential Reentrancy Risk".to_string(),
                    description: "External call before state update".to_string(),
                    location: format!("{}:{}", file_name, i + 1),
                    recommendation: "Update state before making external calls (checks-effects-interactions)".to_string(),
                });
            }
        }

        // Strict mode checks
        if level == "strict" {
            // Check for unused variables
            if line.contains("let ") && !line.contains("mut") && !line.contains("_") {
                if let Some(var_name) = extract_variable_name(line) {
                    let is_used = lines
                        .iter()
                        .skip(i + 1)
                        .take(20)
                        .any(|l| l.contains(&var_name));

                    if !is_used {
                        issues.push(SecurityIssue {
                            severity: Severity::Low,
                            title: "Unused Variable".to_string(),
                            description: format!("Variable '{}' declared but never used", var_name),
                            location: format!("{}:{}", file_name, i + 1),
                            recommendation: "Remove unused variable or prefix with underscore".to_string(),
                        });
                    }
                }
            }

            // Check for missing documentation
            if line.contains("public fun") {
                let has_doc = i > 0 && lines[i - 1].trim().starts_with("///");
                if !has_doc {
                    issues.push(SecurityIssue {
                        severity: Severity::Info,
                        title: "Missing Documentation".to_string(),
                        description: "Public function lacks documentation comment".to_string(),
                        location: format!("{}:{}", file_name, i + 1),
                        recommendation: "Add /// documentation comment explaining function purpose".to_string(),
                    });
                }
            }
        }
    }

    Ok(())
}

fn extract_variable_name(line: &str) -> Option<String> {
    let re = Regex::new(r"let\s+(\w+)").ok()?;
    re.captures(line)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

fn print_text_report(issues: &[SecurityIssue]) -> Result<()> {
    println!();
    println!("{}", "🔒 Security Scan Report".bold());
    println!("{}", "═".repeat(70));
    println!();

    // Group by severity
    let mut critical = 0;
    let mut high = 0;
    let mut medium = 0;
    let mut low = 0;
    let mut info = 0;

    for issue in issues {
        match issue.severity {
            Severity::Critical => critical += 1,
            Severity::High => high += 1,
            Severity::Medium => medium += 1,
            Severity::Low => low += 1,
            Severity::Info => info += 1,
        }
    }

    println!("{}", "Summary:".bold());
    if critical > 0 {
        println!("  {} {} critical issues", "🔴".red(), critical);
    }
    if high > 0 {
        println!("  {} {} high severity issues", "🟠".yellow(), high);
    }
    if medium > 0 {
        println!("  {} {} medium severity issues", "🟡".yellow(), medium);
    }
    if low > 0 {
        println!("  {} {} low severity issues", "🔵".blue(), low);
    }
    if info > 0 {
        println!("  {} {} informational", "ℹ️ ".cyan(), info);
    }
    println!();

    // Print detailed issues
    for (i, issue) in issues.iter().enumerate() {
        println!("{}", format!("Issue #{}", i + 1).bold());
        println!(
            "  Severity: {}",
            issue.severity.as_str().color(issue.severity.color()).bold()
        );
        println!("  Title: {}", issue.title.bold());
        println!("  Location: {}", issue.location.dimmed());
        println!("  Description: {}", issue.description);
        println!("  Recommendation: {}", issue.recommendation.green());
        println!();
    }

    if critical > 0 || high > 0 {
        utils::warning("⚠️  Critical or high severity issues found! Please review before deployment.");
    } else {
        utils::success("✓ No critical security issues found");
    }

    Ok(())
}

fn print_json_report(issues: &[SecurityIssue]) -> Result<()> {
    let json_issues: Vec<_> = issues
        .iter()
        .map(|issue| {
            serde_json::json!({
                "severity": issue.severity.as_str(),
                "title": issue.title,
                "description": issue.description,
                "location": issue.location,
                "recommendation": issue.recommendation,
            })
        })
        .collect();

    let report = serde_json::json!({
        "scan_date": chrono::Utc::now().to_rfc3339(),
        "total_issues": issues.len(),
        "issues": json_issues,
    });

    println!("{}", serde_json::to_string_pretty(&report)?);

    Ok(())
}
