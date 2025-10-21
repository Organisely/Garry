# Architecture

For detailed architecture documentation, see the root [ARCHITECTURE.md](../ARCHITECTURE.md) file.

## Quick Overview

Garry consists of:

1. **CLI Layer** - User commands (start, squash, upload, update, merge)
2. **Git Layer** - Git operations via git2-rs
3. **Adapter Layer** - VCS platform abstraction (GitHub, GitLab, etc.)
4. **Bot Layer** - Automation and queue management
5. **Review Layer** - Data models
6. **Utils Layer** - Configuration, errors, logging

## Data Flow

```
Developer → CLI → Git Operations → VCS Adapter → VCS Platform
                                                       ↓
                                                   Garry Bot
                                                       ↓
                                            Queue → CI → Merge
```

See [../ARCHITECTURE.md](../ARCHITECTURE.md) for complete diagrams and details.
