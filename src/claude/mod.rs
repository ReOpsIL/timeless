use anyhow::Result;

pub mod client;
pub mod prompts;
pub mod context;
pub mod process;

pub use client::ClaudeClient;
pub use prompts::PromptTemplate;
pub use context::ConversationContext;
pub use process::{ClaudeCliProcess, ClaudeCliManager};

pub async fn test_claude_connection() -> Result<()> {
    println!("Testing Claude CLI connection...");
    // TODO: Implement actual Claude CLI test
    Ok(())
}