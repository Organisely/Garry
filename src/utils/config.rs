use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;
use crate::utils::error::{GarryError, Result};

/// Main configuration structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub vcs: VcsConfig,
    pub bot: BotConfig,
    pub git: GitConfig,
}

impl Config {
    /// Load configuration from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| GarryError::ConfigError(format!("Failed to read config file: {}", e)))?;
        
        let config: Config = toml::from_str(&content)
            .map_err(|e| GarryError::ConfigError(format!("Failed to parse config: {}", e)))?;
        
        config.validate()?;
        Ok(config)
    }
    
    /// Load configuration with fallback chain
    pub fn load_with_fallback() -> Result<Self> {
        // Try repository config first
        if let Ok(config) = Self::load(".garry/config.toml") {
            return Ok(config);
        }
        
        // Try user-level config
        if let Some(home) = dirs::home_dir() {
            let user_config = home.join(".config/garry/config.toml");
            if user_config.exists() {
                if let Ok(config) = Self::load(&user_config) {
                    return Ok(config);
                }
            }
        }
        
        // Return default with warning
        Err(GarryError::ConfigError(
            "No configuration file found. Create .garry/config.toml".to_string()
        ))
    }
    
    /// Load configuration with environment variable overrides
    pub fn load_with_env() -> Result<Self> {
        let mut config = Self::load_with_fallback()?;
        
        // Override with environment variables
        if let Ok(token) = std::env::var("GARRY_VCS_TOKEN") {
            config.vcs.token = token;
        }
        if let Ok(platform) = std::env::var("GARRY_VCS_PLATFORM") {
            config.vcs.platform = platform;
        }
        if let Ok(host) = std::env::var("GARRY_VCS_HOST") {
            config.vcs.host = host;
        }
        if let Ok(repo) = std::env::var("GARRY_VCS_REPOSITORY") {
            config.vcs.repository = repo;
        }
        
        config.validate()?;
        Ok(config)
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.vcs.token.is_empty() {
            return Err(GarryError::ConfigError("VCS token is required".to_string()));
        }
        if self.vcs.repository.is_empty() {
            return Err(GarryError::ConfigError("VCS repository is required".to_string()));
        }
        if !self.vcs.repository.contains('/') {
            return Err(GarryError::ConfigError(
                "Repository must be in format 'owner/repo'".to_string()
            ));
        }
        Ok(())
    }
}

/// VCS platform configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VcsConfig {
    /// Platform type: "github", "gitlab", "bitbucket", "gitea", "self-hosted"
    pub platform: String,
    /// VCS host URL (e.g., "github.com", "gitlab.com", or custom domain)
    pub host: String,
    /// Authentication token
    pub token: String,
    /// Repository in format "owner/repo"
    pub repository: String,
}

/// Bot service configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    /// Port for webhook listener
    pub webhook_port: u16,
    /// Interval in seconds to check queue
    pub queue_check_interval: u64,
    /// CI timeout in seconds
    pub ci_timeout: u64,
    /// Main branch name (usually "main" or "master")
    pub main_branch: String,
}

/// Git configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitConfig {
    /// Default remote name (usually "origin")
    pub default_remote: String,
    /// Base branch for squashing (usually "main" or "origin/main")
    pub squash_base: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vcs: VcsConfig {
                platform: "github".to_string(),
                host: "github.com".to_string(),
                token: String::new(),
                repository: String::new(),
            },
            bot: BotConfig {
                webhook_port: 8080,
                queue_check_interval: 30,
                ci_timeout: 3600,
                main_branch: "main".to_string(),
            },
            git: GitConfig {
                default_remote: "origin".to_string(),
                squash_base: "main".to_string(),
            },
        }
    }
}
