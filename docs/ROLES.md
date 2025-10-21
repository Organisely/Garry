# Roles and Responsibilities

Clear separation of roles ensures Garry maintains a clean, safe workflow.

## Role Overview

```
┌─────────────────────────────────────────────────────────────┐
│                         ROLES                                │
└─────────────────────────────────────────────────────────────┘

┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  Developer   │     │  Maintainer  │     │  Garry Bot   │
│              │     │              │     │              │
│ Feature Work │     │ Code Review  │     │ Automation   │
│ PR Creation  │     │ Merge Control│     │ Queue Mgmt   │
└──────────────┘     └──────────────┘     └──────────────┘
```

## Developer Role

**Who:** Regular contributors, team members

**Permissions:**
- ✅ Create branches
- ✅ Make commits
- ✅ Upload PRs
- ✅ Update PRs
- ❌ Merge to main

**Commands:**
```bash
garry start      # ✅ Create feature branch
garry squash     # ✅ Squash commits
garry upload     # ✅ Create PR
garry update     # ✅ Update PR
garry merge      # ❌ NOT ALLOWED
```

**Workflow:**
1. Start feature branch
2. Make commits
3. Squash commits
4. Upload for review
5. Address feedback
6. **STOP** - Wait for merge

**Responsibilities:**
- Write clean code
- Test changes
- Write good PR descriptions
- Address review feedback
- Keep PRs focused and small

## Maintainer Role

**Who:** Repository owners, senior developers

**Permissions:**
- ✅ All developer permissions
- ✅ Review PRs
- ✅ Approve changes
- ✅ Merge to main
- ✅ Manage merge queue

**Commands:**
```bash
# All developer commands PLUS:
garry merge      # ✅ ALLOWED - Merge approved PRs
```

**Workflow:**
1. Review PRs
2. Request changes or approve
3. Verify CI passes
4. Run `garry merge` (or let bot do it)
5. Monitor merge queue

**Responsibilities:**
- Review code quality
- Ensure tests pass
- Maintain main branch integrity
- Manage merge conflicts
- Monitor bot health

## Garry Bot Role

**Who:** Automated service

**Permissions:**
- ✅ Monitor reviews
- ✅ Check CI status
- ✅ Manage merge queue
- ✅ Merge approved PRs
- ✅ Send notifications

**Actions:**
```bash
# Runs continuously
garry-bot

# Automatically:
# - Detects approved PRs
# - Checks CI status
# - Adds to queue
# - Merges in order
# - Notifies developers
```

**Workflow:**
1. Monitor for approved PRs
2. Verify CI passes
3. Add to merge queue
4. Process queue (FIFO)
5. Squash merge to main
6. Notify developer

**Responsibilities:**
- Maintain merge queue
- Enforce linear history
- Handle conflicts
- Send notifications
- Log all actions

## Permission Matrix

| Action | Developer | Maintainer | Bot |
|--------|-----------|------------|-----|
| Create branch | ✅ | ✅ | ❌ |
| Make commits | ✅ | ✅ | ❌ |
| Squash commits | ✅ | ✅ | ❌ |
| Upload PR | ✅ | ✅ | ❌ |
| Update PR | ✅ | ✅ | ❌ |
| Review PR | ❌ | ✅ | ❌ |
| Approve PR | ❌ | ✅ | ❌ |
| Merge PR | ❌ | ✅ | ✅ |
| Manage queue | ❌ | ✅ | ✅ |
| Push to main | ❌ | ❌ | ✅ |

## Why This Separation?

### Security
- Main branch is protected
- Only authorized merges
- Prevents accidental pushes

### Quality
- All changes reviewed
- CI must pass
- Maintainers verify quality

### Order
- FIFO merge queue
- Linear history
- No race conditions

### Accountability
- Clear ownership
- Audit trail
- Traceable changes

## Visual Workflow

```
Developer                Maintainer              Bot
    │                        │                    │
    ├─ garry start          │                    │
    ├─ git commit           │                    │
    ├─ garry squash         │                    │
    ├─ garry upload         │                    │
    │                        │                    │
    │   ┌────────────────────┘                    │
    │   │ Review PR                               │
    │   │ Approve                                 │
    │   └────────────────────┐                    │
    │                        │                    │
    │                        ├─ garry merge       │
    │                        │   (optional)       │
    │                        │                    │
    │                        │   ┌────────────────┘
    │                        │   │ Detect approval
    │                        │   │ Check CI
    │                        │   │ Add to queue
    │                        │   │ Process queue
    │                        │   │ Merge to main
    │                        │   └────────────────┐
    │                        │                    │
    │   ◄────────────────────┴────────────────────┘
    │   Notification: Merged!
    │
```

## Common Scenarios

### Scenario 1: Normal Flow

1. **Developer** creates PR with `garry upload`
2. **Maintainer** reviews and approves
3. **Bot** automatically merges when CI passes
4. **Developer** gets notification

### Scenario 2: Manual Merge

1. **Developer** creates PR with `garry upload`
2. **Maintainer** reviews and approves
3. **Maintainer** runs `garry merge` manually
4. **Bot** processes the merge
5. **Developer** gets notification

### Scenario 3: Feedback Required

1. **Developer** creates PR with `garry upload`
2. **Maintainer** requests changes
3. **Developer** addresses feedback with `garry update`
4. **Maintainer** approves
5. **Bot** merges automatically

### Scenario 4: CI Failure

1. **Developer** creates PR with `garry upload`
2. **CI** fails
3. **Bot** does NOT merge
4. **Developer** fixes issues with `garry update`
5. **CI** passes
6. **Bot** merges automatically

## Best Practices

### For Developers
- Focus on your feature
- Don't worry about merging
- Trust the process
- Address feedback quickly

### For Maintainers
- Review thoroughly
- Approve only when ready
- Let bot handle merges (preferred)
- Use manual merge sparingly

### For Bot Setup
- Run as a service
- Monitor logs
- Configure properly
- Keep updated

## FAQ

**Q: Can I merge my own PR?**
A: Only if you're a maintainer. Regular developers cannot merge.

**Q: Why can't I run garry merge?**
A: To protect main branch integrity and maintain linear history.

**Q: What if the bot is down?**
A: Maintainers can run `garry merge` manually.

**Q: How do I become a maintainer?**
A: Ask your repository owner for maintainer permissions.

**Q: Can I bypass the queue?**
A: No. All merges go through the queue to maintain order.

## Next Steps

- [Workflow Guide](WORKFLOW.md)
- [CLI Reference](CLI.md)
- [Bot Service](BOT.md)
