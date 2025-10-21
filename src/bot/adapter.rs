use async_trait::async_trait;
use crate::review::{ReviewId, ReviewStatus, CiStatus};
use crate::utils::error::Result;
use std::collections::HashMap;

/// VCS adapter trait - unified interface for all Git hosting platforms
#[async_trait]
pub trait VcsAdapter: Send + Sync {
    /// Create a new review (PR/MR)
    async fn create_review(&self, branch: &str, title: &str, description: &str) -> Result<(ReviewId, String)>;
    
    /// Get the status of a review
    async fn get_review_status(&self, review_id: &ReviewId) -> Result<ReviewStatus>;
    
    /// Merge a review
    async fn merge_review(&self, review_id: &ReviewId) -> Result<()>;
    
    /// Post a comment on a review
    async fn post_comment(&self, review_id: &ReviewId, message: &str) -> Result<()>;
    
    /// List all pending reviews
    async fn list_pending_reviews(&self) -> Result<Vec<ReviewId>>;
    
    /// Get CI status for a review
    async fn get_ci_status(&self, review_id: &ReviewId) -> Result<CiStatus>;
    
    /// Setup repository protection rules (called by bot on startup)
    async fn setup_repository_protection(&self, main_branch: &str, bot_user: &str) -> Result<()>;
    
    /// Get the platform-specific name for reviews (PR, MR, etc.)
    fn review_name(&self) -> &str;
}

/// Registry for VCS adapters
pub struct AdapterRegistry {
    adapters: HashMap<String, Box<dyn VcsAdapter>>,
}

impl AdapterRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }
    
    /// Register an adapter
    pub fn register(&mut self, name: String, adapter: Box<dyn VcsAdapter>) {
        self.adapters.insert(name, adapter);
    }
    
    /// Get an adapter by name
    pub fn get_adapter(&self, name: &str) -> Option<&Box<dyn VcsAdapter>> {
        self.adapters.get(name)
    }
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}
