# Garry Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         GARRY SYSTEM                             │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                      DEVELOPER WORKSTATION                       │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    Garry CLI (garry)                       │ │
│  │                                                            │ │
│  │  Commands:                                                 │ │
│  │  • garry start <branch>    → Create feature branch        │ │
│  │  • garry squash            → Squash commits               │ │
│  │  • garry upload            → Create review (PR/MR)        │ │
│  │  • garry update            → Update review                │ │
│  │  • garry merge             → Initiate merge               │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              ↓                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                   Git Operations (git2-rs)                 │ │
│  │                                                            │ │
│  │  • Branch creation & validation                           │ │
│  │  • Commit squashing with message preservation             │ │
│  │  • Remote push operations                                 │ │
│  │  • Repository management                                  │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              ↓
                         HTTP/HTTPS
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                    VCS PLATFORM (GitHub/GitLab/etc)              │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │              Pull Requests / Merge Requests                │ │
│  │                                                            │ │
│  │  • Review creation                                         │ │
│  │  • Approval tracking                                       │ │
│  │  • CI/CD status                                            │ │
│  │  • Comments & notifications                                │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              ↓
                      Webhooks/Polling
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│                    GARRY BOT SERVICE (garry-bot)                 │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    VCS Adapter Layer                       │ │
│  │                                                            │ │
│  │  ┌──────────┬──────────┬──────────┬──────────┬─────────┐  │ │
│  │  │ GitHub   │ GitLab   │Bitbucket │  Gitea   │  Mock   │  │ │
│  │  │ Adapter  │ Adapter  │ Adapter  │ Adapter  │ Adapter │  │ │
│  │  └──────────┴──────────┴──────────┴──────────┴─────────┘  │ │
│  │                                                            │ │
│  │  Unified Interface:                                        │ │
│  │  • create_review()                                         │ │
│  │  • get_review_status()                                     │ │
│  │  • merge_review()                                          │ │
│  │  • post_comment()                                          │ │
│  │  • list_pending_reviews()                                  │ │
│  │  • get_ci_status()                                         │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              ↓                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                   Commit Queue Manager                     │ │
│  │                                                            │ │
│  │  Queue: [Review1] → [Review2] → [Review3] → ...          │ │
│  │                                                            │ │
│  │  • FIFO processing order                                   │ │
│  │  • CI validation before merge                              │ │
│  │  • Conflict detection                                      │ │
│  │  • Automatic retries                                       │ │
│  │  • Status tracking                                         │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              ↓                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                      CI Monitor                            │ │
│  │                                                            │ │
│  │  • Check CI status                                         │ │
│  │  • Wait for completion                                     │ │
│  │  • Handle timeouts                                         │ │
│  │  • Track failures                                          │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              ↓                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    Notification System                     │ │
│  │                                                            │ │
│  │  • Merge success                                           │ │
│  │  • Merge conflicts                                         │ │
│  │  • CI failures                                             │ │
│  │  • Queue position                                          │ │
│  │  • Review updates needed                                   │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Module Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         GARRY MODULES                            │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│  CLI Layer (src/cli/)                                            │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  start.rs    → Create feature branches                     │ │
│  │  squash.rs   → Squash commits                              │ │
│  │  upload.rs   → Create reviews                              │ │
│  │  update.rs   → Update reviews                              │ │
│  │  merge.rs    → Initiate merge                              │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│  Git Layer (src/git/)                                            │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  repo.rs     → Repository wrapper                          │ │
│  │  branch.rs   → Branch operations & validation              │ │
│  │  commit.rs   → Commit squashing & counting                 │ │
│  │  remote.rs   → Push operations                             │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│  Bot Layer (src/bot/)                                            │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  adapter.rs  → VCS adapter trait & registry                │ │
│  │  adapters/   → Platform implementations                    │ │
│  │    ├── github.rs   → GitHub API integration                │ │
│  │    ├── mock.rs     → Testing adapter                       │ │
│  │    └── ...         → Other platforms                       │ │
│  │  queue.rs    → Merge queue management                      │ │
│  │  ci.rs       → CI status monitoring                        │ │
│  │  notifier.rs → Developer notifications                     │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│  Review Layer (src/review/)                                      │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  ReviewId       → Unique review identifier                 │ │
│  │  ReviewStatus   → Review state & metadata                  │ │
│  │  ReviewState    → Open/Approved/Merged/etc                 │ │
│  │  CiStatus       → Pending/Running/Success/Failed           │ │
│  │  Approval       → Reviewer approval data                   │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│  Utils Layer (src/utils/)                                        │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │  error.rs    → Error types & handling                      │ │
│  │  config.rs   → Configuration management                    │ │
│  │  logger.rs   → Logging setup                               │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Data Flow

### Developer Workflow

```
1. Start Feature
   Developer → garry start → Git → Create Branch
                                 ↓
                          Checkout Branch

2. Make Changes
   Developer → git commit → Local Commits
                          ↓
                    Multiple Commits

3. Squash Commits
   Developer → garry squash → Git → Combine Commits
                                  ↓
                           Single Clean Commit

4. Upload Review
   Developer → garry upload → Git → Push Branch
                           ↓
                      VCS Adapter → Create PR/MR
                                 ↓
                            Return Review ID

5. Update Review
   Developer → garry update → Git → Push Updates
                           ↓
                      VCS Adapter → Update PR/MR
                                 ↓
                          Trigger CI

6. Merge
   Developer → garry merge → Bot → Add to Queue
                                 ↓
                          Process Queue
```

