# Design Document

## Overview

Garry is a Rust-based Git workflow enforcement tool that implements a clean, squash-first workflow similar to Chromium's Gerrit system. The architecture consists of two main components:

1. **Garry CLI**: A local command-line tool for developers to manage their feature branches, squash commits, and interact with the review system
2. **Garry Bot**: An automation service that manages the merge queue, monitors CI/CD status, and enforces workflow policies

The system is designed to be VCS-agnostic through a unified adapter interface, supporting GitHub, GitLab, Bitbucket, Gitea, and self-hosted Git servers.

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Developer Workstation                   │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              Garry CLI (Rust Binary)                   │ │
│  │  ┌──────────┬──────────┬──────────┬──────────────────┐│ │
│  │  │  start   │  squash  │  upload  │  update / merge  ││ │
│  │  └──────────┴──────────┴──────────┴──────────────────┘│ │
│  │                         │                               │ │
│  │                         ▼                               │ │
│  │              ┌─────────────────────┐                   │ │
│  │              │   Git Operations    │                   │ │
│  │              │     (git2-rs)       │                   │ │
│  │              └─────────────────────┘                   │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ HTTP/API
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    VCS Platform (GitHub/GitLab/etc)          │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Pull Requests / Merge Requests / Reviews              │ │
│  │  Webhooks / API Events                                 │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ Webhooks/Polling
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                      Garry Bot (Service)                     │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              VCS Adapter Layer                         │ │
│  │  ┌────────┬────────┬────────┬────────┬─────────────┐  │ │
│  │  │ GitHub │ GitLab │Bitbucket│ Gitea │ Self-Hosted │  │ │
│  │  └────────┴────────┴────────┴────────┴─────────────┘  │ │
│  └────────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              Commit Queue Manager                      │ │
│  │  - Track pending reviews                               │ │
│  │  - Monitor CI/CD status                                │ │
│  │  - Enforce merge order                                 │ │
│  │  - Handle conflicts                                    │ │
│  └────────────────────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              Notification System                       │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### Component Interaction Flow

1. Developer uses Garry CLI to start a branch, make commits, and squash
2. CLI uploads changes via VCS adapter to create a review
3. VCS platform triggers webhooks to Garry Bot
4. Garry Bot monitors review status, approvals, and CI checks
5. When approved and CI passes, Bot adds review to merge queue
6. Bot processes queue in order, merging to main and notifying developers

## Components and Interfaces

### 1. Garry CLI

The CLI is the primary interface for developers. It wraps Git operations and communicates with the VCS platform.

#### CLI Commands

```rust
// src/cli/mod.rs
pub enum Command {
    Start { branch_name: String },
    Squash,
    Upload,
    Update,
    Merge,
}

pub struct Cli {
    git_repo: GitRepository,
    config: Config,
}

impl Cli {
    pub fn execute(&self, command: Command) -> Result<()>;
}
```

#### Git Operations Module

```rust
// src/git/mod.rs
pub struct GitRepository {
    repo: git2::Repository,
}

impl GitRepository {
    pub fn create_branch(&self, name: &str) -> Result<()>;
    pub fn squash_commits(&self, base: &str) -> Result<()>;
    pub fn push_branch(&self, branch: &str, remote: &str) -> Result<()>;
    pub fn get_current_branch(&self) -> Result<String>;
    pub fn get_commit_count(&self, base: &str) -> Result<usize>;
}
```

### 2. VCS Adapter Layer

The adapter layer provides a unified interface for interacting with different Git hosting platforms.

#### Core Trait

```rust
// src/bot/adapter.rs
#[async_trait]
pub trait VcsAdapter: Send + Sync {
    async fn create_review(&self, branch: &str, title: &str, description: &str) 
        -> Result<ReviewId>;
    
    async fn get_review_status(&self, review_id: &ReviewId) 
        -> Result<ReviewStatus>;
    
    async fn merge_review(&self, review_id: &ReviewId) 
        -> Result<()>;
    
    async fn post_comment(&self, review_id: &ReviewId, message: &str) 
        -> Result<()>;
    
    async fn list_pending_reviews(&self) 
        -> Result<Vec<ReviewId>>;
    
    async fn get_ci_status(&self, review_id: &ReviewId) 
        -> Result<CiStatus>;
}

pub struct AdapterRegistry {
    adapters: HashMap<String, Box<dyn VcsAdapter>>,
}

impl AdapterRegistry {
    pub fn register(&mut self, name: String, adapter: Box<dyn VcsAdapter>);
    pub fn get_adapter(&self, host: &str) -> Option<&Box<dyn VcsAdapter>>;
}
```

