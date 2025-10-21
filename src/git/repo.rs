use git2::Repository;
use crate::utils::error::{GarryError, Result};
use crate::git::branch::BranchOps;
use crate::git::commit::CommitOps;
use crate::git::remote::RemoteOps;
use std::path::Path;
use tracing::info;

/// Wrapper around git2::Repository with Garry-specific operations
pub struct GitRepository {
    repo: Repository,
}

impl GitRepository {
    /// Open a repository at the given path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::open(path)?;
        Ok(Self { repo })
    }
    
    /// Open the repository in the current directory
    pub fn open_current() -> Result<Self> {
        let repo = Repository::open_from_env()?;
        Ok(Self { repo })
    }
    
    /// Get the underlying git2::Repository
    pub fn inner(&self) -> &Repository {
        &self.repo
    }
    
    /// Create a new branch
    pub fn create_branch(&self, name: &str) -> Result<()> {
        BranchOps::validate_branch_name(name)?;
        
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        
        self.repo.branch(name, &commit, false)
            .map_err(|e| {
                if e.code() == git2::ErrorCode::Exists {
                    GarryError::InvalidBranchName(format!("Branch '{}' already exists", name))
                } else {
                    GarryError::GitError(e)
                }
            })?;
        
        // Checkout the new branch
        self.repo.set_head(&format!("refs/heads/{}", name))?;
        self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        
        info!("Created and checked out branch: {}", name);
        Ok(())
    }
    
    /// Get current branch name
    pub fn get_current_branch(&self) -> Result<String> {
        BranchOps::get_current_branch(&self.repo)
    }
    
    /// Get commit count from base branch
    pub fn get_commit_count(&self, base: &str) -> Result<usize> {
        CommitOps::get_commit_count(&self.repo, base)
    }
    
    /// Squash commits on current branch
    pub fn squash_commits(&self, base: &str) -> Result<()> {
        CommitOps::squash_commits(&self.repo, base)
    }
    
    /// Push branch to remote
    pub fn push_branch(&self, branch: &str, remote: &str) -> Result<()> {
        RemoteOps::push_branch(&self.repo, branch, remote)
    }
    
    /// Force push branch to remote (for after squashing)
    pub fn force_push_branch(&self, branch: &str, remote: &str) -> Result<()> {
        RemoteOps::force_push_branch(&self.repo, branch, remote)
    }
}
