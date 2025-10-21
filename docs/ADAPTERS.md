# VCS Adapters

Garry is platform-agnostic and works with any Git hosting platform through adapters.

## What are Adapters?

Adapters are platform-specific implementations that allow Garry to work with different VCS platforms (GitHub, GitLab, Bitbucket, etc.) through a unified interface.

## Supported Platforms

| Platform | Status | Review Name | Adapter |
|----------|--------|-------------|---------|
| GitHub | ✅ Fully Implemented | Pull Request (PR) | `GithubAdapter` |
| GitLab | 🚧 Architecture Ready | Merge Request (MR) | `GitLabAdapter` |
| Bitbucket | 🚧 Architecture Ready | Pull Request (PR) | `BitbucketAdapter` |
| Gitea | 🚧 Architecture Ready | Pull Request (PR) | `GiteaAdapter` |
| Self-Hosted | 🚧 Architecture Ready | Review | `SelfHostedAdapter` |

## How Adapters Work

### Unified Interface

All adapters implement the `VcsAdapter` trait:

```rust
#[async_trait]
pub trait VcsAdapter {
    // Create a review and return (ID, URL)
    async fn create_review(&self, branch: &str, title: &str, description: &str) 
        -> Result<(ReviewId, String)>;
    
    // Get review status
    async fn get_review_status(&self, review_id: &ReviewId) 
        -> Result<ReviewStatus>;
    
    // Merge a review (Bot only!)
    async fn merge_review(&self, review_id: &ReviewId) 
        -> Result<()>;
    
    // Post a comment
    async fn post_comment(&self, review_id: &ReviewId, message: &str) 
        -> Result<()>;
    
    // List pending reviews
    async fn list_pending_reviews(&self) 
        -> Result<Vec<ReviewId>>;
    
    // Get CI status
    async fn get_ci_status(&self, review_id: &ReviewId) 
        -> Result<CiStatus>;
    
    // Setup repository protection (Bot only!)
    async fn setup_repository_protection(&self, main_branch: &str, bot_user: &str) 
        -> Result<()>;
    
    // Get platform-specific review name
    fn review_name(&self) -> &str;
}
```

### Platform-Specific Features

Each adapter handles platform-specific details:

#### GitHub Adapter
- **Review Name**: "Pull Request" (PR)
- **API**: GitHub REST API v3
- **Protection**: Branch protection rules
- **CI**: GitHub Actions, Check Runs
- **URL Format**: `https://github.com/owner/repo/pull/123`

#### GitLab Adapter (Coming Soon)
- **Review Name**: "Merge Request" (MR)
- **API**: GitLab REST API v4
- **Protection**: Protected branches
- **CI**: GitLab CI/CD
- **URL Format**: `https://gitlab.com/owner/repo/-/merge_requests/123`

#### Bitbucket Adapter (Coming Soon)
- **Review Name**: "Pull Request" (PR)
- **API**: Bitbucket REST API 2.0
- **Protection**: Branch permissions
- **CI**: Bitbucket Pipelines
- **URL Format**: `https://bitbucket.org/owner/repo/pull-requests/123`

## Repository Protection

### What Garry Bot Sets Up

When Garry Bot starts, it automatically configures repository protection:

1. **Branch Protection**
   - Protects the main branch
   - Requires PR reviews
   - Requires CI checks to pass
   - Prevents direct pushes

2. **Push Restrictions**
   - Only Garry Bot can push to main
   - All other users must use PRs
   - Enforced via platform APIs

3. **Review Requirements**
   - At least 1 approval required
   - Stale reviews dismissed on new commits
   - CI must pass before merge

4. **Merge Queue**
   - FIFO processing
   - Automatic conflict detection
   - Linear history enforcement

### GitHub Protection Example

```json
{
  "required_status_checks": {
    "strict": true,
    "contexts": []
  },
  "enforce_admins": false,
  "required_pull_request_reviews": {
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": false,
    "required_approving_review_count": 1
  },
  "restrictions": {
    "users": ["garry-bot"],
    "teams": [],
    "apps": []
  }
}
```

## Review URLs

When you upload a review, Garry shows you the platform-specific URL:

### GitHub Example
```bash
$ garry upload --title "Add feature"
Pushing branch 'feature/my-feature' to remote...
Creating Pull Request...
✓ Successfully created Pull Request!
  Branch: feature/my-feature
  Pull Request ID: #42
  Pull Request URL: https://github.com/organisely/garry/pull/42
```

### GitLab Example (Coming Soon)
```bash
$ garry upload --title "Add feature"
Pushing branch 'feature/my-feature' to remote...
Creating Merge Request...
✓ Successfully created Merge Request!
  Branch: feature/my-feature
  Merge Request ID: #42
  Merge Request URL: https://gitlab.com/organisely/garry/-/merge_requests/42
```

