use crate::utils::error::Result;
use crate::git::GitRepository;
use tracing::info;

/// Execute the start command - creates a new feature branch
pub async fn execute(branch_name: &str) -> Result<()> {
    info!("Starting new branch: {}", branch_name);
    
    let repo = GitRepository::open_current()?;
    repo.create_branch(branch_name)?;
    
    println!("✓ Created and checked out branch: {}", branch_name);
    println!("  You can now make commits on this branch");
    
    Ok(())
}
