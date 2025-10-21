use crate::utils::error::{GarryError, Result};
use crate::utils::config::Config;
use crate::git::GitRepository;
use tracing::info;

/// Execute the update command - updates an existing review
pub async fn execute() -> Result<()> {
    info!("Updating existing review");
    
    // Load config
    let config = Config::load_with_env()?;
    
    // Get current branch
    let repo = GitRepository::open_current()?;
    let branch = repo.get_current_branch()?;
    
    // Push updated commits to remote
    println!("Pushing updated commits for branch '{}'...", branch);
    repo.push_branch(&branch, &config.git.default_remote)?;
    
    println!("✓ Successfully updated review");
    println!("  Branch: {}", branch);
    println!("  CI checks will run automatically");
    
    Ok(())
}
