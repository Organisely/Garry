# Garry Quick Start

Get started with Garry's clean Git workflow in 5 minutes.

## Installation

Build from source:

```bash
git clone https://github.com/organisely/garry.git
cd garry
cargo build --release
cp target/release/garry /usr/local/bin/  # or add to PATH
```

## Configuration

Create `.garry/config.toml` in your repository root:

```toml
[vcs]
platform = "github"
host = "github.com"
token = "ghp_your_token_here"
repository = "owner/repo"

[git]
default_remote = "origin"
squash_base = "main"

[bot]
webhook_port = 8080
queue_check_interval = 30
ci_timeout = 3600
main_branch = "main"
```

Get a GitHub token at: https://github.com/settings/tokens

## Basic Workflow

### 1. Start a feature

```bash
garry start feature/my-feature
```

This creates and checks out a new branch from main.

### 2. Make your changes

```bash
# Edit files, make commits as usual
git add .
git commit -m "Add feature"
git commit -m "Fix typo"
git commit -m "Update tests"
```

### 3. Squash commits

```bash
garry squash
```

This combines all your commits into one clean commit.

### 4. Upload for review

```bash
garry upload --title "Add my feature"
```

This pushes your branch and creates a pull request.

### 5. Update after feedback

```bash
# Make more changes
git add .
git commit -m "Address review feedback"

# Squash again
garry squash

# Update the PR
garry update
```

## Commands Reference

**Developer commands:**
- `garry start <branch>` - Start a new feature branch
- `garry squash` - Squash all commits on current branch
- `garry upload` - Push branch and create review
- `garry update` - Update existing review with new changes

**Maintainer commands:**
- `garry approve <id>` - Approve a review (triggers bot merge)

## For Maintainers: Approving Reviews

Garry uses an automated merge queue. To approve a review:

**Option 1: Command line (recommended)**
```bash
garry approve 1
# or with a message
garry approve 1 --message "LGTM! Great work"
```

**Option 2: GitHub web UI**
1. Go to the PR page
2. Click "Files changed" tab
3. Click "Review changes" button
4. Select "Approve"
5. Click "Submit review"

**That's it!** The bot automatically merges approved reviews with passing CI.

The bot (`garry-bot`) runs continuously and:
- Monitors all open PRs every 30 seconds
- Automatically adds approved PRs (with passing CI) to the merge queue
- Merges them in order, one at a time
- Ensures no conflicts and CI stays green

**Never use GitHub's merge button** - Let the bot handle all merges to maintain clean history.

### Running the Bot

```bash
garry-bot
```

The bot needs to run continuously (use systemd, Docker, or a process manager).

## Tips

- Always squash before uploading to keep history clean
- Use descriptive branch names: `feature/`, `fix/`, `docs/`
- Let the bot handle merging - don't merge manually
- Keep commits atomic - one logical change per PR

## Next Steps

- [Architecture Overview](ARCHITECTURE.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Full Documentation](../README.md)
