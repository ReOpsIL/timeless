use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub team: TeamConfig,
    pub claude: ClaudeConfig,
    pub mcp: McpConfig,
    pub workflows: WorkflowsConfig,
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
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub api_key_env: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    pub enabled: bool,
    pub timeout: u32,
    pub retry_attempts: u32,
    pub servers: HashMap<String, McpServerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: Option<String>,
    pub env: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowsConfig {
    pub daily_status: WorkflowConfig,
    pub weekly_report: WorkflowConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    pub enabled: bool,
    pub schedule: String,
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
        let mut mcp_servers = HashMap::new();
        
        // Default MCP server configurations
        mcp_servers.insert("slack".to_string(), McpServerConfig {
            command: "npx".to_string(),
            args: vec!["@modelcontextprotocol/server-slack".to_string()],
            working_dir: Some(".".to_string()),
            env: HashMap::new(),
        });
        
        mcp_servers.insert("github".to_string(), McpServerConfig {
            command: "npx".to_string(),
            args: vec!["@modelcontextprotocol/server-github".to_string()],
            working_dir: Some(".".to_string()),
            env: HashMap::new(),
        });
        
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
                model: "claude-3-5-sonnet-20241022".to_string(),
                max_tokens: 4000,
                temperature: 0.7,
                api_key_env: "CLAUDE_API_KEY".to_string(),
            },
            mcp: McpConfig {
                enabled: true,
                timeout: 30,
                retry_attempts: 3,
                servers: mcp_servers,
            },
            workflows: WorkflowsConfig {
                daily_status: WorkflowConfig {
                    enabled: true,
                    schedule: "0 9 * * MON-FRI".to_string(),
                },
                weekly_report: WorkflowConfig {
                    enabled: true,
                    schedule: "0 17 * * FRI".to_string(),
                },
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