#### Adapter Implementations

Each VCS platform will have its own adapter implementation:

- **GithubAdapter**: Uses GitHub REST API and GraphQL API
- **GitLabAdapter**: Uses GitLab REST API
- **BitbucketAdapter**: Uses Bitbucket Cloud/Server API
- **GiteaAdapter**: Uses Gitea API
- **SelfHostedGitAdapter**: Generic Git server with webhook support

### 3. Garry Bot

The bot is a long-running service that monitors reviews and manages the merge queue.

#### Queue Manager

```rust
// src/bot/queue.rs
pub struct QueueManager {
    queue: VecDeque<QueueEntry>,
    adapter: Box<dyn VcsAdapter>,
    ci_monitor: CiMonitor,
}

pub struct QueueEntry {
    review_id: ReviewId,
    branch: String,
    author: String,
    added_at: DateTime<Utc>,
    status: QueueStatus,
}

pub enum QueueStatus {
    Pending,
    Testing,
    Merging,
    Failed(String),
}

impl QueueManager {
    pub async fn add_to_queue(&mut self, review_id: ReviewId) -> Result<()>;
    pub async fn process_queue(&mut self) -> Result<()>;
    pub async fn remove_from_queue(&mut self, review_id: &ReviewId) -> Result<()>;
    pub fn get_queue_position(&self, review_id: &ReviewId) -> Option<usize>;
}
```

#### CI Integration

```rust
// src/bot/ci.rs
pub struct CiMonitor {
    adapter: Box<dyn VcsAdapter>,
}

#[derive(Debug, Clone)]
pub enum CiStatus {
    Pending,
    Running,
    Success,
    Failed(Vec<String>),
    Cancelled,
}

impl CiMonitor {
    pub async fn check_ci_status(&self, review_id: &ReviewId) -> Result<CiStatus>;
    pub async fn wait_for_ci(&self, review_id: &ReviewId, timeout: Duration) 
        -> Result<CiStatus>;
}
```

#### Notification System

```rust
// src/bot/notifier.rs
pub struct Notifier {
    adapter: Box<dyn VcsAdapter>,
}

pub enum NotificationType {
    MergeSuccess,
    MergeConflict(String),
    CiFailed(Vec<String>),
    QueuePosition(usize),
    ReviewRequiresUpdate,
}

impl Notifier {
    pub async fn notify(&self, review_id: &ReviewId, notification: NotificationType) 
        -> Result<()>;
}
```

## Data Models

### Core Types

```rust
// src/review/mod.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReviewId(String);

#[derive(Debug, Clone)]
pub struct ReviewStatus {
    pub id: ReviewId,
    pub state: ReviewState,
    pub approvals: Vec<Approval>,
    pub ci_status: CiStatus,
    pub mergeable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReviewState {
    Open,
    Approved,
    ChangesRequested,
    Merged,
    Closed,
}

#[derive(Debug, Clone)]
pub struct Approval {
    pub reviewer: String,
    pub approved_at: DateTime<Utc>,
}
```

### Configuration

```rust
// src/utils/config.rs
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub vcs: VcsConfig,
    pub bot: BotConfig,
    pub git: GitConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VcsConfig {
    pub platform: String,  // "github", "gitlab", etc.
    pub host: String,
    pub token: String,
    pub repository: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BotConfig {
    pub webhook_port: u16,
    pub queue_check_interval: u64,  // seconds
    pub ci_timeout: u64,  // seconds
    pub main_branch: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitConfig {
    pub default_remote: String,
    pub squash_base: String,  // usually "main" or "origin/main"
}
```

## Error Handling

### Error Types

