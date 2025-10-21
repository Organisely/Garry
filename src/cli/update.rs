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
    
    // Try regular push first, fall back to force push if needed (after squash)
    match repo.push_branch(&branch, &config.git.default_remote) {
        Ok(_) => {},
        Err(e) => {
            // If push fails due to non-fast-forward, force push
            if e.to_string().contains("non-fastforwardable") || e.to_string().contains("non-fast-forward") {
                info!("Regular push failed, force pushing (likely after squash)");
                println!("  Force pushing (history was rewritten by squash)...");
                repo.force_push_branch(&branch, &config.git.default_remote)?;
            } else {
                return Err(e);
            }
        }
    }
    
    println!("✓ Successfully updated review");
    println!("  Branch: {}", branch);
    println!("  CI checks will run automatically");
    
    Ok(())
}
