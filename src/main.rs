mod cli;
mod git;
mod review;
mod utils;
mod bot;

use clap::Parser;
use tracing_subscriber;
use utils::error::Result;

#[derive(Parser)]
#[command(name = "garry")]
#[command(about = "A Git workflow enforcement tool", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: cli::Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();

    let cli = Cli::parse();
    
    // Execute the command
    cli::execute(cli.command).await?;
    
    Ok(())
}
