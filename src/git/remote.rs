use git2::{Repository, PushOptions, RemoteCallbacks, Cred};
use crate::utils::error::{GarryError, Result};
use tracing::info;

/// Remote-related operations
pub struct RemoteOps;

impl RemoteOps {
    /// Push a branch to remote
    pub fn push_branch(repo: &Repository, branch: &str, remote_name: &str) -> Result<()> {
        Self::push_branch_internal(repo, branch, remote_name, false)
    }
    
    /// Force push a branch to remote (for after squashing)
    pub fn force_push_branch(repo: &Repository, branch: &str, remote_name: &str) -> Result<()> {
        Self::push_branch_internal(repo, branch, remote_name, true)
    }
    
    /// Internal push implementation
    fn push_branch_internal(repo: &Repository, branch: &str, remote_name: &str, force: bool) -> Result<()> {
        if force {
            info!("Force pushing branch '{}' to remote '{}'", branch, remote_name);
        } else {
            info!("Pushing branch '{}' to remote '{}'", branch, remote_name);
        }
        
        let mut remote = repo.find_remote(remote_name)?;
        
        // Set up callbacks for authentication
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|url, username_from_url, allowed_types| {
            let username = username_from_url.unwrap_or("git");
            
            // Try SSH key from agent first
            if allowed_types.is_ssh_key() {
                if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                    return Ok(cred);
                }
                
                // Try default SSH key locations
                if let Some(home) = dirs::home_dir() {
                    let ssh_key = home.join(".ssh").join("id_rsa");
                    let ssh_key_pub = home.join(".ssh").join("id_rsa.pub");
                    if ssh_key.exists() {
                        if let Ok(cred) = Cred::ssh_key(username, Some(&ssh_key_pub), &ssh_key, None) {
                            return Ok(cred);
                        }
                    }
                    
                    // Try id_ed25519
                    let ssh_key = home.join(".ssh").join("id_ed25519");
                    let ssh_key_pub = home.join(".ssh").join("id_ed25519.pub");
                    if ssh_key.exists() {
                        if let Ok(cred) = Cred::ssh_key(username, Some(&ssh_key_pub), &ssh_key, None) {
                            return Ok(cred);
                        }
                    }
                }
            }
            
            // Try credential helper
            if let Ok(config) = repo.config() {
                if let Ok(cred) = Cred::credential_helper(&config, url, username_from_url) {
                    return Ok(cred);
                }
            }
            
            // Try default
            Cred::default()
        });
        
        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);
        
        // Push the branch (with force if needed)
        let refspec = if force {
            format!("+refs/heads/{}:refs/heads/{}", branch, branch)
        } else {
            format!("refs/heads/{}:refs/heads/{}", branch, branch)
        };
        
        remote.push(&[&refspec], Some(&mut push_options))
            .map_err(|e| GarryError::GitError(e))?;
        
        info!("Successfully pushed branch '{}'", branch);
        Ok(())
    }
}