```rust
// src/utils/error.rs
#[derive(Debug, thiserror::Error)]
pub enum GarryError {
    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),
    
    #[error("VCS adapter error: {0}")]
    VcsError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Review not found: {0}")]
    ReviewNotFound(ReviewId),
    
    #[error("Merge conflict: {0}")]
    MergeConflict(String),
    
    #[error("CI check failed: {0:?}")]
    CiFailed(Vec<String>),
    
    #[error("Invalid branch name: {0}")]
    InvalidBranchName(String),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, GarryError>;
```

### Error Recovery

- **Git errors**: Provide clear messages with suggested fixes
- **Network errors**: Implement retry logic with exponential backoff
- **Merge conflicts**: Remove from queue and notify developer with conflict details
- **CI failures**: Notify developer and prevent merge
- **Configuration errors**: Fail fast with clear validation messages

## Testing Strategy

### Unit Tests

Each module will have comprehensive unit tests:

```rust
// Example: src/git/branch.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_branch() {
        // Test branch creation
    }
    
    #[test]
    fn test_invalid_branch_name() {
        // Test validation
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
#[tokio::test]
async fn test_full_workflow() {
    // 1. Create mock Git repo
    // 2. Start feature branch
    // 3. Make commits
    // 4. Squash commits
    // 5. Upload review (mock adapter)
    // 6. Simulate approval
    // 7. Process merge queue
    // 8. Verify merge
}
```

### Mock Adapters

```rust
// tests/mocks/mock_adapter.rs
pub struct MockVcsAdapter {
    reviews: Arc<Mutex<HashMap<ReviewId, ReviewStatus>>>,
}

#[async_trait]
impl VcsAdapter for MockVcsAdapter {
    async fn create_review(&self, branch: &str, title: &str, description: &str) 
        -> Result<ReviewId> {
        // Mock implementation
    }
    // ... other methods
}
```

### Test Coverage Goals

- Unit test coverage: >80%
- Integration test coverage: All critical paths
- Mock all external dependencies (Git hosting platforms, CI systems)
- Test error conditions and edge cases

## Implementation Notes

### Dependencies

```toml
[dependencies]
git2 = "0.18"           # Git operations
tokio = { version = "1", features = ["full"] }  # Async runtime
reqwest = { version = "0.11", features = ["json"] }  # HTTP client
serde = { version = "1", features = ["derive"] }  # Serialization
serde_json = "1"        # JSON handling
clap = { version = "4", features = ["derive"] }  # CLI parsing
thiserror = "1"         # Error handling
anyhow = "1"            # Error context
tracing = "0.1"         # Logging
tracing-subscriber = "0.3"  # Log formatting
chrono = "0.4"          # Date/time handling
async-trait = "0.1"     # Async traits

[dev-dependencies]
tempfile = "3"          # Temporary directories for tests
mockito = "1"           # HTTP mocking
```

### Logging

Use `tracing` for structured logging:

```rust
use tracing::{info, warn, error, debug};

#[tracing::instrument]
pub async fn process_queue(&mut self) -> Result<()> {
    info!("Processing merge queue");
    // ... implementation
}
```

### Configuration Loading

Configuration will be loaded from:
1. `.garry/config.toml` in the repository root
2. `~/.config/garry/config.toml` for user-level defaults
3. Environment variables (prefixed with `GARRY_`)

### Cross-Platform Considerations

- Use `std::path::PathBuf` for all file paths
- Use `tokio::fs` for async file operations
- Test on Linux, macOS, and Windows in CI
- Handle line endings appropriately (CRLF vs LF)

### Security Considerations

- Store VCS tokens securely (use system keychain when possible)
- Validate all user inputs (branch names, commit messages)
- Use HTTPS for all VCS communication
- Implement rate limiting for API calls
- Sanitize error messages to avoid leaking sensitive information

## Deployment

### Garry CLI

- Distributed as a single binary
- Installation via package managers (cargo, homebrew, apt, etc.)
- Auto-update mechanism for new versions

### Garry Bot

- Deployed as a systemd service (Linux) or equivalent
- Docker container for easy deployment
- Kubernetes deployment for high availability
- Health check endpoints for monitoring
- Graceful shutdown handling

## Future Enhancements

- Web UI for queue visualization
- Metrics and analytics dashboard
- Support for stacked reviews
- Automatic conflict resolution for simple cases
- Integration with code review tools (ReviewBoard, Phabricator)
- Support for monorepo workflows with path-based reviews
