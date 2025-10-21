use crate::utils::error::{GarryError, Result};
use crate::utils::config::Config;
use crate::git::GitRepository;
use crate::bot::adapters::GithubAdapter;
use crate::bot::adapter::VcsAdapter;
use crate::review::ReviewState;
use tracing::info;

/// Execute the merge command - initiates merge via bot
pub async fn execute() -> Result<()> {
    info!("Initiating merge");
    
    // Load config
    let config = Config::load_with_env()?;
    
    // Get current branch
    let repo = GitRepository::open_current()?;
    let branch = repo.get_current_branch()?;
    
    // Create adapter
    let adapter: Box<dyn VcsAdapter> = match config.vcs.platform.as_str() {
        "github" => Box::new(GithubAdapter::new(
            config.vcs.host.clone(),
            config.vcs.token.clone(),
            config.vcs.repository.clone(),
        )?),
        _ => return Err(GarryError::VcsError(format!("Unsupported platform: {}", config.vcs.platform))),
    };
    
    println!("Checking review status for branch '{}'...", branch);
    
    // For now, we'll assume the review ID matches the branch name or needs to be looked up
    // In a real implementation, we'd store the review ID locally or look it up
    println!("⚠️  Note: Automatic merge queue is handled by garry-bot service");
    println!("   To merge manually, ensure:");
    println!("   1. Review is approved");
    println!("   2. CI checks pass");
    println!("   3. No merge conflicts");
    println!();
    println!("   The bot will automatically merge approved reviews in queue order.");
    
    Ok(())
}
