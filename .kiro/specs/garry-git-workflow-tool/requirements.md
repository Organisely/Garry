# Requirements Document

## Introduction

Garry is a Rust-based Git workflow enforcement tool for Organisely that implements a clean, squash-first workflow similar to Chromium's Gerrit system. The tool consists of two main components: a CLI for local developer operations and a Bot for automation and merge queue management. Garry is designed to be VCS-agnostic, supporting multiple Git hosting platforms through a unified adapter interface.

## Requirements

### Requirement 1: Branch Management

**User Story:** As a developer, I want to start feature branches through Garry CLI, so that I can work on features in isolation with proper tracking.

#### Acceptance Criteria

1. WHEN a developer executes `garry start <branch-name>` THEN the system SHALL create a new Git branch locally
2. WHEN a branch is created THEN the system SHALL validate the branch name format
3. WHEN a branch is created THEN the system SHALL log the branch creation action
4. IF a branch with the same name already exists THEN the system SHALL return an error message

### Requirement 2: Commit Squashing

**User Story:** As a developer, I want to squash my local commits before review, so that my change history is clean and atomic.

#### Acceptance Criteria

1. WHEN a developer executes `garry squash` THEN the system SHALL combine all commits on the current branch into a single commit
2. WHEN commits are squashed THEN the system SHALL preserve the commit messages for reference
3. WHEN squashing fails due to conflicts THEN the system SHALL provide clear error messages
4. IF there is only one commit THEN the system SHALL inform the user that squashing is not needed

### Requirement 3: Review Upload

**User Story:** As a developer, I want to upload my changes for review through Garry CLI, so that my code can be reviewed before merging.

#### Acceptance Criteria

1. WHEN a developer executes `garry upload` THEN the system SHALL push the branch to the remote repository
2. WHEN uploading THEN the system SHALL create a review request through the VCS adapter
3. WHEN a review is created THEN the system SHALL return the review ID to the developer
4. IF the upload fails THEN the system SHALL provide actionable error messages
5. WHEN uploading THEN the system SHALL automatically trigger CI/CD pipelines

### Requirement 4: Review Updates

**User Story:** As a developer, I want to update existing reviews with new changes, so that I can address reviewer feedback.

#### Acceptance Criteria

1. WHEN a developer executes `garry update` THEN the system SHALL push updated commits to the existing review
2. WHEN updating THEN the system SHALL preserve the review ID and conversation history
3. WHEN an update is pushed THEN the system SHALL re-trigger CI/CD pipelines
4. IF no existing review is found THEN the system SHALL prompt the user to upload instead

### Requirement 5: Merge Initiation

**User Story:** As a developer, I want to initiate a merge through Garry CLI, so that my approved changes can be merged to main.

#### Acceptance Criteria

1. WHEN a developer executes `garry merge` THEN the system SHALL communicate with Garry Bot to queue the merge
2. WHEN a merge is initiated THEN the system SHALL verify that the review is approved
3. WHEN a merge is initiated THEN the system SHALL verify that CI checks have passed
4. IF merge conditions are not met THEN the system SHALL inform the user of missing requirements

### Requirement 6: VCS Adapter Interface

**User Story:** As a system administrator, I want Garry to work with multiple Git hosting platforms, so that we can use it regardless of our VCS choice.

#### Acceptance Criteria

1. WHEN Garry interacts with a VCS THEN it SHALL use the unified VcsAdapter trait interface
2. WHEN an adapter is needed THEN the system SHALL select the appropriate adapter based on configuration
3. THE system SHALL support GitHub, GitLab, Bitbucket, Gitea, and self-hosted Git adapters
4. WHEN an adapter operation fails THEN the system SHALL return standardized error types
5. EACH adapter SHALL implement create_review, get_review_status, merge_review, post_comment, and list_pending_reviews methods

### Requirement 7: Main Branch Protection

**User Story:** As a team lead, I want to prevent direct pushes to main, so that all changes go through the review process.

#### Acceptance Criteria

1. WHEN Garry Bot is configured THEN it SHALL enforce that main branch is protected
2. WHEN a direct push to main is attempted THEN the system SHALL reject the push
3. WHEN a push is rejected THEN the system SHALL provide guidance on using the Garry workflow
4. THE system SHALL allow only Garry Bot to push to main after merge queue processing

### Requirement 8: Commit Queue Management

**User Story:** As a team lead, I want approved changes to be merged in order through a commit queue, so that we maintain linear history and prevent conflicts.

#### Acceptance Criteria

1. WHEN a review is approved and CI passes THEN Garry Bot SHALL add it to the commit queue
2. WHEN processing the queue THEN the system SHALL merge changes in FIFO order
3. WHEN a merge conflict occurs THEN the system SHALL remove the change from queue and notify the developer
4. WHEN a change is merged THEN the system SHALL update the queue and process the next item
5. THE system SHALL enforce linear history on the main branch

### Requirement 9: CI/CD Integration

**User Story:** As a developer, I want CI/CD pipelines to run automatically on my reviews, so that I know my changes pass all checks.

#### Acceptance Criteria

1. WHEN a review is uploaded THEN the system SHALL trigger configured CI/CD pipelines
2. WHEN a review is updated THEN the system SHALL re-trigger CI/CD pipelines
3. WHEN CI status changes THEN Garry Bot SHALL track the updated status
4. WHEN all CI checks pass THEN the system SHALL mark the review as ready for merge
5. IF any CI check fails THEN the system SHALL prevent merging and notify the developer

### Requirement 10: Developer Notifications

**User Story:** As a developer, I want to receive notifications about my review status, so that I stay informed about merge progress and issues.

#### Acceptance Criteria

1. WHEN a review is merged THEN the system SHALL notify the developer
2. WHEN a merge fails due to conflicts THEN the system SHALL notify the developer with details
3. WHEN CI checks fail THEN the system SHALL notify the developer
4. WHEN a review requires updates THEN the system SHALL notify the developer
5. NOTIFICATIONS SHALL be delivered through the VCS platform's native notification system

### Requirement 11: Cross-Platform Support

**User Story:** As a developer, I want to use Garry on any operating system, so that the entire team can use the same workflow regardless of their OS.

#### Acceptance Criteria

1. THE Garry CLI SHALL run on Linux, macOS, and Windows
2. THE Garry Bot SHALL run on Linux, macOS, and Windows
3. WHEN executing Git operations THEN the system SHALL use cross-platform compatible libraries
4. WHEN handling file paths THEN the system SHALL use OS-agnostic path handling

### Requirement 12: Logging and Transparency

**User Story:** As a developer and administrator, I want all Garry operations to be logged, so that I can debug issues and audit workflow actions.

#### Acceptance Criteria

1. WHEN any CLI command is executed THEN the system SHALL log the action with timestamp
2. WHEN Garry Bot performs any operation THEN it SHALL log the action with timestamp
3. WHEN errors occur THEN the system SHALL log detailed error information
4. THE system SHALL support configurable log levels (debug, info, warn, error)
5. LOGS SHALL be written to both console and file outputs
