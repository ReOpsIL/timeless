use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub team: TeamConfig,
    pub claude: ClaudeConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub data_dir: String,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamConfig {
    pub name: String,
    pub timezone: String,
    pub working_hours: WorkingHours,
    pub working_days: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingHours {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub backup_enabled: bool,
    pub backup_interval: String,
    pub max_backups: u32,
    pub compression: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            app: AppConfig {
                name: "Smart Team Manager".to_string(),
                version: "0.1.0".to_string(),
                data_dir: "./data".to_string(),
                log_level: "info".to_string(),
            },
            team: TeamConfig {
                name: "Engineering Team".to_string(),
                timezone: "UTC".to_string(),
                working_hours: WorkingHours {
                    start: "09:00".to_string(),
                    end: "17:00".to_string(),
                },
                working_days: vec![
                    "Monday".to_string(),
                    "Tuesday".to_string(),
                    "Wednesday".to_string(),
                    "Thursday".to_string(),
                    "Friday".to_string(),
                ],
            },
            claude: ClaudeConfig {
                enabled: true,
            },
            storage: StorageConfig {
                backup_enabled: true,
                backup_interval: "daily".to_string(),
                max_backups: 7,
                compression: true,
            },
        }
    }
}

pub fn load_config(_path: &str) -> Result<Config> {
    // TODO: Implement actual config loading from TOML file
    // For now, return default config
    Ok(Config::default())
}