use crate::utils::error::Result;
use crate::git::GitRepository;
use crate::utils::config::Config;
use tracing::info;

/// Execute the squash command - squashes commits on current branch
pub async fn execute() -> Result<()> {
    info!("Squashing commits on current branch");
    
    let repo = GitRepository::open_current()?;
    let current_branch = repo.get_current_branch()?;
    
    // Load config to get base branch
    let config = Config::load_with_env().unwrap_or_default();
    let base = &config.git.squash_base;
    
    let count = repo.get_commit_count(base)?;
    
    if count == 0 {
        println!("✓ No commits to squash (branch is up to date with {})", base);
        return Ok(());
    }
    
    if count == 1 {
        println!("✓ Only one commit, no squashing needed");
        return Ok(());
    }
    
    println!("Squashing {} commits on branch '{}'...", count, current_branch);
    repo.squash_commits(base)?;
    
    println!("✓ Successfully squashed {} commits into one", count);
    println!("  Your changes are now in a single clean commit");
    
    Ok(())
}
