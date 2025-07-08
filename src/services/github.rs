use anyhow::Result;
use crate::claude::ClaudeCliManager;

pub struct GitHubService;

impl GitHubService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_user_issues(&self, username: &str, repo: Option<&str>) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let repo_filter = repo
            .map(|r| format!(" in repository {}", r))
            .unwrap_or_default();
        
        let prompt = format!(
            "Get current open issues assigned to GitHub user {}{} - Use GitHub MCP tools. \
            Include issue titles, labels, creation dates, and current status.",
            username, repo_filter
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn get_pull_requests(&self, username: &str, repo: Option<&str>) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let repo_filter = repo
            .map(|r| format!(" in repository {}", r))
            .unwrap_or_default();
        
        let prompt = format!(
            "Get open pull requests for GitHub user {}{} - Use GitHub MCP tools. \
            Include PR titles, status, review status, and target branch.",
            username, repo_filter
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn create_issue(&self, repo: &str, title: &str, body: &str, labels: &[String]) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let labels_str = if labels.is_empty() {
            String::new()
        } else {
            format!(" with labels: {}", labels.join(", "))
        };
        
        let prompt = format!(
            "Create a new GitHub issue in repository {} with title '{}' and body: '{}'{} - \
            Use GitHub MCP tools to create the issue and return the issue URL.",
            repo, title, body, labels_str
        );
        
        claude.send_prompt(&prompt).await
    }

    pub async fn get_repository_metrics(&self, repo: &str, period: &str) -> Result<String> {
        let claude = ClaudeCliManager::get_instance().await?;
        
        let prompt = format!(
            "Get repository metrics for {} for the {} period - Use GitHub MCP tools. \
            Include: \
            - Number of commits \
            - Number of issues opened/closed \
            - Number of PRs opened/merged \
            - Top contributors \
            - Code review metrics",
            repo, period
        );
        
        claude.send_prompt(&prompt).await
    }
}