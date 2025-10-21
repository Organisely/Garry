use std::collections::VecDeque;
use chrono::{DateTime, Utc};
use crate::bot::adapter::VcsAdapter;
use crate::bot::ci::CiMonitor;
use crate::review::{ReviewId, ReviewState, CiStatus};
use crate::utils::error::{GarryError, Result};
use tracing::{info, warn, error};

/// Entry in the merge queue
#[derive(Debug, Clone)]
pub struct QueueEntry {
    pub review_id: ReviewId,
    pub branch: String,
    pub author: String,
    pub added_at: DateTime<Utc>,
    pub status: QueueStatus,
}

/// Status of a queue entry
#[derive(Debug, Clone, PartialEq)]
pub enum QueueStatus {
    Pending,
    Testing,
    Merging,
    Failed(String),
}

/// Manages the merge queue
pub struct QueueManager {
    queue: VecDeque<QueueEntry>,
    adapter: Box<dyn VcsAdapter>,
    ci_monitor: CiMonitor,
}

impl QueueManager {
    /// Create a new queue manager
    pub fn new(adapter: Box<dyn VcsAdapter>) -> Self {
        let ci_monitor = CiMonitor::new(adapter.as_ref());
        Self {
            queue: VecDeque::new(),
            adapter,
            ci_monitor,
        }
    }
    
    /// Discover approved reviews and add them to the queue
    pub async fn discover_and_queue_reviews(&mut self) -> Result<()> {
        // Get all pending reviews
        let pending_reviews = self.adapter.list_pending_reviews().await?;
        
        for review_id in pending_reviews {
            // Skip if already in queue
            if self.queue.iter().any(|e| e.review_id == review_id) {
                continue;
            }
            
            // Check if approved and CI passed
            match self.adapter.get_review_status(&review_id).await {
                Ok(status) => {
                    if status.state == ReviewState::Approved && status.ci_status == CiStatus::Success {
                        info!("Found approved review {} with passing CI, adding to queue", review_id);
                        let _ = self.add_to_queue(review_id).await;
                    }
                },
                Err(e) => {
                    warn!("Failed to get status for review {}: {}", review_id, e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Add a review to the merge queue
    pub async fn add_to_queue(&mut self, review_id: ReviewId) -> Result<()> {
        info!("Adding review {} to merge queue", review_id);
        
        // Validate review is approved and CI passed
        let status = self.adapter.get_review_status(&review_id).await?;
        
        if status.state != ReviewState::Approved {
            return Err(GarryError::VcsError(
                format!("Review {} is not approved", review_id)
            ));
        }
        
        if status.ci_status != CiStatus::Success {
            return Err(GarryError::CiFailed(vec![
                "CI checks must pass before adding to queue".to_string()
            ]));
        }
        
        // Check if already in queue
        if self.queue.iter().any(|e| e.review_id == review_id) {
            warn!("Review {} is already in queue", review_id);
            return Ok(());
        }
        
        let entry = QueueEntry {
            review_id: review_id.clone(),
            branch: "unknown".to_string(), // TODO: Get from review
            author: "unknown".to_string(),
            added_at: Utc::now(),
            status: QueueStatus::Pending,
        };
        
        self.queue.push_back(entry);
        info!("Review {} added to queue at position {}", review_id, self.queue.len());
        
        Ok(())
    }
    
    /// Process the merge queue
    pub async fn process_queue(&mut self) -> Result<()> {
        if self.queue.is_empty() {
            return Ok(());
        }
        
        info!("Processing merge queue ({} entries)", self.queue.len());
        
        // Process first entry
        if let Some(entry) = self.queue.front_mut() {
            match entry.status {
                QueueStatus::Pending => {
                    info!("Testing review {}", entry.review_id);
                    entry.status = QueueStatus::Testing;
                    
                    // Check CI status
                    match self.ci_monitor.check_ci_status(&entry.review_id).await? {
                        CiStatus::Success => {
                            info!("CI passed for review {}, proceeding to merge", entry.review_id);
                            entry.status = QueueStatus::Merging;
                        },
                        CiStatus::Failed(failures) => {
                            error!("CI failed for review {}: {:?}", entry.review_id, failures);
                            entry.status = QueueStatus::Failed(format!("CI failed: {:?}", failures));
                        },
                        _ => {
                            info!("CI still running for review {}", entry.review_id);
                        }
                    }
                },
                QueueStatus::Testing => {
                    // Re-check CI
                    match self.ci_monitor.check_ci_status(&entry.review_id).await? {
                        CiStatus::Success => {
                            entry.status = QueueStatus::Merging;
                        },
                        CiStatus::Failed(failures) => {
                            entry.status = QueueStatus::Failed(format!("CI failed: {:?}", failures));
                        },
                        _ => {}
                    }
                },
                QueueStatus::Merging => {
                    info!("Merging review {}", entry.review_id);
                    
                    match self.adapter.merge_review(&entry.review_id).await {
                        Ok(_) => {
                            info!("Successfully merged review {}", entry.review_id);
                            self.queue.pop_front();
                        },
                        Err(e) => {
                            error!("Failed to merge review {}: {}", entry.review_id, e);
                            entry.status = QueueStatus::Failed(format!("Merge failed: {}", e));
                        }
                    }
                },
                QueueStatus::Failed(_) => {
                    warn!("Removing failed review {} from queue", entry.review_id);
                    self.queue.pop_front();
                }
            }
        }
        
        Ok(())
    }
    
    /// Remove a review from the queue
    pub async fn remove_from_queue(&mut self, review_id: &ReviewId) -> Result<()> {
        info!("Removing review {} from queue", review_id);
        self.queue.retain(|e| &e.review_id != review_id);
        Ok(())
    }
    
    /// Get position of a review in the queue
    pub fn get_queue_position(&self, review_id: &ReviewId) -> Option<usize> {
        self.queue.iter().position(|e| &e.review_id == review_id)
    }
    
    /// Get the current queue
    pub fn get_queue(&self) -> &VecDeque<QueueEntry> {
        &self.queue
    }
}
