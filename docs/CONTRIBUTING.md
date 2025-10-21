# Contributing to Garry

We love contributions! Garry is built by developers, for developers.

## The Meta Way: Use Garry to Contribute to Garry

The best way to contribute to Garry is to use Garry itself! This ensures you experience the workflow firsthand and helps us dogfood our own tool.

## How to Contribute

### 1. Fork and Clone

```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/garry.git
cd garry
```

### 2. Install Garry

```bash
cargo build --release
cargo install --path .
```

### 3. Configure Garry

Create `.garry/config.toml` with your GitHub token and fork details:

```toml
[vcs]
platform = "github"
host = "github.com"
token = "your-github-token"
repository = "organisely/garry"

[git]
default_remote = "origin"
squash_base = "main"
```

### 4. Use Garry to Contribute!

```bash
# Start your feature
garry start feature/amazing-feature

# Make your changes
# ... edit files ...
git add .
git commit -m "Add amazing feature"

# Squash commits
garry squash

# Upload for review
garry upload --title "Add amazing feature" --description "This PR adds..."

# STOP HERE! Wait for maintainer/bot to merge
# (Don't run garry merge - that's for maintainers only! It will not work if you don't have the maintainer role.)
```

## What to Contribute

### Good First Issues
- Documentation improvements
- Bug fixes
- Test coverage
- Example configurations

### Bigger Features
- New VCS adapters (GitLab, Bitbucket)
- Webhook support
- Web UI
- Additional CLI commands

## Code Guidelines

- Write clean, idiomatic Rust
- Add tests for new features
- Update documentation
- Follow existing code style
- Keep commits atomic and well-described

## Testing

```bash
# Run tests
cargo test

# Run specific test
cargo test test_name

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

## Pull Request Process

1. Use Garry to create your PR (dogfooding!)
2. Ensure all tests pass
3. Update documentation if needed
4. Wait for review
5. Address feedback
6. Merge when approved

## Questions?

- Open an issue
- Start a discussion
- Check existing documentation

## Code of Conduct

Be respectful, constructive, and helpful. We're all here to build great tools together.

---

Thank you for contributing to Garry! 🎉
