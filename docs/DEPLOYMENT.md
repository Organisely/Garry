# Deployment Guide

Garry Bot can be deployed anywhere! Choose the option that fits your infrastructure.

## Quick Start

### Docker (Easiest)

```bash
# 1. Create config
cp .env.example .env
# Edit .env with your settings

# 2. Run with docker-compose
docker-compose up -d

# 3. Check logs
docker-compose logs -f garry-bot
```

### Kubernetes

```bash
# 1. Create secret
kubectl create secret generic garry-secrets \
  --from-literal=vcs-token=your-github-token

# 2. Update configmap
kubectl apply -f k8s/configmap.yaml

# 3. Deploy
kubectl apply -f k8s/

# 4. Check status
kubectl get pods -l app=garry-bot
kubectl logs -f deployment/garry-bot
```

### Systemd (Linux Server)

```bash
# 1. Build and install
cargo build --release
sudo cp target/release/garry-bot /usr/local/bin/

# 2. Create user
sudo useradd -r -s /bin/false garry
sudo mkdir -p /opt/garry/.garry
sudo chown -R garry:garry /opt/garry

# 3. Create config
sudo cp .garry/config.toml.example /opt/garry/.garry/config.toml
sudo nano /opt/garry/.garry/config.toml

# 4. Install service
sudo cp systemd/garry-bot.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable garry-bot
sudo systemctl start garry-bot

# 5. Check status
sudo systemctl status garry-bot
sudo journalctl -u garry-bot -f
```

## Platform-Specific Guides

### Fly.io

```bash
# 1. Install flyctl
curl -L https://fly.io/install.sh | sh

# 2. Login
flyctl auth login

# 3. Create app
flyctl launch

# 4. Set secrets
flyctl secrets set GARRY_VCS_TOKEN=your-token

# 5. Deploy
flyctl deploy
```

### Railway

```bash
# 1. Install railway CLI
npm install -g @railway/cli

# 2. Login
railway login

# 3. Create project
railway init

# 4. Set variables
railway variables set GARRY_VCS_TOKEN=your-token

# 5. Deploy
railway up
```

### AWS ECS

```bash
# 1. Build and push image
docker build -t garry-bot .
aws ecr get-login-password | docker login --username AWS --password-stdin <account>.dkr.ecr.<region>.amazonaws.com
docker tag garry-bot:latest <account>.dkr.ecr.<region>.amazonaws.com/garry-bot:latest
docker push <account>.dkr.ecr.<region>.amazonaws.com/garry-bot:latest

# 2. Create task definition (see AWS console or use terraform)

# 3. Create service
aws ecs create-service --cluster your-cluster --service-name garry-bot --task-definition garry-bot
```

### Google Cloud Run

```bash
# 1. Build and push
gcloud builds submit --tag gcr.io/your-project/garry-bot

# 2. Deploy
gcloud run deploy garry-bot \
  --image gcr.io/your-project/garry-bot \
  --platform managed \
  --region us-central1 \
  --set-env-vars GARRY_VCS_TOKEN=your-token
```

## Configuration

### Environment Variables

All configuration can be done via environment variables:

```bash
GARRY_VCS_TOKEN=your-token
GARRY_VCS_PLATFORM=github
GARRY_VCS_HOST=github.com
GARRY_VCS_REPOSITORY=owner/repo
RUST_LOG=info
```

### Config File

Or use a config file at `.garry/config.toml`:

```toml
[vcs]
platform = "github"
host = "github.com"
token = "your-token"
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

## Monitoring

### Health Checks

Garry Bot exposes health information through process checks:

```bash
# Check if running
pgrep garry-bot

# Check logs
docker logs garry-bot
kubectl logs deployment/garry-bot
journalctl -u garry-bot
```

### Metrics

Garry logs all operations:

```bash
# Set log level
RUST_LOG=debug garry-bot

# Filter logs
RUST_LOG=garry::bot=debug,garry::queue=info garry-bot
```

## Scaling

### Single Instance (Recommended)

Garry Bot should run as a single instance to maintain queue order:

```yaml
# Kubernetes
replicas: 1

# Docker Compose
deploy:
  replicas: 1
```

### High Availability

For HA, use:
- Leader election (future feature)
- Database-backed queue (future feature)
- For now: Run single instance with auto-restart

## Security

### Secrets Management

**Never commit secrets!**

Use:
- Kubernetes Secrets
- Docker Secrets
- AWS Secrets Manager
- HashiCorp Vault
- Environment variables

### Network Security

- Run behind firewall
- Use HTTPS for webhooks
- Restrict access to webhook port
- Use VPC/private networks

## Troubleshooting

### Bot Not Starting

```bash
# Check config
cat .garry/config.toml

# Check logs
docker logs garry-bot
kubectl logs deployment/garry-bot

# Verify token
echo $GARRY_VCS_TOKEN
```

### Bot Not Processing Queue

```bash
# Check if running
pgrep garry-bot

# Check logs for errors
grep ERROR /var/log/garry-bot.log

# Verify VCS connection
curl -H "Authorization: token $GARRY_VCS_TOKEN" https://api.github.com/user
```

### High Memory Usage

```bash
# Check resource usage
docker stats garry-bot
kubectl top pod -l app=garry-bot

# Adjust limits
# In docker-compose.yml or k8s/deployment.yaml
```

## Backup & Recovery

### Configuration Backup

```bash
# Backup config
cp .garry/config.toml .garry/config.toml.backup

# Backup secrets
kubectl get secret garry-secrets -o yaml > garry-secrets-backup.yaml
```

### Disaster Recovery

```bash
# Restore from backup
cp .garry/config.toml.backup .garry/config.toml

# Restart service
docker-compose restart garry-bot
kubectl rollout restart deployment/garry-bot
sudo systemctl restart garry-bot
```

## Updates

### Docker

```bash
# Pull latest image
docker-compose pull

# Restart
docker-compose up -d
```

### Kubernetes

```bash
# Update image
kubectl set image deployment/garry-bot garry-bot=organisely/garry-bot:v1.1.0

# Or apply new manifest
kubectl apply -f k8s/
```

### Systemd

```bash
# Build new version
cargo build --release

# Stop service
sudo systemctl stop garry-bot

# Update binary
sudo cp target/release/garry-bot /usr/local/bin/

# Start service
sudo systemctl start garry-bot
```

## Cost Estimates

### Cloud Hosting

- **Fly.io**: Free tier, then ~$3/month
- **Railway**: ~$5/month
- **DigitalOcean**: $6/month (basic droplet)
- **AWS ECS**: ~$8/month (Fargate)
- **GCP Cloud Run**: ~$5/month
- **Self-hosted VPS**: $5-10/month

### Resource Requirements

- **CPU**: 0.1-0.5 cores
- **Memory**: 128-256 MB
- **Storage**: <100 MB
- **Network**: Minimal (<1 GB/month)

## Next Steps

- [Configuration Guide](CONFIGURATION.md)
- [Security Best Practices](SECURITY.md)
- [Troubleshooting](TROUBLESHOOTING.md)
