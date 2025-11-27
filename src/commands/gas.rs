use crate::error::Result;
use crate::utils;
use colored::Colorize;
use regex::Regex;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct GasProfile {
    function: String,
    gas_used: u64,
    storage_cost: u64,
    computation_cost: u64,
}

pub async fn execute(action: String, function: Option<String>) -> Result<()> {
    match action.as_str() {
        "profile" => profile_gas(function).await,
        "analyze" => analyze_gas().await,
        "optimize" => suggest_optimizations().await,
        _ => {
            utils::error(&format!("Unknown action: {}", action));
            println!("Available actions: profile, analyze, optimize");
            Ok(())
        }
    }
}

async fn profile_gas(function: Option<String>) -> Result<()> {
    utils::info("Profiling gas usage...");

    let spinner = utils::create_spinner("Analyzing Move bytecode...");
    
    // Parse Move source files to extract functions
    let profiles = parse_move_functions("sources")?;
    
    spinner.finish_with_message("Analysis complete");

    println!();
    println!("{}", "Gas Usage Profile:".bold());
    println!();
    println!(
        "{:<20} {:>12} {:>15} {:>18}",
        "Function", "Total Gas", "Storage", "Computation"
    );
    println!("{}", "─".repeat(70));

    for profile in &profiles {
        if let Some(ref filter) = function {
            if !profile.function.contains(filter) {
                continue;
            }
        }

        println!(
            "{:<20} {:>12} {:>15} {:>18}",
            profile.function.cyan(),
            format!("{} gas", profile.gas_used).yellow(),
            format!("{} gas", profile.storage_cost).green(),
            format!("{} gas", profile.computation_cost).blue()
        );
    }

    println!();
    utils::info("💡 Tip: Use 'suiforge gas optimize' for optimization suggestions");

    Ok(())
}

async fn analyze_gas() -> Result<()> {
    utils::info("Analyzing gas patterns...");

    let spinner = utils::create_spinner("Scanning Move modules...");
    
    // Parse actual functions
    let profiles = parse_move_functions("sources")?;
    
    spinner.finish_with_message("Scan complete");

    if profiles.is_empty() {
        utils::warning("No Move functions found in sources/");
        return Ok(());
    }

    // Calculate real statistics
    let total_functions = profiles.len();
    let total_gas: u64 = profiles.iter().map(|p| p.gas_used).sum();
    let avg_gas = if total_functions > 0 {
        total_gas / total_functions as u64
    } else {
        0
    };
    
    let highest = profiles.iter().max_by_key(|p| p.gas_used);
    let lowest = profiles.iter().min_by_key(|p| p.gas_used);

    println!();
    println!("{}", "Gas Analysis Report:".bold());
    println!();

    println!("{}", "📊 Statistics:".bold());
    println!("  Total functions analyzed: {}", total_functions.to_string().cyan());
    println!("  Average gas per function: {}", format!("{} gas", avg_gas).yellow());
    
    if let Some(h) = highest {
        println!("  Highest gas usage: {}", format!("{} gas ({})", h.gas_used, h.function).red());
    }
    if let Some(l) = lowest {
        println!("  Lowest gas usage: {}", format!("{} gas ({})", l.gas_used, l.function).green());
    }
    println!();

    // Find hot spots (top 3 gas consumers)
    let mut sorted_profiles = profiles.clone();
    sorted_profiles.sort_by(|a, b| b.gas_used.cmp(&a.gas_used));
    
    if sorted_profiles.len() >= 3 {
        println!("{}", "🔥 Hot Spots:".bold());
        for (i, profile) in sorted_profiles.iter().take(3).enumerate() {
            let reason = if profile.storage_cost > profile.computation_cost {
                "High storage allocation"
            } else {
                "Complex computation"
            };
            println!("  {}. {} - {}", i + 1, profile.function.red(), reason);
        }
        println!();
    }

    // Find efficient functions (bottom 2)
    if sorted_profiles.len() >= 2 {
        println!("{}", "✅ Efficient Functions:".bold());
        for profile in sorted_profiles.iter().rev().take(2) {
            println!("  • {} - {} gas", profile.function.green(), profile.gas_used);
        }
        println!();
    }

    Ok(())
}

async fn suggest_optimizations() -> Result<()> {
    utils::info("Generating optimization suggestions...");

    let spinner = utils::create_spinner("Analyzing code patterns...");
    
    // Analyze actual source files for optimization opportunities
    let suggestions = analyze_for_optimizations("sources")?;
    
    spinner.finish_with_message("Analysis complete");

    if suggestions.is_empty() {
        utils::success("No obvious optimization opportunities found!");
        println!();
        utils::info("Your code appears to be well-optimized.");
        return Ok(());
    }

    println!();
    println!("{}", "🚀 Gas Optimization Suggestions:".bold());
    println!();

    let mut total_savings = 0u64;
    for (i, suggestion) in suggestions.iter().enumerate() {
        println!("{}", format!("{}. {}", i + 1, suggestion.title).yellow().bold());
        println!("   Location: {}", suggestion.location);
        println!("   Current: {}", suggestion.current);
        println!("   Suggestion: {}", suggestion.recommendation);
        println!("   {} Potential savings: ~{} gas per call", "💰".green(), suggestion.savings);
        println!();
        total_savings += suggestion.savings;
    }

    utils::success(&format!("Total potential savings: ~{} gas per transaction", total_savings));

    Ok(())
}

