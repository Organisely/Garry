# Garry

[![GitHub release](https://img.shields.io/badge/release-v0.1.0-blue.svg?style=flat-square)](https://github.com/organisely/garry/releases/latest)
[![GitHub license](https://img.shields.io/github/license/organisely/garry)](https://github.com/organisely/garry/blob/main/LICENSE)
[![GitHub contributors](https://img.shields.io/github/contributors/organisely/garry)](https://github.com/organisely/garry/graphs/contributors)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/m/organisely/garry)](https://github.com/organisely/garry/commits)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)

---

### ⚠️ Project Status

**Garry is production-ready and actively maintained by Organisely.**

Garry is an independent Git workflow enforcement tool. It is designed for teams that want to maintain clean Git history and enforce code review processes.

**Garry has absolutely no association with:**
- Other Git tools or services not listed under Organisely
- Any unrelated projects using similar names
- Third-party services or platforms

Garry stands for **Git Automated Review & Relay System**. It is inspired by Chromium's Gerrit workflow.

---

Garry is a Rust-based Git workflow enforcement tool that implements a clean, squash-first workflow. It ensures every change goes through proper review while maintaining a linear Git history.

⚠️ **Garry is production-ready** but actively evolving. Expect regular updates and improvements.

## What is Garry?

Garry enforces clean Git workflows by:
- 🚫 Preventing direct pushes to main
- 🧹 Automatically squashing commits into atomic changes
- 🤖 Managing merge queues with CI/CD integration
- 🔌 Working with GitHub, GitLab, Bitbucket, and more
- ⚡ Providing a fast, reliable CLI and bot service

## Get Started

- [Read documentation 📖](https://github.com/organisely/garry/blob/main/docs)
- [Installation guide 🔽](https://github.com/organisely/garry/blob/main/docs/INSTALLATION.md)
- [Quick start tutorial 🚀](https://github.com/organisely/garry/blob/main/docs/QUICKSTART.md)
- [Architecture overview 🏗️](https://github.com/organisely/garry/blob/main/docs/ARCHITECTURE.md)

## Quick Example

```bash
# Developer workflow
garry start feature/my-feature    # Start feature
git commit -m "Add feature"       # Make commits
garry squash                      # Squash commits
garry upload --title "Add feature" # Upload for review

# Maintainer/Bot handles merge automatically
# (Developers don't run garry merge!)
```

## Features

- ✅ Clean branch management
- ✅ Automatic commit squashing
- ✅ VCS-agnostic adapter system
- ✅ Automated merge queue
- ✅ CI/CD integration
- ✅ Developer notifications
- ✅ Cross-platform support (Linux, macOS, Windows)

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

Made with ❤️ by [Organisely](https://organisely.com)
