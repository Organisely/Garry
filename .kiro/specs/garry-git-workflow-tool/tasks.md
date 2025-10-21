# Implementation Plan

- [x] 1. Set up project structure and core dependencies
  - Initialize Cargo project with workspace structure
  - Add all required dependencies to Cargo.toml (git2, tokio, clap, etc.)
  - Create module structure: cli/, bot/, git/, review/, utils/
  - Set up logging infrastructure with tracing
  - _Requirements: 11, 12_

- [x] 2. Implement configuration system
  - Create Config struct with VcsConfig, BotConfig, and GitConfig
  - Implement configuration loading from .garry/config.toml
  - Add support for user-level config at ~/.config/garry/config.toml
  - Implement environment variable overrides (GARRY_ prefix)
  - Add configuration validation
  - _Requirements: 6, 11_

- [x] 3. Implement error handling foundation
  - Create GarryError enum with all error variants
  - Implement Display and Error traits
  - Add error context helpers
  - Create Result type alias
  - _Requirements: 1, 2, 3, 4, 5, 12_

- [x] 4. Implement Git operations module
- [x] 4.1 Create GitRepository wrapper
  - Implement repository initialization and opening
  - Add current branch detection
  - Add commit counting functionality
  - _Requirements: 1, 2_

- [x] 4.2 Implement branch operations
  - Create branch creation with validation
  - Add branch name validation (format checking)
  - Implement branch switching
  - Add error handling for existing branches
  - _Requirements: 1_

- [x] 4.3 Implement commit squashing
  - Create squash_commits function with base branch parameter
  - Preserve commit messages during squash
  - Handle single-commit edge case
  - Add conflict detection and error reporting
  - _Requirements: 2_

- [x] 4.4 Implement remote operations
  - Add push_branch functionality
  - Implement remote configuration handling
  - Add authentication support
  - _Requirements: 3, 4_

- [ ]* 4.5 Write unit tests for Git operations
  - Test branch creation and validation
  - Test commit squashing with multiple scenarios
  - Test remote operations with mock repositories
  - _Requirements: 1, 2, 3_

- [x] 5. Implement CLI command structure
- [x] 5.1 Set up CLI parser with clap
  - Define Command enum (Start, Squash, Upload, Update, Merge)
  - Create CLI struct with argument parsing
  - Add help text and usage examples
  - _Requirements: 1, 2, 3, 4, 5_

- [x] 5.2 Implement 'garry start' command
  - Parse branch name argument
  - Call GitRepository::create_branch
  - Add logging for branch creation
  - Handle errors and provide user feedback
  - _Requirements: 1, 12_

- [x] 5.3 Implement 'garry squash' command
  - Detect current branch
  - Determine base branch from config
  - Call GitRepository::squash_commits
  - Provide feedback on squash result
  - _Requirements: 2, 12_

- [x] 5.4 Implement 'garry upload' command stub
  - Parse optional title and description
  - Push branch to remote
  - Prepare for VCS adapter integration (placeholder)
  - Add logging
  - _Requirements: 3, 12_

- [x] 5.5 Implement 'garry update' command stub
  - Detect existing review (placeholder)
  - Push updated commits
  - Prepare for VCS adapter integration (placeholder)
  - _Requirements: 4, 12_

- [x] 5.6 Implement 'garry merge' command stub
  - Prepare for bot communication (placeholder)
  - Add validation checks (placeholder)
  - _Requirements: 5, 12_

- [ ]* 5.7 Write integration tests for CLI commands
  - Test start command with valid and invalid branch names
  - Test squash command with multiple commits
  - Test error handling for each command
  - _Requirements: 1, 2, 3, 4, 5_

- [x] 6. Implement VCS adapter trait and registry
- [x] 6.1 Define VcsAdapter trait
  - Create async trait with all required methods
  - Define ReviewId, ReviewStatus, ReviewState types
  - Define CiStatus enum
  - Add documentation for each method
  - _Requirements: 6_

- [x] 6.2 Create AdapterRegistry
  - Implement HashMap-based adapter storage
  - Add register method for adding adapters
  - Add get_adapter method with host lookup
  - _Requirements: 6_

- [x] 6.3 Implement MockVcsAdapter for testing
  - Create in-memory review storage
  - Implement all VcsAdapter trait methods
  - Add helper methods for test setup
  - _Requirements: 6_

- [ ]* 6.4 Write unit tests for adapter registry
  - Test adapter registration and retrieval
  - Test missing adapter handling
  - _Requirements: 6_

- [x] 7. Implement GitHub adapter
- [x] 7.1 Create GithubAdapter struct
  - Add HTTP client with authentication
  - Store repository configuration
  - Implement rate limiting
  - _Requirements: 6_

- [x] 7.2 Implement create_review for GitHub
  - Create pull request via GitHub API
  - Handle API errors and rate limits
  - Return ReviewId from PR number
  - _Requirements: 3, 6_

- [x] 7.3 Implement get_review_status for GitHub
  - Fetch PR details and approval status
  - Parse review approvals
  - Check mergeable status
  - _Requirements: 6, 8_

- [x] 7.4 Implement merge_review for GitHub
  - Merge PR using squash merge
  - Handle merge conflicts
  - Verify merge success
  - _Requirements: 5, 6, 8_

- [x] 7.5 Implement post_comment for GitHub
  - Post comment to PR
  - Handle API errors
  - _Requirements: 6, 10_

- [x] 7.6 Implement list_pending_reviews for GitHub
  - List open PRs for repository
  - Filter by status
  - _Requirements: 6, 8_

- [x] 7.7 Implement get_ci_status for GitHub
  - Fetch check runs and statuses
  - Aggregate CI results
  - Map to CiStatus enum
  - _Requirements: 6, 9_

