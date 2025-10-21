# Workflow Guide

This guide explains the proper Garry workflow and who does what.

## Roles

### Developers
Regular contributors who work on features and fixes.

**Responsibilities:**
- Create feature branches
- Make commits
- Squash commits
- Upload reviews (PRs)
- Address review feedback

**Commands:**
- ✅ `garry start`
- ✅ `garry squash`
- ✅ `garry upload`
- ✅ `garry update`
- ❌ `garry merge` (NOT for developers!)

### Maintainers
Repository owners who manage the main branch.

**Responsibilities:**
- Review PRs
- Approve changes
- Manage merge queue
- Handle conflicts
- Maintain main branch

**Commands:**
- ✅ All developer commands
- ✅ `garry merge` (maintainer privilege)

### Garry Bot
Automated service that handles merges.

**Responsibilities:**
- Monitor approved reviews
- Check CI status
- Manage merge queue
- Automatically merge approved changes
- Notify developers

**Actions:**
- Automatically runs `garry merge` logic
- Processes queue in FIFO order
- Handles conflicts
- Sends notifications

## Developer Workflow

```
┌─────────────────────────────────────────────────────────────┐
│                    DEVELOPER WORKFLOW                        │
└─────────────────────────────────────────────────────────────┘

1. Start Feature
   ↓
   garry start feature/my-feature
   ↓
2. Make Changes
   ↓
   git add .
   git commit -m "Add feature"
   git commit -m "Add tests"
   ↓
3. Squash Commits
   ↓
   garry squash
   ↓
4. Upload for Review
   ↓
   garry upload --title "Add feature"
   ↓
5. Wait for Review & CI
   ↓
6. Address Feedback (if needed)
   ↓
   git add .
   git commit -m "Address feedback"
   garry update
   ↓
7. STOP HERE! ← Developer's job is done
   ↓
   Wait for maintainer/bot to merge
```

## Maintainer/Bot Workflow

```
┌─────────────────────────────────────────────────────────────┐
│                  MAINTAINER/BOT WORKFLOW                     │
└─────────────────────────────────────────────────────────────┘

1. Review Approved
   ↓
2. CI Checks Pass
   ↓
3. Add to Merge Queue
   ↓
   garry merge (maintainer)
   OR
   Automatic (bot)
   ↓
4. Process Queue (FIFO)
   ↓
5. Squash Merge to Main
   ↓
6. Notify Developer
   ↓
7. Done! ✅
```

## Why This Separation?

### 🛡️ Protects Main Branch
- Only maintainers/bot can merge
- Prevents accidental merges
- Ensures all conditions are met

### 📊 Enforces Linear History
- Bot merges in correct order
- Automatic rebasing and squashing
- Clean, readable history

### 🔒 Handles Conflicts Safely
- Bot detects conflicts
- Removes from queue
- Notifies developer to rebase

### 👥 Maintains Accountability
- Clear separation of concerns
- Developers focus on features
- Maintainers control integration

## Complete Example

### Developer Side

```bash
# 1. Start feature
garry start feature/user-login

# 2. Work on feature
echo "login code" > login.rs
git add login.rs
git commit -m "Add login form"

echo "validation" >> login.rs
git add login.rs
git commit -m "Add validation"

# 3. Squash commits
garry squash

# 4. Upload for review
garry upload --title "Add user login" --description "Implements login feature"

# 5. Wait for review...
# 6. Address feedback if needed
git add .
git commit -m "Fix validation bug"
garry update

# 7. DONE! Wait for maintainer/bot to merge
```

### Maintainer Side

```bash
# 1. Review the PR on GitHub/GitLab
# 2. Approve if looks good
# 3. Wait for CI to pass
# 4. Merge (or let bot do it automatically)
garry merge

# Bot will handle the rest!
```

### Bot Side (Automatic)

```bash
# Bot runs continuously
garry-bot

# Bot automatically:
# - Detects approved reviews
# - Checks CI status
# - Adds to queue
# - Merges in order
# - Notifies developers
```

## Best Practices

### For Developers

✅ **DO:**
- Use `garry start` for new branches
- Squash before uploading
- Write clear PR descriptions
- Address feedback promptly
- Keep PRs small and focused

❌ **DON'T:**
- Run `garry merge` (not your job!)
- Push directly to main
- Skip squashing
- Make huge PRs
- Ignore review feedback

### For Maintainers

✅ **DO:**
- Review PRs thoroughly
- Approve only when ready
- Let bot handle merges (preferred)
- Use `garry merge` manually if needed
- Monitor merge queue

❌ **DON'T:**
- Approve without CI passing
- Merge out of order
- Skip the queue
- Push directly to main

### For Bot Setup

✅ **DO:**
- Run bot as a service
- Monitor bot logs
- Configure proper intervals
- Set up notifications
- Keep bot updated

❌ **DON'T:**
- Run multiple bots
- Skip configuration
- Ignore bot errors
- Disable CI checks

## Workflow Diagram

```
┌──────────────┐
│  Developer   │
└──────┬───────┘
       │
       ├─→ garry start
       ├─→ git commit (multiple)
       ├─→ garry squash
       ├─→ garry upload
       └─→ garry update (if needed)
       
       ↓ (STOP HERE)
       
┌──────────────┐
│  Maintainer  │
│  or Bot      │
└──────┬───────┘
       │
       ├─→ Review & Approve
       ├─→ Check CI
       └─→ garry merge
       
       ↓
       
┌──────────────┐
│  Main Branch │
│  (Protected) │
└──────────────┘
```

## Troubleshooting

### "I uploaded my PR, now what?"

Wait! Your job is done. The maintainer or bot will merge it once approved and CI passes.

### "Can I merge my own PR?"

Only if you're a maintainer. Regular developers should not run `garry merge`.

### "My PR is approved but not merged"

Check:
- CI status (must pass)
- Merge queue position
- Bot is running
- No merge conflicts

### "I accidentally ran garry merge"

Contact a maintainer immediately. They may need to:
- Check main branch integrity
- Verify merge queue state
- Potentially revert if issues

## Next Steps

- [CLI Reference](CLI.md)
- [Bot Service](BOT.md)
- [Contributing Guide](CONTRIBUTING.md)
