# Garry Project Structure

```
garry/
├── Cargo.toml                  # Project dependencies and configuration
├── README.md                   # Project documentation
├── STRUCTURE.md               # This file
├── .gitignore                 # Git ignore rules
│
├── .garry/
│   └── config.toml.example    # Example configuration file
│
├── src/
│   ├── main.rs                # CLI entry point
│   │
│   ├── bin/
│   │   └── garry-bot.rs       # Bot service entry point
│   │
│   ├── cli/                   # CLI commands
│   │   ├── mod.rs
│   │   ├── start.rs           # garry start
│   │   ├── squash.rs          # garry squash
│   │   ├── upload.rs          # garry upload
│   │   ├── update.rs          # garry update
│   │   └── merge.rs           # garry merge
│   │
│   ├── git/                   # Git operations (using git2-rs)
│   │   ├── mod.rs
│   │   ├── repo.rs            # Repository wrapper
│   │   ├── branch.rs          # Branch operations
│   │   ├── commit.rs          # Commit operations
│   │   └── remote.rs          # Remote operations
│   │
│   ├── review/                # Review data models
│   │   └── mod.rs             # ReviewId, ReviewStatus, CiStatus
│   │
│   └── utils/                 # Utilities
│       ├── mod.rs
│       ├── error.rs           # Error types
│       ├── config.rs          # Configuration structures
│       └── logger.rs          # Logging setup
│
└── tests/
    └── integration_test.rs    # Integration tests

## Modules to be added in future tasks:

src/bot/                       # Bot automation (Task 13+)
├── mod.rs
├── adapter.rs                 # VCS adapter trait
├── queue.rs                   # Merge queue manager
├── ci.rs                      # CI monitoring
└── notifier.rs                # Notifications

src/bot/adapters/              # VCS platform adapters (Task 7+)
├── github.rs                  # GitHub adapter
├── gitlab.rs                  # GitLab adapter
├── bitbucket.rs               # Bitbucket adapter
└── mock.rs                    # Mock adapter for testing
```

## Current Status

✅ Task 1 Complete: Project structure and core dependencies set up
- All module files created
- Error handling foundation in place
- Configuration system ready
- CLI command structure defined
- Git operations module scaffolded
- Review data models defined

## Next Steps

Task 2: Implement configuration loading system
Task 3: Complete error handling (already done!)
Task 4: Implement Git operations
Task 5: Implement CLI commands
Task 6+: VCS adapters and bot service
