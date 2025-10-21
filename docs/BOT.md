# Garry Bot Service

The Garry Bot is an automation service that manages the merge queue and handles CI/CD integration.

## What Does the Bot Do?

- 📋 Manages merge queue (FIFO)
- ✅ Monitors CI/CD status
- 🔍 Tracks review approvals
- 🚀 Automatically merges approved changes
- 🔔 Sends notifications to developers

## Running the Bot

### Basic Usage

```bash
garry-bot
```

### With Custom Log Level

```bash
RUST_LOG=debug garry-bot
```

### As a Background Service

```bash
# Using nohup
nohup garry-bot > garry-bot.log 2>&1 &

# Using systemd (Linux)
sudo systemctl start garry-bot
```

## Configuration

The bot uses the same configuration as the CLI. See [CONFIGURATION.md](CONFIGURATION.md).

Important bot settings:

```toml
[bot]
webhook_port = 8080
queue_check_interval = 30  # Check queue every 30 seconds
ci_timeout = 3600           # Wait up to 1 hour for CI
main_branch = "main"
```

## How the Queue Works

1. **Review Approved** → Bot detects approval
2. **CI Checks** → Bot waits for CI to pass
3. **Add to Queue** → Review added to merge queue
4. **Process Queue** → Bot processes in FIFO order
5. **Merge** → Squash merge to main
6. **Notify** → Developer notified of success

## Queue States

- **Pending** - Waiting in queue
- **Testing** - CI checks running
- **Merging** - Being merged
- **Failed** - Merge failed (conflict or CI failure)

## Deployment

### Docker

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin garry-bot

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/garry-bot /usr/local/bin/
CMD ["garry-bot"]
```

### Systemd Service

Create `/etc/systemd/system/garry-bot.service`:

```ini
[Unit]
Description=Garry Bot Service
After=network.target

[Service]
Type=simple
User=garry
WorkingDirectory=/opt/garry
ExecStart=/usr/local/bin/garry-bot
Restart=always
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable garry-bot
sudo systemctl start garry-bot
sudo systemctl status garry-bot
```

## Monitoring

### Logs

```bash
# View logs
RUST_LOG=info garry-bot

# Debug logs
RUST_LOG=debug garry-bot

# Specific module logs
RUST_LOG=garry::bot=debug garry-bot
```

### Health Checks

The bot logs its status regularly:

```
INFO garry_bot: Starting Garry Bot...
INFO garry_bot: Loaded configuration for repository: owner/repo
INFO garry_bot: Garry Bot started successfully
INFO garry_bot: Processing merge queue (0 entries)
```

## Troubleshooting

### Bot Not Processing Queue

- Check configuration is correct
- Verify VCS token has proper permissions
- Check CI status is being reported correctly
- Review bot logs for errors

### Reviews Not Being Merged

- Ensure review is approved
- Verify CI checks have passed
- Check for merge conflicts
- Confirm review is in queue

### High CPU Usage

- Increase `queue_check_interval` to reduce polling
- Check for stuck reviews in queue
- Review log level (debug logs are verbose)

## Next Steps

- [CLI Reference](CLI.md)
- [Configuration Guide](CONFIGURATION.md)
- [Troubleshooting](TROUBLESHOOTING.md)
