# Configuration Guide

Garry uses TOML configuration files to manage settings.

## Configuration File Locations

Garry looks for configuration in this order:

1. `.garry/config.toml` (repository-level)
2. `~/.config/garry/config.toml` (user-level)
3. Environment variables

## Basic Configuration

Create `.garry/config.toml` in your repository root:

```toml
[vcs]
platform = "github"
host = "github.com"
token = "your-token-here"
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

## Configuration Sections

### VCS Configuration

```toml
[vcs]
# Platform type: "github", "gitlab", "bitbucket", "gitea"
platform = "github"

# VCS host (e.g., "github.com" or your custom domain)
host = "github.com"

# Authentication token (keep this secret!)
token = "ghp_your_token_here"

# Repository in format "owner/repo"
repository = "organisely/your-repo"
```

### Bot Configuration

```toml
[bot]
# Port for webhook listener
webhook_port = 8080

# How often to check the merge queue (in seconds)
queue_check_interval = 30

# Maximum time to wait for CI (in seconds)
ci_timeout = 3600

# Name of the main/protected branch
main_branch = "main"
```

### Git Configuration

```toml
[git]
# Default remote name (usually "origin")
default_remote = "origin"

# Base branch for squashing (usually "main" or "origin/main")
squash_base = "main"
```

## Environment Variables

Override configuration with environment variables:

```bash
export GARRY_VCS_TOKEN="your-token"
export GARRY_VCS_PLATFORM="github"
export GARRY_VCS_HOST="github.com"
export GARRY_VCS_REPOSITORY="owner/repo"
```

## Getting a GitHub Token

1. Go to GitHub Settings → Developer settings → Personal access tokens
2. Generate new token (classic)
3. Select scopes: `repo`, `workflow`
4. Copy the token and add it to your config

## Example Configurations

### GitHub

```toml
[vcs]
platform = "github"
host = "github.com"
token = "ghp_xxxxxxxxxxxx"
repository = "organisely/garry"
```

### GitLab

```toml
[vcs]
platform = "gitlab"
host = "gitlab.com"
token = "glpat-xxxxxxxxxxxx"
repository = "organisely/garry"
```

### Self-Hosted

```toml
[vcs]
platform = "github"
host = "git.company.com"
token = "your-token"
repository = "team/project"
```

## Security

- Never commit your token to Git
- Add `.garry/config.toml` to `.gitignore`
- Use environment variables in CI/CD
- Rotate tokens regularly

## Validation

Garry validates your configuration on startup. Common errors:

- **Empty token**: Add your VCS token
- **Invalid repository format**: Use "owner/repo" format
- **Missing configuration**: Create `.garry/config.toml`

## Next Steps

- [Quick Start Guide](QUICKSTART.md)
- [CLI Reference](CLI.md)
- [Bot Service](BOT.md)
