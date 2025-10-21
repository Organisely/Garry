use thiserror::Error;

/// Main error type for Garry operations
#[derive(Debug, Error)]
pub enum GarryError {
    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),
    
    #[error("VCS adapter error: {0}")]
    VcsError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Review not found: {0}")]
    ReviewNotFound(String),
    
    #[error("Merge conflict: {0}")]
    MergeConflict(String),
    
    #[error("CI check failed: {0:?}")]
    CiFailed(Vec<String>),
    
    #[error("Invalid branch name: {0}")]
    InvalidBranchName(String),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("{0}")]
    Other(String),
}

/// Result type alias for Garry operations
pub type Result<T> = std::result::Result<T, GarryError>;
