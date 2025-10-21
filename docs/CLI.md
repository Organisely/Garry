# CLI Reference

Complete reference for all Garry CLI commands.

## Commands

### `garry start <branch-name>`

Create and checkout a new feature branch.

```bash
garry start feature/my-feature
```

**Options:**
- `<branch-name>` - Name of the branch to create (required)

**Example:**
```bash
garry start feature/user-authentication
garry start fix/bug-123
```

---

### `garry squash`

Squash all commits on the current branch into a single commit.

```bash
garry squash
```

**Behavior:**
- Combines all commits since the base branch
- Preserves all commit messages
- Creates a single clean commit
- Skips if only one commit exists

**Example:**
```bash
# After making multiple commits
git commit -m "Add feature"
git commit -m "Fix typo"
git commit -m "Add tests"

# Squash them all
garry squash
```

---

### `garry upload`

Upload changes for review (creates a PR/MR).

```bash
garry upload [OPTIONS]
```

**Options:**
- `--title, -t <title>` - Title for the review (optional)
- `--description, -d <description>` - Description for the review (optional)

**Example:**
```bash
garry upload --title "Add user authentication" --description "Implements login and signup"
garry upload  # Uses defaults
```

---

### `garry update`

Update an existing review with new commits.

```bash
garry update
```

**Behavior:**
- Pushes new commits to the existing review
- Re-triggers CI checks
- Preserves review conversation

**Example:**
```bash
# After addressing review feedback
git add .
git commit -m "Address feedback"
garry update
```

---

### `garry approve`

⚠️ **MAINTAINER/BOT ONLY** - This command should only be run by repository maintainers or the Garry Bot service.

Initiate merge via the bot queue.

```bash
garry approve
```

**Behavior:**
- Adds review to merge queue
- Bot processes queue automatically
- Requires approval and passing CI
- Enforces linear history
- Handles conflicts safely

**Who Should Use This:**
- ✅ Repository maintainers
- ✅ Garry Bot (automated)
- ❌ Regular developers (use `garry upload` instead)

**Why Restricted:**
- Protects main branch integrity
- Enforces merge queue order
- Maintains linear history
- Ensures accountability

**Example:**
```bash
# Maintainers only
garry approve
```
Note: It is usually fine to do this when testing or when there aren't that many developers working on your project, but this may be required if you work in a huge team.

---

## Global Options

All commands support:

```bash
--help, -h     Show help information
--version, -V  Show version information
```

## Environment Variables

- `RUST_LOG` - Set log level (debug, info, warn, error)
- `GARRY_VCS_TOKEN` - Override VCS token
- `GARRY_VCS_PLATFORM` - Override VCS platform
- `GARRY_VCS_HOST` - Override VCS host
- `GARRY_VCS_REPOSITORY` - Override repository

**Example:**
```bash
RUST_LOG=debug garry start feature/test
```

## Configuration

See [CONFIGURATION.md](CONFIGURATION.md) for details on configuring Garry.