- [ ]* 7.8 Write integration tests for GitHub adapter
  - Use mockito to mock GitHub API
  - Test all adapter methods
  - Test error handling and rate limiting
  - _Requirements: 6_

- [x] 8. Integrate VCS adapter with CLI upload command
  - Load adapter from registry based on config
  - Call create_review when uploading
  - Display review URL to user
  - Handle adapter errors gracefully
  - _Requirements: 3, 6_

- [x] 9. Integrate VCS adapter with CLI update command
  - Detect existing review from branch
  - Call adapter to update review
  - Provide feedback to user
  - _Requirements: 4, 6_

- [x] 10. Implement commit queue manager
- [x] 10.1 Create QueueManager struct
  - Initialize with VecDeque for queue storage
  - Add VCS adapter reference
  - Create QueueEntry and QueueStatus types
  - _Requirements: 8_

- [x] 10.2 Implement add_to_queue
  - Validate review is approved
  - Verify CI status is passing
  - Add entry to queue
  - Log queue addition
  - _Requirements: 8, 9_

- [x] 10.3 Implement process_queue
  - Process queue entries in FIFO order
  - Check CI status before merging
  - Call adapter merge_review
  - Handle merge conflicts
  - Remove merged entries from queue
  - _Requirements: 8, 9_

- [x] 10.4 Implement remove_from_queue
  - Remove entry by review ID
  - Log removal reason
  - _Requirements: 8_

- [x] 10.5 Implement get_queue_position
  - Return position in queue for given review
  - _Requirements: 8_

- [ ]* 10.6 Write unit tests for queue manager
  - Test queue addition with validation
  - Test FIFO processing order
  - Test conflict handling
  - Test queue position tracking
  - _Requirements: 8_

- [x] 11. Implement CI monitoring
- [x] 11.1 Create CiMonitor struct
  - Add VCS adapter reference
  - Define CiStatus enum variants
  - _Requirements: 9_

- [x] 11.2 Implement check_ci_status
  - Call adapter get_ci_status
  - Parse and return status
  - _Requirements: 9_

- [x] 11.3 Implement wait_for_ci with timeout
  - Poll CI status at intervals
  - Respect timeout parameter
  - Return final status or timeout error
  - _Requirements: 9_

- [ ]* 11.4 Write unit tests for CI monitor
  - Test status checking
  - Test timeout behavior
  - Test status transitions
  - _Requirements: 9_

- [x] 12. Implement notification system
- [x] 12.1 Create Notifier struct
  - Add VCS adapter reference
  - Define NotificationType enum
  - _Requirements: 10_

- [x] 12.2 Implement notify method
  - Format notification message based on type
  - Call adapter post_comment
  - Log notification
  - _Requirements: 10_

- [x] 12.3 Add notification templates
  - Create message templates for each notification type
  - Include actionable information
  - _Requirements: 10_

- [ ]* 12.4 Write unit tests for notifier
  - Test each notification type
  - Verify message formatting
  - _Requirements: 10_

- [ ] 13. Implement Garry Bot service
- [x] 13.1 Create Bot struct
  - Initialize queue manager
  - Initialize CI monitor
  - Initialize notifier
  - Load configuration
  - _Requirements: 7, 8, 9, 10_

- [ ] 13.2 Implement webhook listener
  - Set up HTTP server with tokio
  - Parse webhook payloads
  - Route events to appropriate handlers
  - _Requirements: 8, 9_

- [ ] 13.3 Implement review approval handler
  - Detect approval events
  - Add approved reviews to queue
  - _Requirements: 8_

- [ ] 13.4 Implement CI status handler
  - Detect CI completion events
  - Update queue entry status
  - Trigger queue processing
  - _Requirements: 9_

- [x] 13.5 Implement queue processing loop
  - Run periodic queue checks
  - Process queue on events
  - Handle errors and retry logic
  - _Requirements: 8_

- [ ] 13.6 Implement main branch protection
  - Reject direct pushes to main
  - Provide helpful error messages
  - _Requirements: 7_

- [ ]* 13.7 Write integration tests for bot
  - Test webhook handling
  - Test queue processing flow
  - Test notification delivery
  - _Requirements: 7, 8, 9, 10_

- [x] 14. Implement CLI merge command integration
  - Send merge request to bot via adapter
  - Display queue position to user
  - Poll for merge status
  - Display final result
  - _Requirements: 5, 8_

- [x] 15. Add comprehensive logging
  - Add tracing spans to all major operations
  - Configure log levels from config
  - Add file logging output
  - Format logs for readability
  - _Requirements: 12_

- [x] 16. Implement cross-platform support
  - Use PathBuf for all file paths
  - Test on Linux, macOS, and Windows
  - Handle line ending differences
  - Use tokio::fs for async file operations
  - _Requirements: 11_

- [ ] 17. Create end-to-end integration test
  - Set up mock Git repository
  - Run full workflow: start → commit → squash → upload → approve → merge
  - Verify final state
  - Test error scenarios
  - _Requirements: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10_

- [x] 18. Add CLI binary and bot binary entry points
  - Create src/main.rs for CLI
  - Create src/bin/garry-bot.rs for bot service
  - Add proper argument parsing
  - Add version information
  - _Requirements: 1, 2, 3, 4, 5, 7, 8_

- [x] 19. Create example configuration files
  - Create .garry/config.toml.example
  - Document all configuration options
  - Add comments explaining each setting
  - _Requirements: 6, 11_

- [ ] 20. Implement additional VCS adapters (optional)
  - Create GitLabAdapter following GitHub pattern
  - Create BitbucketAdapter following GitHub pattern
  - Update adapter registry with new adapters
  - _Requirements: 6_
