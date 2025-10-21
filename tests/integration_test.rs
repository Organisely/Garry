// Integration tests for Garry

use garry::bot::adapters::MockVcsAdapter;
use garry::bot::adapter::VcsAdapter;
use garry::review::{ReviewState, CiStatus};

#[tokio::test]
async fn test_mock_adapter_workflow() {
    let adapter = MockVcsAdapter::new();
    
    // Create a review
    let (review_id, review_url) = adapter
        .create_review("feature/test", "Test PR", "Test description")
        .await
        .expect("Failed to create review");
    
    assert!(!review_url.is_empty());
    
    // Get review status
    let status = adapter
        .get_review_status(&review_id)
        .await
        .expect("Failed to get review status");
    
    assert_eq!(status.state, ReviewState::Open);
    assert_eq!(status.ci_status, CiStatus::Pending);
    
    // Post a comment
    adapter
        .post_comment(&review_id, "Test comment")
        .await
        .expect("Failed to post comment");
    
    // Merge the review
    adapter
        .merge_review(&review_id)
        .await
        .expect("Failed to merge review");
    
    // Verify merged
    let status = adapter
        .get_review_status(&review_id)
        .await
        .expect("Failed to get review status");
    
    assert_eq!(status.state, ReviewState::Merged);
}

#[tokio::test]
async fn test_list_pending_reviews() {
    let adapter = MockVcsAdapter::new();
    
    // Create multiple reviews
    let (_review1, _) = adapter
        .create_review("feature/test1", "Test PR 1", "Description 1")
        .await
        .expect("Failed to create review 1");
    
    let (_review2, _) = adapter
        .create_review("feature/test2", "Test PR 2", "Description 2")
        .await
        .expect("Failed to create review 2");
    
    // List pending reviews
    let reviews = adapter
        .list_pending_reviews()
        .await
        .expect("Failed to list reviews");
    
    assert_eq!(reviews.len(), 2);
}

#[test]
fn test_config_validation() {
    use garry::utils::config::Config;
    
    let config = Config::default();
    
    // Default config should fail validation (empty token and repo)
    assert!(config.validate().is_err());
}

#[test]
fn test_review_id_display() {
    use garry::review::ReviewId;
    
    let id = ReviewId::new("123".to_string());
    assert_eq!(id.to_string(), "123");
    assert_eq!(id.as_str(), "123");
}

#[test]
fn test_ci_status_variants() {
    use garry::review::CiStatus;
    
    let pending = CiStatus::Pending;
    let running = CiStatus::Running;
    let success = CiStatus::Success;
    let failed = CiStatus::Failed(vec!["test failed".to_string()]);
    let cancelled = CiStatus::Cancelled;
    
    assert_eq!(pending, CiStatus::Pending);
    assert_eq!(running, CiStatus::Running);
    assert_eq!(success, CiStatus::Success);
    assert_ne!(success, failed);
    assert_eq!(cancelled, CiStatus::Cancelled);
}

#[test]
fn test_branch_name_validation() {
    use garry::git::branch::BranchOps;
    
    // Valid branch names
    assert!(BranchOps::validate_branch_name("feature/test").is_ok());
    assert!(BranchOps::validate_branch_name("fix/bug-123").is_ok());
    assert!(BranchOps::validate_branch_name("main").is_ok());
    
    // Invalid branch names
    assert!(BranchOps::validate_branch_name("").is_err());
    assert!(BranchOps::validate_branch_name("feature..test").is_err());
    assert!(BranchOps::validate_branch_name("feature//test").is_err());
}

#[test]
fn test_error_types() {
    use garry::utils::error::GarryError;
    
    let git_error = GarryError::GitError(git2::Error::from_str("test error"));
    let vcs_error = GarryError::VcsError("test".to_string());
    let config_error = GarryError::ConfigError("test".to_string());
    
    assert!(git_error.to_string().contains("Git operation failed"));
    assert!(vcs_error.to_string().contains("VCS adapter error"));
    assert!(config_error.to_string().contains("Configuration error"));
}
