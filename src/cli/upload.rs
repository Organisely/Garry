use crate::utils::error::{GarryError, Result};
use crate::utils::config::Config;
use crate::git::GitRepository;
use crate::bot::adapters::GithubAdapter;
use crate::bot::adapter::VcsAdapter;
use tracing::info;

/// Execute the upload command - uploads changes for review
pub async fn execute(title: Option<String>, description: Option<String>) -> Result<()> {
    info!("Uploading changes for review");
    
    // Load config
    let config = Config::load_with_env()?;
    
    // Get current branch
    let repo = GitRepository::open_current()?;
    let branch = repo.get_current_branch()?;
    
    // Push branch to remote
    println!("Pushing branch '{}' to remote...", branch);
    repo.push_branch(&branch, &config.git.default_remote)?;
    
    // Create adapter based on platform
    let adapter: Box<dyn VcsAdapter> = match config.vcs.platform.as_str() {
        "github" => Box::new(GithubAdapter::new(
            config.vcs.host.clone(),
            config.vcs.token.clone(),
            config.vcs.repository.clone(),
        )?),
        _ => return Err(GarryError::VcsError(format!("Unsupported platform: {}", config.vcs.platform))),
    };
    
    // Generate title and description
    let review_title = title.unwrap_or_else(|| format!("Review: {}", branch));
    let review_description = description.unwrap_or_else(|| format!("Changes from branch {}", branch));
    
    // Create review
    let review_name = adapter.review_name();
    println!("Creating {}...", review_name);
    let (review_id, review_url) = adapter.create_review(&branch, &review_title, &review_description).await?;
    
    println!("✓ Successfully created {}!", review_name);
    println!("  Branch: {}", branch);
    println!("  {} ID: #{}", review_name, review_id);
    println!("  {} URL: {}", review_name, review_url);
    
    Ok(())
}
