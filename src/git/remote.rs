use git2::{Repository, PushOptions, RemoteCallbacks, Cred};
use crate::utils::error::{GarryError, Result};
use tracing::info;

/// Remote-related operations
pub struct RemoteOps;

impl RemoteOps {
    /// Push a branch to remote
    pub fn push_branch(repo: &Repository, branch: &str, remote_name: &str) -> Result<()> {
        info!("Pushing branch '{}' to remote '{}'", branch, remote_name);
        
        let mut remote = repo.find_remote(remote_name)?;
        
        // Set up callbacks for authentication
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            // Try SSH key first
            if let Ok(cred) = Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                return Ok(cred);
            }
            
            // Try credential helper
            if let Some(config) = repo.config().ok().as_ref() {
                if let Ok(cred) = Cred::credential_helper(config, _url, username_from_url) {
                    return Ok(cred);
                }
            }
            
            // Try default
            Cred::default()
        });
        
        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);
        
        // Push the branch
        let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);
        remote.push(&[&refspec], Some(&mut push_options))
            .map_err(|e| GarryError::GitError(e))?;
        
        info!("Successfully pushed branch '{}'", branch);
        Ok(())
    }
}
