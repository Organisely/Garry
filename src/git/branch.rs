use git2::Repository;
use crate::utils::error::{GarryError, Result};

/// Branch-related operations
pub struct BranchOps;

impl BranchOps {
    /// Validate branch name format
    pub fn validate_branch_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(GarryError::InvalidBranchName("Branch name cannot be empty".to_string()));
        }
        
        // Check for invalid characters
        if name.contains("..") || name.contains("//") {
            return Err(GarryError::InvalidBranchName(
                "Branch name contains invalid sequences".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Get the current branch name
    pub fn get_current_branch(repo: &Repository) -> Result<String> {
        let head = repo.head()?;
        
        if let Some(name) = head.shorthand() {
            Ok(name.to_string())
        } else {
            Err(GarryError::GitError(git2::Error::from_str("Not on a branch")))
        }
    }
}