### Bot Workflow

```
1. Monitor Reviews
   Bot → VCS Adapter → List Pending Reviews
                    ↓
              Check Each Review

2. Validate Review
   Bot → VCS Adapter → Get Review Status
                    ↓
              Check Approvals
                    ↓
              Check CI Status

3. Add to Queue
   Bot → Queue Manager → Validate
                      ↓
                 Add to Queue
                      ↓
              Update Position

4. Process Queue
   Bot → Queue Manager → Get First Entry
                      ↓
                 Check CI Status
                      ↓
                 Merge Review
                      ↓
              Remove from Queue

5. Notify Developer
   Bot → Notifier → Post Comment
                 ↓
          Send Notification
```

## Component Interactions

```
┌──────────────┐
│   Developer  │
└──────┬───────┘
       │
       ↓
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Garry CLI   │────→│ Git Layer    │────→│ Local Repo   │
└──────┬───────┘     └──────────────┘     └──────────────┘
       │
       ↓
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│ VCS Adapter  │────→│ HTTP Client  │────→│ VCS Platform │
└──────┬───────┘     └──────────────┘     └──────┬───────┘
       │                                          │
       │                                          │
       ↓                                          ↓
┌──────────────┐                          ┌──────────────┐
│  Garry Bot   │←─────────────────────────│  Webhooks    │
└──────┬───────┘                          └──────────────┘
       │
       ↓
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│Queue Manager │────→│  CI Monitor  │────→│  Notifier    │
└──────────────┘     └──────────────┘     └──────────────┘
```

## Configuration Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    Configuration Loading                         │
└─────────────────────────────────────────────────────────────────┘

1. Try Repository Config
   .garry/config.toml
        ↓
   [Found] → Load & Validate → Use
        ↓
   [Not Found]
        ↓
2. Try User Config
   ~/.config/garry/config.toml
        ↓
   [Found] → Load & Validate → Use
        ↓
   [Not Found]
        ↓
3. Environment Variables
   GARRY_VCS_TOKEN
   GARRY_VCS_PLATFORM
   GARRY_VCS_HOST
   GARRY_VCS_REPOSITORY
        ↓
   [Found] → Override Values → Use
        ↓
   [Not Found]
        ↓
4. Error
   No configuration found
```

## Error Handling Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                      Error Handling                              │
└─────────────────────────────────────────────────────────────────┘

Operation
    ↓
[Success] → Return Result
    ↓
[Error]
    ↓
Classify Error
    ├─→ GitError       → Log → Format Message → Return
    ├─→ VcsError       → Log → Format Message → Return
    ├─→ ConfigError    → Log → Format Message → Return
    ├─→ NetworkError   → Log → Retry? → Return
    └─→ Other          → Log → Format Message → Return
```

## Logging Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                         Logging                                  │
└─────────────────────────────────────────────────────────────────┘

Operation Start
    ↓
Log: INFO "Starting operation"
    ↓
Operation Steps
    ├─→ DEBUG "Step 1"
    ├─→ DEBUG "Step 2"
    └─→ DEBUG "Step 3"
    ↓
[Success]
    ↓
Log: INFO "Operation completed"
    ↓
[Error]
    ↓
Log: ERROR "Operation failed: {error}"
    ↓
Output to:
    ├─→ Console (formatted)
    └─→ File (structured)
```

## Deployment Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      Production Deployment                       │
└─────────────────────────────────────────────────────────────────┘

Developer Machines
    ├─→ garry CLI (installed via cargo)
    └─→ .garry/config.toml (local config)

Server/Cloud
    ├─→ garry-bot (systemd/docker/k8s)
    ├─→ Configuration (environment variables)
    ├─→ Logs (structured logging)
    └─→ Monitoring (health checks)

VCS Platform
    ├─→ GitHub/GitLab/etc
    ├─→ Webhooks (optional)
    └─→ API Access (required)
```

## Security Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Security                                 │
└─────────────────────────────────────────────────────────────────┘

Authentication
    ├─→ VCS Token (stored in config)
    ├─→ SSH Keys (for Git operations)
    └─→ HTTPS (for API calls)

Authorization
    ├─→ Repository access (via VCS)
    └─→ Branch protection (via VCS)

Data Protection
    ├─→ No sensitive data in logs
    ├─→ Secure credential storage
    └─→ Input validation

Network Security
    ├─→ HTTPS only
    ├─→ Certificate validation
    └─→ Rate limiting
```

## Performance Characteristics

```
┌─────────────────────────────────────────────────────────────────┐
│                       Performance                                │
└─────────────────────────────────────────────────────────────────┘

CLI Operations
    ├─→ start:  <100ms
    ├─→ squash: <1s (depends on commits)
    ├─→ upload: <2s (network dependent)
    ├─→ update: <2s (network dependent)
    └─→ merge:  <500ms

Bot Operations
    ├─→ Queue check: 30s interval (configurable)
    ├─→ CI check:    <500ms per review
    ├─→ Merge:       <2s per review
    └─→ Notify:      <500ms per notification

Resource Usage
    ├─→ CLI Memory:  <50MB
    ├─→ Bot Memory:  <100MB
    ├─→ CPU:         <5% average
    └─→ Network:     Minimal (API calls only)
```

---

This architecture provides a clean, modular, and extensible system for enforcing Git workflows across any VCS platform.
