use crate::bot::adapter::VcsAdapter;
use crate::review::ReviewId;
use crate::utils::error::Result;
use tracing::info;

/// Type of notification to send
#[derive(Debug, Clone)]
pub enum NotificationType {
    MergeSuccess,
    MergeConflict(String),
    CiFailed(Vec<String>),
    QueuePosition(usize),
    ReviewRequiresUpdate,
}

/// Notification service
pub struct Notifier {
    adapter: Box<dyn VcsAdapter>,
}

impl Notifier {
    /// Create a new notifier
    pub fn new(adapter: Box<dyn VcsAdapter>) -> Self {
        Self { adapter }
    }
    
    /// Send a notification
    pub async fn notify(&self, review_id: &ReviewId, notification: NotificationType) -> Result<()> {
        let message = match notification {
            NotificationType::MergeSuccess => {
                "✅ Your review has been successfully merged to main!".to_string()
            },
            NotificationType::MergeConflict(ref details) => {
                format!("⚠️ Merge conflict detected:\n\n{}\n\nPlease rebase your branch and update the review.", details)
            },
            NotificationType::CiFailed(ref failures) => {
                format!("❌ CI checks failed:\n\n{}\n\nPlease fix the issues and update the review.", failures.join("\n"))
            },
            NotificationType::QueuePosition(position) => {
                format!("📋 Your review is in the merge queue at position {}", position + 1)
            },
            NotificationType::ReviewRequiresUpdate => {
                "🔄 Your review requires updates. Please address the feedback and push new commits.".to_string()
            },
        };
        
        info!("Sending notification to review {}: {:?}", review_id, notification);
        self.adapter.post_comment(review_id, &message).await?;
        
        Ok(())
    }
}
