# Quick Start Guide

Get up and running with Garry in 5 minutes!

## Step 1: Install Garry

```bash
git clone https://github.com/organisely/garry.git
cd garry
./quickstart.sh
```

## Step 2: Configure

Create `.garry/config.toml` in your repository:

```toml
[vcs]
platform = "github"
host = "github.com"
token = "your-github-token"
repository = "owner/repo"

[bot]
webhook_port = 8080
queue_check_interval = 30
ci_timeout = 3600
main_branch = "main"

[git]
default_remote = "origin"
squash_base = "main"
```

## Step 3: Your First Feature

```bash
# Start a feature branch
garry start feature/my-first-feature

# Make some changes
echo "Hello Garry!" > test.txt
git add test.txt
git commit -m "Add test file"

# Squash commits (if you made multiple)
garry squash

# Upload for review
garry upload --title "My first feature" --description "Testing Garry"

# Wait for maintainer/bot to merge automatically
# (Don't run garry merge yourself!)
```

## Step 4: Run the Bot (Optional)

In a separate terminal:

```bash
garry-bot
```

The bot will automatically manage the merge queue!

## Next Steps

- [Full CLI Reference](CLI.md)
- [Workflow Best Practices](WORKFLOW.md)
- [Bot Configuration](BOT.md)
