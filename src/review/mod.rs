use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Unique identifier for a review (PR/MR)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReviewId(pub String);

impl ReviewId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ReviewId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Status of a review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewStatus {
    pub id: ReviewId,
    pub state: ReviewState,
    pub approvals: Vec<Approval>,
    pub ci_status: CiStatus,
    pub mergeable: bool,
}

/// State of a review
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewState {
    Open,
    Approved,
    ChangesRequested,
    Merged,
    Closed,
}

/// An approval from a reviewer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub reviewer: String,
    pub approved_at: DateTime<Utc>,
}

/// CI/CD status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CiStatus {
    Pending,
    Running,
    Success,
    Failed(Vec<String>),
    Cancelled,
}