## How Merging Works

### Important: Bot Token vs User Token

- **User Token**: Used for `garry upload`, `garry update` (creating/updating reviews)
- **Bot Token**: Used for `garry merge` (merging to main)

### The Flow

1. **Developer** runs `garry upload`
   - Uses developer's token
   - Creates PR/MR
   - Returns review URL

2. **Maintainer** approves review
   - Via platform UI
   - Or via `garry merge` command

3. **Garry Bot** merges
   - Uses bot's token (NOT user token!)
   - Bot has special permission to push to main
   - All other users are blocked from pushing to main

### Why This Matters

```
┌─────────────────────────────────────────────────────────┐
│                  TOKEN SEPARATION                        │
└─────────────────────────────────────────────────────────┘

Developer Token                    Bot Token
      │                                │
      ├─ garry upload                 │
      ├─ garry update                 │
      │                                │
      ❌ Cannot push to main           ✅ Can push to main
      ❌ Cannot merge PRs              ✅ Can merge PRs
      ✅ Can create PRs                ✅ Can create PRs
      ✅ Can update PRs                ✅ Can update PRs
```

## Adapter Configuration

### Selecting an Adapter

In `.garry/config.toml`:

```toml
[vcs]
platform = "github"  # or "gitlab", "bitbucket", etc.
host = "github.com"
token = "your-token"
repository = "owner/repo"
```

### Platform-Specific Settings

#### GitHub
```toml
[vcs]
platform = "github"
host = "github.com"  # or "github.enterprise.com"
token = "ghp_xxxxxxxxxxxx"
repository = "organisely/garry"
```

#### GitLab (Coming Soon)
```toml
[vcs]
platform = "gitlab"
host = "gitlab.com"  # or "gitlab.company.com"
token = "glpat-xxxxxxxxxxxx"
repository = "organisely/garry"
```

#### Bitbucket (Coming Soon)
```toml
[vcs]
platform = "bitbucket"
host = "bitbucket.org"
token = "your-app-password"
repository = "organisely/garry"
```

## Creating a New Adapter

Want to add support for a new platform? Here's how:

### 1. Implement the Trait

```rust
pub struct MyPlatformAdapter {
    client: Client,
    host: String,
    token: String,
    repository: String,
}

#[async_trait]
impl VcsAdapter for MyPlatformAdapter {
    async fn create_review(&self, branch: &str, title: &str, description: &str) 
        -> Result<(ReviewId, String)> {
        // Call your platform's API
        // Return (review_id, review_url)
    }
    
    // Implement other methods...
    
    fn review_name(&self) -> &str {
        "Pull Request"  // or "Merge Request", etc.
    }
}
```

### 2. Add to Registry

```rust
// In src/bot/adapters/mod.rs
pub mod myplatform;
pub use myplatform::MyPlatformAdapter;
```

### 3. Update Configuration

```rust
// In CLI/Bot code
let adapter: Box<dyn VcsAdapter> = match config.vcs.platform.as_str() {
    "github" => Box::new(GithubAdapter::new(...)?),
    "myplatform" => Box::new(MyPlatformAdapter::new(...)?),
    _ => return Err(GarryError::VcsError(...)),
};
```

### 4. Test

```rust
#[tokio::test]
async fn test_myplatform_adapter() {
    let adapter = MyPlatformAdapter::new(...);
    let (id, url) = adapter.create_review("test", "Test", "Description").await.unwrap();
    assert!(!url.is_empty());
}
```

## Adapter Best Practices

### Error Handling
- Return clear error messages
- Handle rate limiting
- Retry on network errors
- Log all API calls

### Authentication
- Support token-based auth
- Validate tokens on startup
- Handle expired tokens gracefully

### URLs
- Always return full URLs
- Use platform's canonical format
- Include protocol (https://)

### Review Names
- Use platform's terminology
- Be consistent (PR vs Pull Request)
- Document in adapter

### Protection Setup
- Make it idempotent
- Handle missing permissions gracefully
- Log what was configured
- Provide manual setup instructions if needed

## Troubleshooting

### "Unsupported platform"
- Check `platform` in config
- Verify adapter is implemented
- Check spelling

### "Failed to set up protection"
- Bot needs admin permissions
- Configure manually via platform UI
- Check bot token permissions

### "Review URL not shown"
- Adapter may not support URLs
- Check adapter implementation
- File an issue

## Next Steps

- [Configuration Guide](CONFIGURATION.md)
- [Bot Service](BOT.md)
- [Contributing](CONTRIBUTING.md)
