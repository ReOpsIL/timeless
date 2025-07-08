use anyhow::Result;
use crate::claude::ClaudeCliManager;

pub struct JiraService;

impl JiraService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_work_items_for_week(&self, projects: &[String]) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let prompt = format!(
            "Get this week's Jira work items for all users in projects {} - Use Jira MCP tools to retrieve information. \
            Please provide a summary of tickets, their status, assignees, and any blockers.",
            projects.join(", ")
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn get_user_tickets(&self, user_email: &str, project: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let prompt = format!(
            "Get current active Jira tickets for user {} in project {} - Use Jira MCP tools. \
            Include ticket status, priority, and estimated completion time.",
            user_email, project
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn update_ticket_status(&self, ticket_key: &str, new_status: &str, comment: Option<&str>) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let comment_part = comment
            .map(|c| format!(" with comment: '{}'", c))
            .unwrap_or_default();
        
        let prompt = format!(
            "Update Jira ticket {} to status '{}'{} - Use Jira MCP tools to perform the update. \
            Confirm the update was successful and provide the new ticket status.",
            ticket_key, new_status, comment_part
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn create_ticket(&self, project: &str, summary: &str, description: &str, issue_type: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let prompt = format!(
            "Create a new Jira ticket in project {} with the following details: \
            Summary: '{}' \
            Description: '{}' \
            Issue Type: '{}' \
            Use Jira MCP tools to create the ticket and return the ticket key.",
            project, summary, description, issue_type
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn get_project_metrics(&self, project: &str, period: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let prompt = format!(
            "Generate project metrics for Jira project {} for the {} period. \
            Use Jira MCP tools to gather data on: \
            - Number of tickets created, completed, and in progress \
            - Average resolution time \
            - Team velocity \
            - Blockers and impediments \
            Provide a comprehensive analysis.",
            project, period
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn search_tickets(&self, jql_query: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let prompt = format!(
            "Search Jira tickets using JQL query: '{}' - Use Jira MCP tools. \
            Provide a formatted list of matching tickets with key details.",
            jql_query
        );
        
        claude.send_prompt(&prompt).await
    }
}