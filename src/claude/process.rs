use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};
use tokio::process::Command;
use anyhow::{Result, anyhow};
use log::{info, debug};

pub struct ClaudeCliProcess {
    is_running: Arc<Mutex<bool>>,
}

impl ClaudeCliProcess {
    pub async fn new() -> Result<Self> {
        debug!("Initializing Claude CLI process manager");
        
        let process = ClaudeCliProcess {
            is_running: Arc::new(Mutex::new(true)),
        };

        Ok(process)
    }

    pub async fn send_prompt(&self, prompt: &str) -> Result<String> {
        debug!("Sending prompt to Claude CLI: {}", prompt);
        
        let output = Command::new("claude")
            .arg("--")
            .arg(prompt)
            .output()
            .await
            .map_err(|e| anyhow!("Failed to execute Claude CLI: {}. Make sure Claude CLI is installed and in PATH", e))?;

        if output.status.success() {
            let response = String::from_utf8_lossy(&output.stdout).trim().to_string();
            debug!("Claude CLI response received: {} chars", response.len());
            Ok(response)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Claude CLI failed: {}", error))
        }
    }

    pub async fn send_prompt_with_mcp(&self, prompt: &str, mcp_tools: &[String]) -> Result<String> {
        let mcp_context = if !mcp_tools.is_empty() {
            format!("Use these MCP tools: {}\n\n", mcp_tools.join(", "))
        } else {
            String::new()
        };
        
        let full_prompt = format!("{}{}", mcp_context, prompt);
        self.send_prompt(&full_prompt).await
    }

    pub async fn is_alive(&self) -> bool {
        *self.is_running.lock().await
    }

    pub async fn shutdown(&self) -> Result<()> {
        {
            let mut is_running = self.is_running.lock().await;
            *is_running = false;
        }
        
        info!("Claude CLI process manager shut down");
        Ok(())
    }
}

static CLAUDE_CLI_INSTANCE: OnceCell<Arc<ClaudeCliProcess>> = OnceCell::const_new();

pub struct ClaudeCliManager;

impl ClaudeCliManager {
    pub async fn get_instance() -> Result<Arc<ClaudeCliProcess>> {
        let instance = CLAUDE_CLI_INSTANCE.get_or_try_init(|| async {
            let process = ClaudeCliProcess::new().await?;
            Ok::<Arc<ClaudeCliProcess>, anyhow::Error>(Arc::new(process))
        }).await?;
        
        Ok(Arc::clone(instance))
    }

    pub async fn shutdown() -> Result<()> {
        if let Some(instance) = CLAUDE_CLI_INSTANCE.get() {
            instance.shutdown().await?;
        }
        Ok(())
    }
}