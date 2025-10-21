use async_trait::async_trait;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use crate::bot::adapter::VcsAdapter;
use crate::review::{ReviewId, ReviewStatus, ReviewState, CiStatus, Approval};
use crate::utils::error::{GarryError, Result};
use chrono::{DateTime, Utc};
use tracing::{info, debug, warn};

/// GitHub adapter implementation
pub struct GithubAdapter {
    client: Client,
    host: String,
    token: String,
    repository: String,
}

impl GithubAdapter {
    /// Create a new GitHub adapter
    pub fn new(host: String, token: String, repository: String) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", token))
                .map_err(|e| GarryError::VcsError(format!("Invalid token: {}", e)))?,
        );
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("garry-bot"),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/vnd.github+json"),
        );
        
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| GarryError::NetworkError(e))?;
        
        Ok(Self {
            client,
            host,
            token,
            repository,
        })
    }
    
    fn api_url(&self, path: &str) -> String {
        if self.host == "github.com" {
            format!("https://api.github.com{}", path)
        } else {
            format!("https://{}/api/v3{}", self.host, path)
        }
    }
}

#[async_trait]
impl VcsAdapter for GithubAdapter {
    async fn create_review(&self, branch: &str, title: &str, description: &str) -> Result<(ReviewId, String)> {
        info!("Creating GitHub PR for branch: {}", branch);
        
        #[derive(Serialize)]
        struct CreatePrRequest {
            title: String,
            body: String,
            head: String,
            base: String,
        }
        
        #[derive(Deserialize)]
        struct PrResponse {
            number: u64,
            html_url: String,
        }
        
        // For same-repo PRs, head should just be the branch name
        // For cross-repo PRs, it should be "owner:branch"
        let head = if branch.contains(':') {
            branch.to_string()
        } else {
            // Extract owner from repository (format: "owner/repo")
            let owner = self.repository.split('/').next()
                .ok_or_else(|| GarryError::VcsError("Invalid repository format".to_string()))?;
            format!("{}:{}", owner, branch)
        };
        
        let request = CreatePrRequest {
            title: title.to_string(),
            body: description.to_string(),
            head,
            base: "main".to_string(),
        };
        
        let url = self.api_url(&format!("/repos/{}/pulls", self.repository));
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GarryError::VcsError(format!("Failed to create PR: {}", error_text)));
        }
        
        let pr: PrResponse = response.json().await?;
        info!("Created PR #{}: {}", pr.number, pr.html_url);
        
        Ok((ReviewId::new(pr.number.to_string()), pr.html_url))
    }
    
    async fn get_review_status(&self, review_id: &ReviewId) -> Result<ReviewStatus> {
        debug!("Getting status for PR #{}", review_id);
        
        #[derive(Deserialize)]
        struct PrDetails {
            state: String,
            mergeable: Option<bool>,
        }
        
        #[derive(Deserialize)]
        struct Review {
            user: User,
            state: String,
            submitted_at: Option<String>,
        }
        
        #[derive(Deserialize)]
        struct User {
            login: String,
        }
        
        // Get PR details
        let url = self.api_url(&format!("/repos/{}/pulls/{}", self.repository, review_id.as_str()));
        let pr: PrDetails = self.client.get(&url).send().await?.json().await?;
        
        // Get reviews
        let reviews_url = self.api_url(&format!("/repos/{}/pulls/{}/reviews", self.repository, review_id.as_str()));
        let reviews: Vec<Review> = self.client.get(&reviews_url).send().await?.json().await?;
        
        // Parse state
        let state = match pr.state.as_str() {
            "open" => {
                let has_approval = reviews.iter().any(|r| r.state == "APPROVED");
                let has_changes_requested = reviews.iter().any(|r| r.state == "CHANGES_REQUESTED");
                
                if has_changes_requested {
                    ReviewState::ChangesRequested
                } else if has_approval {
                    ReviewState::Approved
                } else {
                    ReviewState::Open
                }
            },
            "closed" => ReviewState::Closed,
            _ => ReviewState::Open,
        };
        
        // Parse approvals
        let approvals: Vec<Approval> = reviews
            .iter()
            .filter(|r| r.state == "APPROVED")
            .map(|r| Approval {
                reviewer: r.user.login.clone(),
                approved_at: r.submitted_at
                    .as_ref()
                    .and_then(|s| s.parse::<DateTime<Utc>>().ok())
                    .unwrap_or_else(Utc::now),
            })
            .collect();
        
        // Get CI status
        let ci_status = self.get_ci_status(review_id).await?;
        
        Ok(ReviewStatus {
            id: review_id.clone(),
            state,
            approvals,
            ci_status,
            mergeable: pr.mergeable.unwrap_or(false),
        })
    }
    
    async fn merge_review(&self, review_id: &ReviewId) -> Result<()> {
        info!("Merging PR #{}", review_id);
        
        #[derive(Serialize)]
        struct MergeRequest {
            merge_method: String,
        }
        
        let request = MergeRequest {
            merge_method: "squash".to_string(),
        };
        
        let url = self.api_url(&format!("/repos/{}/pulls/{}/merge", self.repository, review_id.as_str()));
        let response = self.client
            .put(&url)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GarryError::MergeConflict(format!("Failed to merge: {}", error_text)));
        }
        
        info!("Successfully merged PR #{}", review_id);
        Ok(())
    }
    
    async fn post_comment(&self, review_id: &ReviewId, message: &str) -> Result<()> {
        debug!("Posting comment to PR #{}", review_id);
        
        #[derive(Serialize)]
        struct CommentRequest {
            body: String,
        }
        
        let request = CommentRequest {
            body: message.to_string(),
        };
        
        let url = self.api_url(&format!("/repos/{}/issues/{}/comments", self.repository, review_id.as_str()));
        self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        Ok(())
    }
    
    async fn approve_review(&self, review_id: &ReviewId, message: Option<&str>) -> Result<()> {
        info!("Approving PR #{}", review_id);
        
        #[derive(Serialize)]
        struct ReviewRequest {
            body: String,
            event: String,
        }
        
        let request = ReviewRequest {
            body: message.unwrap_or("Approved").to_string(),
            event: "APPROVE".to_string(),
        };
        
        let url = self.api_url(&format!("/repos/{}/pulls/{}/reviews", self.repository, review_id.as_str()));
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GarryError::VcsError(format!("Failed to approve review: {}", error_text)));
        }
        
        info!("Successfully approved PR #{}", review_id);
        Ok(())
    }
    
    async fn list_pending_reviews(&self) -> Result<Vec<ReviewId>> {
        debug!("Listing pending PRs");
        
        #[derive(Deserialize)]
        struct PrListItem {
            number: u64,
        }
        
        let url = self.api_url(&format!("/repos/{}/pulls?state=open", self.repository));
        let prs: Vec<PrListItem> = self.client.get(&url).send().await?.json().await?;
        
        Ok(prs.into_iter().map(|pr| ReviewId::new(pr.number.to_string())).collect())
    }
    
    async fn get_ci_status(&self, review_id: &ReviewId) -> Result<CiStatus> {
        debug!("Getting CI status for PR #{}", review_id);
        
        #[derive(Deserialize)]
        struct PrDetails {
            head: Head,
        }
        
        #[derive(Deserialize)]
        struct Head {
            sha: String,
        }
        
        #[derive(Deserialize)]
        struct CheckRunsResponse {
            check_runs: Vec<CheckRun>,
        }
        
        #[derive(Deserialize)]
        struct CheckRun {
            name: String,
            status: String,
            conclusion: Option<String>,
        }
        
        // Get PR to find commit SHA
        let pr_url = self.api_url(&format!("/repos/{}/pulls/{}", self.repository, review_id.as_str()));
        let pr: PrDetails = self.client.get(&pr_url).send().await?.json().await?;
        
        // Get check runs for the commit
        let checks_url = self.api_url(&format!("/repos/{}/commits/{}/check-runs", self.repository, pr.head.sha));
        let checks: CheckRunsResponse = self.client.get(&checks_url).send().await?.json().await?;
        
        if checks.check_runs.is_empty() {
            return Ok(CiStatus::Pending);
        }
        
        let mut has_running = false;
        let mut failures = Vec::new();
        
        for check in &checks.check_runs {
            match check.status.as_str() {
                "completed" => {
                    if let Some(conclusion) = &check.conclusion {
                        match conclusion.as_str() {
                            "success" => continue,
                            "failure" | "timed_out" => failures.push(check.name.clone()),
                            "cancelled" => return Ok(CiStatus::Cancelled),
                            _ => {}
                        }
                    }
                },
                "in_progress" | "queued" => has_running = true,
                _ => {}
            }
        }
        
        if !failures.is_empty() {
            Ok(CiStatus::Failed(failures))
        } else if has_running {
            Ok(CiStatus::Running)
        } else {
            Ok(CiStatus::Success)
        }
    }
    
    async fn setup_repository_protection(&self, main_branch: &str, bot_user: &str) -> Result<()> {
        info!("Setting up repository protection for branch: {}", main_branch);
        
        #[derive(Serialize)]
        struct ProtectionRequest {
            required_status_checks: Option<StatusChecks>,
            enforce_admins: bool,
            required_pull_request_reviews: PullRequestReviews,
            restrictions: Option<Restrictions>,
        }
        
        #[derive(Serialize)]
        struct StatusChecks {
            strict: bool,
            contexts: Vec<String>,
        }
        
        #[derive(Serialize)]
        struct PullRequestReviews {
            dismissal_restrictions: Option<DismissalRestrictions>,
            dismiss_stale_reviews: bool,
            require_code_owner_reviews: bool,
            required_approving_review_count: u32,
        }
        
        #[derive(Serialize)]
        struct DismissalRestrictions {
            users: Vec<String>,
            teams: Vec<String>,
        }
        
        #[derive(Serialize)]
        struct Restrictions {
            users: Vec<String>,
            teams: Vec<String>,
            apps: Vec<String>,
        }
        
        let protection = ProtectionRequest {
            required_status_checks: Some(StatusChecks {
                strict: true,
                contexts: vec![],
            }),
            enforce_admins: false,
            required_pull_request_reviews: PullRequestReviews {
                dismissal_restrictions: None,
                dismiss_stale_reviews: true,
                require_code_owner_reviews: false,
                required_approving_review_count: 1,
            },
            restrictions: Some(Restrictions {
                users: vec![bot_user.to_string()],
                teams: vec![],
                apps: vec![],
            }),
        };
        
        let url = self.api_url(&format!("/repos/{}/branches/{}/protection", self.repository, main_branch));
        let response = self.client
            .put(&url)
            .json(&protection)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            warn!("Failed to set up branch protection (may need admin permissions): {}", error_text);
        } else {
            info!("Successfully set up branch protection for {}", main_branch);
        }
        
        Ok(())
    }
    
    fn review_name(&self) -> &str {
        "Pull Request"
    }
}
