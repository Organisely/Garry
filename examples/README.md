# Garry Configuration Examples

This directory contains example configurations for different scenarios.

## Quick Start

### For organisely/garry Repository

```bash
# Copy the example config
cp examples/config.toml .garry/config.toml

# Edit with your GitHub token
nano .garry/config.toml
```

### Get Your GitHub Token

1. Go to https://github.com/settings/tokens
2. Click "Generate new token (classic)"
3. Give it a name: "Garry Bot"
4. Select scopes:
   - ✅ `repo` (Full control of private repositories)
   - ✅ `workflow` (Update GitHub Action workflows)
   - ✅ `admin:repo_hook` (Full control of repository hooks)
5. Click "Generate token"
6. Copy the token (starts with `ghp_`)
7. Paste it in your `.garry/config.toml`

### Test Your Configuration

```bash
# Try running the CLI
cargo run -- start feature/test

# Try running the bot
cargo run --bin garry-bot
```

## Configuration Options

### VCS Platforms

#### GitHub (Public)
```toml
[vcs]
platform = "github"
host = "github.com"
token = "ghp_your_token"
repository = "organisely/garry"
```

#### GitHub Enterprise
```toml
[vcs]
platform = "github"
host = "github.company.com"
token = "ghp_your_token"
repository = "company/repo"
```

#### GitLab (Coming Soon)
```toml
[vcs]
platform = "gitlab"
host = "gitlab.com"
token = "glpat_your_token"
repository = "organisely/garry"
```

### Bot Settings

#### Fast Queue Processing
```toml
[bot]
queue_check_interval = 15  # Check every 15 seconds
ci_timeout = 1800           # 30 minute timeout
```

#### Conservative (Rate Limit Friendly)
```toml
[bot]
queue_check_interval = 60  # Check every minute
ci_timeout = 7200           # 2 hour timeout
```

### Git Settings

#### Using Develop Branch
```toml
[git]
default_remote = "origin"
squash_base = "develop"

[bot]
main_branch = "develop"
```

## Environment Variables

You can also configure via environment variables:

```bash
export GARRY_VCS_TOKEN="ghp_your_token"
export GARRY_VCS_PLATFORM="github"
export GARRY_VCS_HOST="github.com"
export GARRY_VCS_REPOSITORY="organisely/garry"
```

## Security

⚠️ **Never commit your token to Git!**

The `.garry/config.toml` file is already in `.gitignore`.

### Secure Token Storage

**Option 1: Environment Variables**
```bash
# In your shell profile (~/.bashrc, ~/.zshrc)
export GARRY_VCS_TOKEN="ghp_your_token"
```

**Option 2: Separate Config File**
```bash
# Store token separately
echo 'token = "ghp_your_token"' > ~/.garry-token.toml

# Reference in main config
# (Note: This requires code changes to support)
```

**Option 3: Secret Manager**
- AWS Secrets Manager
- HashiCorp Vault
- 1Password CLI
- etc.

## Troubleshooting

### "No configuration file found"

```bash
# Check if file exists
ls -la .garry/config.toml

# If not, copy from example
cp examples/config.toml .garry/config.toml
```

### "Invalid token"

```bash
# Test your token
curl -H "Authorization: token ghp_your_token" https://api.github.com/user

# If it fails, generate a new token
```

### "Repository not found"

```bash
# Check repository format (must be "owner/repo")
# Correct: "organisely/garry"
# Wrong: "https://github.com/organisely/garry"
# Wrong: "garry"
```

## Next Steps

- [Configuration Guide](../docs/CONFIGURATION.md)
- [Deployment Guide](../docs/DEPLOYMENT.md)
- [Quick Start](../docs/QUICKSTART.md)
