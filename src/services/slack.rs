use anyhow::Result;
use crate::claude::ClaudeCliManager;

pub struct SlackService;

impl SlackService {
    pub fn new() -> Self {
        Self
    }

    pub async fn send_message(&self, channel: &str, message: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let prompt = format!(
            "Send a message to Slack channel {} with content: '{}' - Use Slack MCP tools. \
            Confirm the message was sent successfully.",
            channel, message
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn get_user_status(&self, user_id: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let prompt = format!(
            "Get the current status and presence information for Slack user {} - Use Slack MCP tools. \
            Include their status message, presence (online/away), and when they were last active.",
            user_id
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn collect_team_status(&self, channel: &str, team_members: &[String]) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let members_list = team_members.join(", ");
        let prompt = format!(
            "Collect status updates from team members {} in channel {} - Use Slack MCP tools. \
            Ask each member for their current work status, blockers, and plans for today. \
            Provide a summary of all responses.",
            members_list, channel
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn schedule_standup_reminder(&self, channel: &str, time: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let prompt = format!(
            "Schedule a daily standup reminder in Slack channel {} for {} - Use Slack MCP tools. \
            Set up a recurring message that asks team members for their status updates.",
            channel, time
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn send_team_report(&self, channel: &str, report_content: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let prompt = format!(
            "Send a formatted team report to Slack channel {} with the following content: \
            {} \
            Use Slack MCP tools to send the message with proper formatting.",
            channel, report_content
        );
        
        claude.send_prompt(&prompt).await
    }
}