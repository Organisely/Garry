use git2::{Repository, Signature};
use crate::utils::error::{GarryError, Result};
use tracing::info;

/// Commit-related operations
pub struct CommitOps;

impl CommitOps {
    /// Get the number of commits between current HEAD and base branch
    pub fn get_commit_count(repo: &Repository, base: &str) -> Result<usize> {
        let head = repo.head()?;
        let head_commit = head.peel_to_commit()?;
        
        // Try to find base branch
        let base_ref = repo.find_reference(&format!("refs/heads/{}", base))
            .or_else(|_| repo.find_reference(&format!("refs/remotes/origin/{}", base)))?;
        let base_commit = base_ref.peel_to_commit()?;
        
        // Count commits
        let mut revwalk = repo.revwalk()?;
        revwalk.push(head_commit.id())?;
        revwalk.hide(base_commit.id())?;
        
        Ok(revwalk.count())
    }
    
    /// Squash commits on current branch into a single commit
    pub fn squash_commits(repo: &Repository, base: &str) -> Result<()> {
        let count = Self::get_commit_count(repo, base)?;
        
        if count == 0 {
            info!("No commits to squash");
            return Ok(());
        }
        
        if count == 1 {
            info!("Only one commit, no squashing needed");
            return Ok(());
        }
        
        info!("Squashing {} commits", count);
        
        // Get current branch
        let head = repo.head()?;
        let branch_name = head.shorthand()
            .ok_or_else(|| GarryError::GitError(git2::Error::from_str("Not on a branch")))?;
        
        // Get base commit
        let base_ref = repo.find_reference(&format!("refs/heads/{}", base))
            .or_else(|_| repo.find_reference(&format!("refs/remotes/origin/{}", base)))?;
        let base_commit = base_ref.peel_to_commit()?;
        
        // Get current HEAD commit
        let head_commit = head.peel_to_commit()?;
        
        // Collect all commit messages
        let mut messages = Vec::new();
        let mut revwalk = repo.revwalk()?;
        revwalk.push(head_commit.id())?;
        revwalk.hide(base_commit.id())?;
        
        for oid in revwalk {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;
            if let Some(msg) = commit.message() {
                messages.push(msg.to_string());
            }
        }
        messages.reverse();
        
        // Create combined message
        let combined_message = if messages.len() > 1 {
            format!("Squashed {} commits:\n\n{}", messages.len(), messages.join("\n\n"))
        } else {
            messages.first().cloned().unwrap_or_else(|| "Squashed commit".to_string())
        };
        
        // Get the tree from HEAD (this has all the changes)
        let tree = head_commit.tree()?;
        
        // Get signature
        let signature = repo.signature()
            .or_else(|_| Signature::now("Garry", "garry@organisely.com"))?;
        
        // Create new commit on base
        let new_commit_oid = repo.commit(
            Some(&format!("refs/heads/{}", branch_name)),
            &signature,
            &signature,
            &combined_message,
            &tree,
            &[&base_commit],
        )?;
        
        info!("Squashed commits into: {}", new_commit_oid);
        
        // Force checkout to update working directory
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        
        Ok(())
    }
}
