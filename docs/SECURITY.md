# Security Model

Garry's security model ensures that only authorized users can merge to main, while maintaining a smooth developer experience.

## Token Separation

### Developer Tokens
Used for day-to-day development work:

**Permissions Needed:**
- ✅ Read repository
- ✅ Create branches
- ✅ Push to feature branches
- ✅ Create PRs/MRs
- ✅ Update PRs/MRs
- ❌ Push to main (BLOCKED)
- ❌ Merge PRs/MRs (BLOCKED)

**Commands:**
- `garry start`
- `garry squash`
- `garry upload`
- `garry update`

### Bot Token
Used exclusively by Garry Bot:

**Permissions Needed:**
- ✅ Read repository
- ✅ Create branches
- ✅ Push to ALL branches (including main)
- ✅ Merge PRs/MRs
- ✅ Manage branch protection
- ✅ Create/modify repository rules

**Commands:**
- Automatic merge queue processing
- `garry merge` (when run by maintainers, communicates with bot)

## Branch Protection

### Automatic Setup

When Garry Bot starts, it automatically configures:

1. **Main Branch Protection**
   ```
   - Require PR before merging
   - Require at least 1 approval
   - Require CI checks to pass
   - Dismiss stale reviews
   - Restrict who can push (Bot only!)
   ```

2. **Push Restrictions**
   ```
   - Only garry-bot can push to main
   - All other users MUST use PRs
   - Direct pushes are blocked
   ```

3. **Review Requirements**
   ```
   - At least 1 approval required
   - CI must pass
   - No bypassing allowed
   ```

### Manual Configuration

If bot can't set up protection automatically (needs admin permissions), configure manually:

#### GitHub
1. Go to Settings → Branches
2. Add rule for `main`
3. Enable:
   - Require pull request reviews (1 approval)
   - Require status checks to pass
   - Restrict who can push (add garry-bot)

#### GitLab (Coming Soon)
1. Go to Settings → Repository → Protected Branches
2. Protect `main`
3. Set:
   - Allowed to merge: Maintainers + garry-bot
   - Allowed to push: garry-bot only

## How Merging Works

### The Secure Flow

```
┌─────────────────────────────────────────────────────────┐
│                    SECURE MERGE FLOW                     │
└─────────────────────────────────────────────────────────┘

1. Developer creates PR
   ├─ Uses developer token
   ├─ Cannot push to main
   └─ Cannot merge PR

2. Maintainer approves
   ├─ Reviews code
   ├─ Approves PR
   └─ Runs garry merge (optional)

3. Garry Bot merges
   ├─ Uses bot token
   ├─ Has permission to push to main
   ├─ Processes merge queue
   ├─ Squash merges to main
   └─ Notifies developer

4. Main branch updated
   ├─ Linear history maintained
   ├─ All checks passed
   └─ Audit trail preserved
```

### Why This is Secure

1. **Separation of Concerns**
   - Developers can't accidentally push to main
   - Only bot has merge permissions
   - Clear audit trail

2. **Enforced Review Process**
   - All changes require PR
   - All PRs require approval
   - All PRs require CI

3. **Queue Management**
   - FIFO processing prevents race conditions
   - Conflicts detected automatically
   - Linear history enforced

4. **Token Isolation**
   - Developer tokens can't merge
   - Bot token is separate
   - No shared credentials

## Attack Scenarios & Mitigations

### Scenario 1: Developer Tries to Push to Main

**Attack:**
```bash
git push origin main
```

**Mitigation:**
- Branch protection blocks the push
- Error message directs to use Garry
- Audit log records attempt

### Scenario 2: Developer Tries to Bypass Queue

**Attack:**
```bash
garry merge  # Run by non-maintainer
```

**Mitigation:**
- Command communicates with bot
- Bot checks permissions
- Only maintainers can initiate merge
- Queue order is enforced

### Scenario 3: Compromised Developer Token

**Attack:**
- Attacker gets developer token
- Tries to merge malicious code

