use anyhow::Result;
use crate::claude::ClaudeCliManager;

pub struct EmailService;

impl EmailService {
    pub fn new() -> Self {
        Self
    }

    pub async fn send_team_report(&self, recipients: &[String], subject: &str, content: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let recipients_str = recipients.join(", ");
        let prompt = format!(
            "Send an email report to recipients: {} \
            Subject: '{}' \
            Content: '{}' \
            Use Email MCP tools to send a well-formatted HTML email. \
            Confirm delivery status.",
            recipients_str, subject, content
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn send_status_reminder(&self, recipient: &str, team_member_name: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let prompt = format!(
            "Send a friendly status update reminder email to {} for team member {} - \
            Use Email MCP tools. Create a personalized message asking for their current \
            work status, any blockers, and today's priorities.",
            recipient, team_member_name
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn send_weekly_summary(&self, recipients: &[String], team_metrics: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let recipients_str = recipients.join(", ");
        let prompt = format!(
            "Send a weekly team summary email to {} with the following metrics: \
            {} \
            Use Email MCP tools to create an attractive HTML email with charts and \
            formatted data. Include insights and recommendations.",
            recipients_str, team_metrics
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn send_alert(&self, recipients: &[String], alert_type: &str, details: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let recipients_str = recipients.join(", ");
        let prompt = format!(
            "Send an urgent alert email to {} about: {} \
            Details: {} \
            Use Email MCP tools to send a high-priority email with clear action items.",
            recipients_str, alert_type, details
        );
        
        claude.send_prompt(&prompt).await
    }
}