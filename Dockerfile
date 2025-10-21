# Multi-stage build for Garry Bot
FROM rust:1.70 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release --bin garry-bot

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates git && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/garry-bot /usr/local/bin/garry-bot

# Create non-root user
RUN useradd -m -u 1000 garry && \
    mkdir -p /app/.garry && \
    chown -R garry:garry /app

USER garry
WORKDIR /app

# Expose webhook port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD pgrep garry-bot || exit 1

# Run the bot
CMD ["garry-bot"]
