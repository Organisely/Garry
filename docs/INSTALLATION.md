# Installation Guide

## Prerequisites

- Rust 1.70+ (2021 edition)
- Git 2.0+
- A GitHub/GitLab/Bitbucket account with API access

## Quick Install

```bash
# Clone the repository
git clone https://github.com/organisely/garry.git
cd garry

# Run the quick start script
chmod +x quickstart.sh
./quickstart.sh
```

## Manual Installation

```bash
# Build from source
cargo build --release

# Install binaries
cargo install --path .
```

## Verify Installation

```bash
garry --version
garry-bot --version
```

## Next Steps

- [Configure Garry](CONFIGURATION.md)
- [Quick Start Tutorial](QUICKSTART.md)