**Mitigation:**
- Token can't push to main
- Token can't merge PRs
- Still requires approval
- Still requires CI

### Scenario 4: Compromised Bot Token

**Attack:**
- Attacker gets bot token
- Tries to push directly to main

**Mitigation:**
- Bot only merges approved PRs
- Bot checks CI status
- Bot processes queue in order
- All actions logged
- Rotate token immediately

## Best Practices

### For Developers

✅ **DO:**
- Keep your token secure
- Use environment variables
- Never commit tokens
- Rotate tokens regularly

❌ **DON'T:**
- Share your token
- Commit tokens to Git
- Try to bypass Garry
- Push directly to main

### For Maintainers

✅ **DO:**
- Review PRs thoroughly
- Verify CI passes
- Let bot handle merges
- Monitor bot logs
- Rotate bot token regularly

❌ **DON'T:**
- Approve without review
- Bypass the queue
- Share bot token
- Disable protection

### For Bot Setup

✅ **DO:**
- Use dedicated bot account
- Store token securely
- Monitor bot logs
- Set up alerts
- Keep bot updated

❌ **DON'T:**
- Use personal token for bot
- Store token in code
- Run multiple bots
- Disable logging

## Token Management

### Creating Tokens

#### Developer Token (GitHub)
1. Settings → Developer settings → Personal access tokens
2. Generate new token (classic)
3. Scopes: `repo`, `workflow`
4. Copy and store securely

#### Bot Token (GitHub)
1. Create dedicated bot account
2. Add as collaborator with write access
3. Generate token with scopes: `repo`, `workflow`, `admin:repo_hook`
4. Store in secure location (environment variable, secrets manager)

### Rotating Tokens

#### When to Rotate
- Every 90 days (recommended)
- When team member leaves
- If token is compromised
- After security incident

#### How to Rotate
1. Generate new token
2. Update configuration
3. Restart bot
4. Revoke old token
5. Verify everything works

## Audit Trail

### What Gets Logged

- All PR creations
- All PR updates
- All merge attempts
- All queue operations
- All protection changes
- All errors

### Where Logs Go

- Bot console output
- Log files (if configured)
- Platform audit logs
- CI/CD logs

### Monitoring

```bash
# Watch bot logs
RUST_LOG=info garry-bot

# Check for errors
grep ERROR garry-bot.log

# Monitor merge queue
grep "Processing merge queue" garry-bot.log
```

## Compliance

### SOC 2 / ISO 27001

Garry helps with compliance by:
- Enforcing code review
- Maintaining audit trail
- Separating duties
- Preventing unauthorized changes

### GDPR

- No personal data stored
- Tokens are user-managed
- Logs can be anonymized
- Data retention configurable

## Incident Response

### If Developer Token is Compromised

1. Revoke token immediately
2. Generate new token
3. Update configuration
4. Review recent PRs from that user
5. Check audit logs

### If Bot Token is Compromised

1. **URGENT**: Revoke token immediately
2. Check recent merges to main
3. Review bot logs
4. Generate new bot token
5. Update bot configuration
6. Restart bot
7. Verify branch protection
8. Audit recent changes to main

### If Main Branch is Compromised

1. Lock repository
2. Review recent commits
3. Identify malicious changes
4. Revert if needed
5. Investigate how it happened
6. Fix security gap
7. Rotate all tokens
8. Document incident

## Security Checklist

### Initial Setup
- [ ] Create dedicated bot account
- [ ] Generate bot token with correct permissions
- [ ] Store bot token securely
- [ ] Configure branch protection
- [ ] Test protection rules
- [ ] Set up monitoring
- [ ] Document token locations

### Ongoing
- [ ] Review bot logs weekly
- [ ] Rotate tokens quarterly
- [ ] Audit PRs regularly
- [ ] Monitor for anomalies
- [ ] Keep Garry updated
- [ ] Review access permissions
- [ ] Test disaster recovery

## Next Steps

- [Bot Service](BOT.md)
- [Configuration](CONFIGURATION.md)
- [Adapters](ADAPTERS.md)