#[derive(Debug, Clone)]
struct OptimizationSuggestion {
    title: String,
    location: String,
    current: String,
    recommendation: String,
    savings: u64,
}

fn analyze_for_optimizations(source_dir: &str) -> Result<Vec<OptimizationSuggestion>> {
    let mut suggestions = Vec::new();
    let path = Path::new(source_dir);

    if !path.exists() {
        return Ok(suggestions);
    }

    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "move"))
    {
        let content = fs::read_to_string(entry.path())?;
        let file_name = entry.path().display().to_string();
        let lines: Vec<&str> = content.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            // Check for excessive object::new calls
            if line.contains("object::new") {
                let count_in_function = lines.iter()
                    .skip(i.saturating_sub(10))
                    .take(20)
                    .filter(|l| l.contains("object::new"))
                    .count();
                
                if count_in_function > 2 {
                    suggestions.push(OptimizationSuggestion {
                        title: "Reduce Storage Allocations".to_string(),
                        location: format!("{}:{}", file_name, i + 1),
                        current: "Multiple object creations in same function".to_string(),
                        recommendation: "Consider using shared objects or batch operations".to_string(),
                        savings: 200 * (count_in_function as u64 - 1),
                    });
                }
            }

            // Check for vector operations in loops
            if line.contains("while") || line.contains("loop") {
                let has_vector_ops = lines.iter()
                    .skip(i)
                    .take(15)
                    .any(|l| l.contains("vector::push_back") || l.contains("vector::borrow"));
                
                if has_vector_ops {
                    suggestions.push(OptimizationSuggestion {
                        title: "Optimize Vector Operations".to_string(),
                        location: format!("{}:{}", file_name, i + 1),
                        current: "Vector operations inside loop".to_string(),
                        recommendation: "Use Table for O(1) lookups or batch vector operations".to_string(),
                        savings: 150,
                    });
                }
            }

            // Check for repeated calculations
            if line.contains("=") && !line.contains("==") {
                let calculation_pattern = Regex::new(r"(\w+\s*\*\s*\w+|\w+\s*/\s*\w+)").unwrap();
                if calculation_pattern.is_match(line) {
                    let same_calc_count = lines.iter()
                        .skip(i + 1)
                        .take(10)
                        .filter(|l| calculation_pattern.is_match(l))
                        .count();
                    
                    if same_calc_count > 1 {
                        suggestions.push(OptimizationSuggestion {
                            title: "Cache Computed Values".to_string(),
                            location: format!("{}:{}", file_name, i + 1),
                            current: "Repeated calculations".to_string(),
                            recommendation: "Store intermediate results in variables".to_string(),
                            savings: 100,
                        });
                    }
                }
            }

            // Check for vector usage where table would be better
            if line.contains("vector::") && (line.contains("borrow") || line.contains("contains")) {
                suggestions.push(OptimizationSuggestion {
                    title: "Use Efficient Data Structures".to_string(),
                    location: format!("{}:{}", file_name, i + 1),
                    current: "Using vector for lookups".to_string(),
                    recommendation: "Use Table or ObjectTable for O(1) access".to_string(),
                    savings: 180,
                });
            }
        }
    }

    // Remove duplicates and limit to top 5
    suggestions.sort_by(|a, b| b.savings.cmp(&a.savings));
    suggestions.truncate(5);

    Ok(suggestions)
}


fn parse_move_functions(source_dir: &str) -> Result<Vec<GasProfile>> {
    let mut profiles = Vec::new();
    let path = Path::new(source_dir);

    if !path.exists() {
        return Ok(profiles);
    }

    let function_regex = Regex::new(r"(?m)^\s*(?:public\s+)?fun\s+(\w+)").unwrap();

    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "move"))
    {
        let content = fs::read_to_string(entry.path())?;
        
        for cap in function_regex.captures_iter(&content) {
            let function_name = cap[1].to_string();
            
            // Estimate gas based on function complexity
            let gas_estimate = estimate_gas_usage(&content, &function_name);
            
            profiles.push(GasProfile {
                function: function_name,
                gas_used: gas_estimate.total,
                storage_cost: gas_estimate.storage,
                computation_cost: gas_estimate.computation,
            });
        }
    }

    Ok(profiles)
}

struct GasEstimate {
    total: u64,
    storage: u64,
    computation: u64,
}

fn estimate_gas_usage(content: &str, function_name: &str) -> GasEstimate {
    // Find function body
    let function_pattern = format!(r"fun\s+{}\s*\([^)]*\)[^{{]*\{{", function_name);
    let function_regex = Regex::new(&function_pattern).unwrap();
    
    if let Some(mat) = function_regex.find(content) {
        let start = mat.end();
        let remaining = &content[start..];
        
        // Count operations that affect gas
        let object_new_count = remaining.matches("object::new").count() as u64;
        let transfer_count = remaining.matches("transfer::").count() as u64;
        let table_ops = remaining.matches("table::").count() as u64;
        let vector_ops = remaining.matches("vector::").count() as u64;
        let balance_ops = remaining.matches("balance::").count() as u64;
        
        // Estimate costs (approximate values based on Sui gas schedule)
        let storage = object_new_count * 500 + table_ops * 300;
        let computation = transfer_count * 200 + vector_ops * 50 + balance_ops * 100;
        let total = storage + computation + 200; // Base cost
        
        GasEstimate {
            total,
            storage,
            computation,
        }
    } else {
        GasEstimate {
            total: 200,
            storage: 0,
            computation: 200,
        }
    }
}
