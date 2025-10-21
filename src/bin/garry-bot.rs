// Garry Bot - Automation service for merge queue management

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use tracing::{info, error, warn};

// Import from main crate
use garry::bot::VcsAdapter;
use garry::bot::queue::QueueManager;
use garry::bot::adapters::GithubAdapter;
use garry::utils::config::Config;
use garry::utils::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();
    
    info!("Starting Garry Bot...");
    
    // Load configuration
    let config = Config::load_with_env()?;
    info!("Loaded configuration for repository: {}", config.vcs.repository);
    
    // Create VCS adapter
    let adapter = match config.vcs.platform.as_str() {
        "github" => Box::new(GithubAdapter::new(
            config.vcs.host.clone(),
            config.vcs.token.clone(),
            config.vcs.repository.clone(),
        )?),
        _ => {
            error!("Unsupported VCS platform: {}", config.vcs.platform);
            return Err(garry::utils::error::GarryError::ConfigError(
                format!("Unsupported platform: {}", config.vcs.platform)
            ));
        }
    };
    
    // Set up repository protection
    info!("Setting up repository protection...");
    if let Err(e) = adapter.setup_repository_protection(&config.bot.main_branch, "garry-bot").await {
        warn!("Could not set up repository protection: {}", e);
        warn!("You may need to configure branch protection manually");
    }
    
    // Create queue manager
    let queue_manager = Arc::new(Mutex::new(QueueManager::new(adapter)));
    
    info!("Garry Bot started successfully");
    info!("Monitoring repository: {}", config.vcs.repository);
    info!("Queue check interval: {}s", config.bot.queue_check_interval);
    info!("⚠️  Only Garry Bot can push to {} - all merges go through the queue", config.bot.main_branch);
    
    // Start queue processing loop
    let mut check_interval = interval(Duration::from_secs(config.bot.queue_check_interval));
    
    loop {
        check_interval.tick().await;
        
        let mut qm = queue_manager.lock().await;
        
        // Discover and queue approved PRs
        if let Err(e) = qm.discover_and_queue_reviews().await {
            error!("Error discovering reviews: {}", e);
        }
        
        // Process the merge queue
        if let Err(e) = qm.process_queue().await {
            error!("Error processing queue: {}", e);
        }
    }
}
