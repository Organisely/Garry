use crate::utils::error::{GarryError, Result};
use crate::utils::config::Config;
use crate::bot::adapters::GithubAdapter;
use crate::bot::adapter::VcsAdapter;
use crate::review::ReviewId;
use tracing::info;

/// Execute the approve command - approves a review
pub async fn execute(review_id: String, message: Option<String>) -> Result<()> {
    info!("Approving review {}", review_id);
    
    // Load config
    let config = Config::load_with_env()?;
    
    // Create adapter
    let adapter: Box<dyn VcsAdapter> = match config.vcs.platform.as_str() {
        "github" => Box::new(GithubAdapter::new(
            config.vcs.host.clone(),
            config.vcs.token.clone(),
            config.vcs.repository.clone(),
        )?),
        _ => return Err(GarryError::VcsError(format!("Unsupported platform: {}", config.vcs.platform))),
    };
    
    let review_id = ReviewId::new(review_id);
    
    println!("Approving review {}...", review_id);
    adapter.approve_review(&review_id, message.as_deref()).await?;
    
    println!("✓ Successfully approved review {}", review_id);
    println!("  The bot will automatically merge it when CI passes");
    
    Ok(())
}
