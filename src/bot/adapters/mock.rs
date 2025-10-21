use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::bot::adapter::VcsAdapter;
use crate::review::{ReviewId, ReviewStatus, ReviewState, CiStatus};
use crate::utils::error::Result;

/// Mock VCS adapter for testing
pub struct MockVcsAdapter {
    reviews: Arc<Mutex<HashMap<ReviewId, ReviewStatus>>>,
    next_id: Arc<Mutex<u64>>,
}

impl MockVcsAdapter {
    pub fn new() -> Self {
        Self {
            reviews: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

#[async_trait]
impl VcsAdapter for MockVcsAdapter {
    async fn create_review(&self, _branch: &str, _title: &str, _description: &str) -> Result<(ReviewId, String)> {
        let mut next_id = self.next_id.lock().unwrap();
        let id = ReviewId::new(next_id.to_string());
        *next_id += 1;
        
        let status = ReviewStatus {
            id: id.clone(),
            state: ReviewState::Open,
            approvals: vec![],
            ci_status: CiStatus::Pending,
            mergeable: true,
        };
        
        self.reviews.lock().unwrap().insert(id.clone(), status);
        let url = format!("https://mock-vcs.example.com/review/{}", id.as_str());
        Ok((id, url))
    }
    
    async fn get_review_status(&self, review_id: &ReviewId) -> Result<ReviewStatus> {
        let reviews = self.reviews.lock().unwrap();
        reviews.get(review_id)
            .cloned()
            .ok_or_else(|| crate::utils::error::GarryError::ReviewNotFound(review_id.to_string()))
    }
    
    async fn merge_review(&self, review_id: &ReviewId) -> Result<()> {
        let mut reviews = self.reviews.lock().unwrap();
        if let Some(status) = reviews.get_mut(review_id) {
            status.state = ReviewState::Merged;
        }
        Ok(())
    }
    
    async fn post_comment(&self, _review_id: &ReviewId, _message: &str) -> Result<()> {
        Ok(())
    }
    
    async fn approve_review(&self, review_id: &ReviewId, _message: Option<&str>) -> Result<()> {
        let mut reviews = self.reviews.lock().unwrap();
        if let Some(status) = reviews.get_mut(review_id) {
            status.state = ReviewState::Approved;
        }
        Ok(())
    }
    
    async fn list_pending_reviews(&self) -> Result<Vec<ReviewId>> {
        let reviews = self.reviews.lock().unwrap();
        Ok(reviews.keys().cloned().collect())
    }
    
    async fn get_ci_status(&self, review_id: &ReviewId) -> Result<CiStatus> {
        let reviews = self.reviews.lock().unwrap();
        Ok(reviews.get(review_id)
            .map(|s| s.ci_status.clone())
            .unwrap_or(CiStatus::Pending))
    }
    
    async fn setup_repository_protection(&self, _main_branch: &str, _bot_user: &str) -> Result<()> {
        // Mock implementation - no-op
        Ok(())
    }
    
    fn review_name(&self) -> &str {
        "Review"
    }
}
