use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "aur-dependency-tracker")]
#[command(about = "Track AUR package dependencies")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Remote { package } => {
            println!("Tracking remote AUR package: {}", package);
            // TODO: Implement remote tracking logic
        }
        Commands::Local { package } => {
            println!("Tracking local AUR package: {}", package);
            // TODO: Implement local tracking logic
        }
        Commands::List => {
            println!("Listing tracked packages...");
            // TODO: Implement list logic
        }
    }

    Ok(())
}
