use crate::bot::adapter::VcsAdapter;
use crate::review::{ReviewId, CiStatus};
use crate::utils::error::Result;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, debug};

/// CI monitoring service
pub struct CiMonitor {
    // We store a reference to the adapter but can't use trait objects directly
    // In real implementation, this would be handled differently
}

impl CiMonitor {
    /// Create a new CI monitor
    pub fn new(_adapter: &dyn VcsAdapter) -> Self {
        Self {}
    }
    
    /// Check CI status for a review
    pub async fn check_ci_status(&self, review_id: &ReviewId) -> Result<CiStatus> {
        debug!("Checking CI status for review {}", review_id);
        // This will be called through the adapter in the queue manager
        Ok(CiStatus::Pending)
    }
    
    /// Wait for CI to complete with timeout
    pub async fn wait_for_ci(&self, review_id: &ReviewId, timeout: Duration) -> Result<CiStatus> {
        info!("Waiting for CI to complete for review {} (timeout: {:?})", review_id, timeout);
        
        let start = std::time::Instant::now();
        let check_interval = Duration::from_secs(30);
        
        loop {
            if start.elapsed() > timeout {
                return Ok(CiStatus::Failed(vec!["CI timeout".to_string()]));
            }
            
            let status = self.check_ci_status(review_id).await?;
            
            match status {
                CiStatus::Success | CiStatus::Failed(_) | CiStatus::Cancelled => {
                    return Ok(status);
                },
                _ => {
                    sleep(check_interval).await;
                }
            }
        }
    }
}
