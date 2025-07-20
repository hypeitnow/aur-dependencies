use clap::{Parser, Subcommand};
use anyhow::{Result, Context};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Command-line interface for the AUR Dependency Tracker
#[derive(Parser)]
#[command(name = "aur-dependency-tracker")]
#[command(about = "Track AUR package dependencies")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Supported subcommands for the CLI
#[derive(Subcommand)]
enum Commands {
    /// Track remote AUR package dependencies
    Remote {
        /// Package name to track
        package: String,
    },
    /// Track local AUR package dependencies
    Local {
        /// Package name to track
        package: String,
    },
    /// List tracked packages
    List,
}

/// AUR API response structure
#[derive(Debug, Deserialize)]
struct AurResponse {
    /// List of package results
    results: Vec<AurPackage>,
}

/// AUR package structure
#[derive(Debug, Deserialize)]
struct AurPackage {
    /// List of dependencies (if any)
    Depends: Option<Vec<String>>,
    /// Name of the package
    Name: String,
}

/// Main entry point for the CLI application
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Remote { package } => {
            println!("Tracking remote AUR package: {}", package);
            let url = format!(
                "https://aur.archlinux.org/rpc/?v=5&type=info&arg={}",
                package
            );
            let resp = reqwest::get(&url).await?.json::<serde_json::Value>().await?;
            if let Some(results) = resp.get("results") {
                if let Some(pkg) = results.get(0) {
                    let name = pkg.get("Name").and_then(|n| n.as_str()).unwrap_or("");
                    let depends = pkg.get("Depends").and_then(|d| d.as_array());
                    println!("Package: {}", name);
                    println!("Dependencies:");
                    if let Some(depends) = depends {
                        for dep in depends {
                            if let Some(dep_str) = dep.as_str() {
                                println!("  - {}", dep_str);
                            }
                        }
                    } else {
                        println!("  (none)");
                    }
                } else {
                    println!("No such package found in AUR.");
                }
            } else {
                println!("Invalid response from AUR API.");
            }
        }
        Commands::Local { package } => {
            println!("Tracking local AUR package: {}", package);
            // Try to find the package in the local pacman database
            let db_path = "/var/lib/pacman/local";
            let pkg_dir = Path::new(db_path).join(format!("{}-", package));
            let entries = fs::read_dir(db_path).context("Failed to read local pacman db")?;
            let mut found = false;
            for entry in entries {
                let entry = entry?;
                let fname = entry.file_name();
                let fname = fname.to_string_lossy();
                if fname.starts_with(&format!("{}-", package)) {
                    let desc_path = entry.path().join("desc");
                    if desc_path.exists() {
                        let content = fs::read_to_string(&desc_path)?;
                        println!("Found local package: {}", fname);
                        for line in content.lines() {
                            if line == "%DEPENDS%" {
                                println!("Dependencies:");
                                for dep in content.lines().skip_while(|l| *l != "%DEPENDS%").skip(1).take_while(|l| !l.starts_with('%')) {
                                    println!("  - {}", dep);
                                }
                                break;
                            }
                        }
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                println!("Local package not found in pacman db.");
            }
        }
        Commands::List => {
            println!("Listing tracked packages...");
            // Placeholder: In a real app, this would read from a config or database
            println!("(No tracked packages yet. Implement persistent storage to enable this feature.)");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_aur_api_response() {
        let data = json!({
            "results": [
                {
                    "Name": "yay",
                    "Depends": ["git", "base-devel", "go"]
                }
            ]
        });
        let results = data.get("results").unwrap().as_array().unwrap();
        let pkg = &results[0];
        let name = pkg.get("Name").and_then(|n| n.as_str()).unwrap();
        let depends = pkg.get("Depends").and_then(|d| d.as_array()).unwrap();
        let deps: Vec<_> = depends.iter().map(|d| d.as_str().unwrap()).collect();
        assert_eq!(name, "yay");
        assert_eq!(deps, vec!["git", "base-devel", "go"]);
    }

    #[test]
    fn test_parse_local_desc_depends() {
        let desc_content = "%NAME%\nyay\n%VERSION%\n11.1.2-1\n%DEPENDS%\ngit\nbase-devel\ngo\n%END%\n";
        let mut found = false;
        for line in desc_content.lines() {
            if line == "%DEPENDS%" {
                let deps: Vec<_> = desc_content
                    .lines()
                    .skip_while(|l| *l != "%DEPENDS%")
                    .skip(1)
                    .take_while(|l| !l.starts_with('%'))
                    .collect();
                assert_eq!(deps, vec!["git", "base-devel", "go"]);
                found = true;
                break;
            }
        }
        assert!(found, "Should find %DEPENDS% section");
    }

    #[test]
    fn test_parse_aur_api_response_no_results() {
        let data = json!({ "results": [] });
        let results = data.get("results").unwrap().as_array().unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_parse_local_desc_no_depends() {
        let desc_content = "%NAME%\nyay\n%VERSION%\n11.1.2-1\n%END%\n";
        let mut found = false;
        for line in desc_content.lines() {
            if line == "%DEPENDS%" {
                found = true;
            }
        }
        assert!(!found, "Should not find %DEPENDS% section");
    }

    #[test]
    fn test_parse_local_desc_empty_depends() {
        let desc_content = "%NAME%\nyay\n%VERSION%\n11.1.2-1\n%DEPENDS%\n%END%\n";
        let mut found = false;
        for line in desc_content.lines() {
            if line == "%DEPENDS%" {
                let deps: Vec<_> = desc_content
                    .lines()
                    .skip_while(|l| *l != "%DEPENDS%")
                    .skip(1)
                    .take_while(|l| !l.starts_with('%'))
                    .collect();
                assert!(deps.is_empty());
                found = true;
                break;
            }
        }
        assert!(found, "Should find %DEPENDS% section");
    }

    #[test]
    fn test_parse_aur_api_response_malformed_json() {
        let data = json!({ "unexpected": 123 });
        assert!(data.get("results").is_none());
    }
}
