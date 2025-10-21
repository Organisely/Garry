mod start;
mod squash;
mod upload;
mod update;
mod approve;

use clap::Subcommand;
use crate::utils::error::Result;

#[derive(Subcommand)]
pub enum Command {
    /// Start a new feature branch
    Start {
        /// Name of the branch to create
        branch_name: String,
    },
    /// Squash commits on the current branch
    Squash,
    /// Upload changes for review
    Upload {
        /// Optional title for the review
        #[arg(short, long)]
        title: Option<String>,
        /// Optional description for the review
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Update an existing review with new changes
    Update,
    /// Approve a review (maintainers only)
    Approve {
        /// Review ID to approve
        review_id: String,
        /// Optional approval message
        #[arg(short, long)]
        message: Option<String>,
    },
}

/// Execute a CLI command
pub async fn execute(command: Command) -> Result<()> {
    match command {
        Command::Start { branch_name } => start::execute(&branch_name).await,
        Command::Squash => squash::execute().await,
        Command::Upload { title, description } => upload::execute(title, description).await,
        Command::Update => update::execute().await,
        Command::Approve { review_id, message } => approve::execute(review_id, message).await,
    }
}
