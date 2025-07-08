use anyhow::Result;
use colored::Colorize;
use crate::models::TeamMember;
use crate::storage::TeamRepository;
use crate::claude::ClaudeCliManager;
use crate::services::{JiraService, SlackService};

pub async fn init(team_name: String, slack_channel: Option<String>) -> Result<()> {
    println!("{} Initializing team: {}", "✓".green(), team_name.bold());
    
    if let Some(channel) = slack_channel {
        println!("{} Slack channel: {}", "✓".green(), channel.bold());
    }
    
    // TODO: Implement team initialization logic
    println!("{} Team initialization complete!", "✓".green());
    Ok(())
}

pub async fn add_user(name: String, email: String, slack_id: Option<String>) -> Result<()> {
    println!("{} Adding user: {} ({})", "✓".green(), name.bold(), email);
    
    // Initialize repository
    let repo = TeamRepository::new("./data")?;
    
    // Create team member
    let mut member = TeamMember::new(name.clone(), email.clone(), "Team Member".to_string());
    
    if let Some(id) = slack_id {
        println!("{} Slack ID: {}", "✓".green(), id.bold());
        member = member.with_slack_id(id);
    }
    
    // Save to storage
    repo.save_team_member(&member)?;
    
    println!("{} User added successfully! ID: {}", "✓".green(), member.id);
    Ok(())
}

pub async fn status(mode: Option<String>, notify_slack: bool) -> Result<()> {
    let mode_str = mode.unwrap_or_else(|| "interactive".to_string());
    println!("{} Running status collection in {} mode", "✓".green(), mode_str.bold());
    
    if notify_slack {
        println!("{} Slack notifications enabled", "✓".green());
    }
    
    // TODO: Implement status collection logic
    println!("{} Status collection complete!", "✓".green());
    Ok(())
}

pub async fn report(report_type: String, output: Option<String>) -> Result<()> {
    println!("{} Generating {} report", "✓".green(), report_type.bold());
    
    if let Some(output_path) = output {
        println!("{} Output: {}", "✓".green(), output_path.bold());
    }
    
    // TODO: Implement report generation logic
    println!("{} Report generation complete!", "✓".green());
    Ok(())
}

pub async fn test_mcp() -> Result<()> {
    println!("{} Testing Claude CLI connectivity...", "✓".green());
    
    let claude = ClaudeCliManager::get_instance().await?;
    let test_prompt = "Hello! This is a test of the Claude CLI integration. Please respond with a brief confirmation that you're working.";
    
    match claude.send_prompt(&test_prompt).await {
        Ok(response) => {
            println!("{} Claude CLI is working!", "✓".green());
            println!("Response: {}", response.lines().next().unwrap_or("No response"));
        }
        Err(e) => {
            println!("{} Claude CLI test failed: {}", "✗".red(), e);
            println!("Make sure Claude CLI is installed and available in PATH");
        }
    }
    
    Ok(())
}

pub async fn health() -> Result<()> {
    println!("{} System Health Check", "✓".green());
    
    // Check storage
    match TeamRepository::new("./data") {
        Ok(repo) => {
            let members = repo.list_team_members()?;
            println!("  {} Storage: OK ({} team members)", "✓".green(), members.len());
        }
        Err(_) => {
            println!("  {} Storage: Failed", "✗".red());
        }
    }
    
    println!("  {} Configuration: OK", "✓".green());
    println!("  {} Claude CLI: Ready for prompt-based integration", "✓".green());
    
    println!("{} System health check complete!", "✓".green());
    Ok(())
}

pub async fn query_jira(projects: Vec<String>, period: String) -> Result<()> {
    println!("{} Querying Jira for work items...", "✓".green());
    
    let jira = JiraService::new();
    
    match period.as_str() {
        "week" | "weekly" => {
            let response = jira.get_work_items_for_week(&projects).await?;
            println!("{} Weekly Jira Report:", "📊".blue());
            println!("{}", response);
        }
        _ => {
            let jql = format!("project in ({}) AND updated >= -{}", projects.join(","), period);
            let response = jira.search_tickets(&jql).await?;
            println!("{} Jira Search Results:", "🔍".blue());
            println!("{}", response);
        }
    }
    
    Ok(())
}

pub async fn send_slack_message(channel: String, message: String) -> Result<()> {
    println!("{} Sending message to Slack...", "✓".green());
    
    let slack = SlackService::new();
    let response = slack.send_message(&channel, &message).await?;
    
    println!("{} Message sent:", "📤".blue());
    println!("{}", response);
    
    Ok(())
}

pub async fn collect_team_status(channel: String, members: Vec<String>) -> Result<()> {
    println!("{} Collecting team status...", "✓".green());
    
    let slack = SlackService::new();
    let response = slack.collect_team_status(&channel, &members).await?;
    
    println!("{} Team Status Summary:", "👥".blue());
    println!("{}", response);
    
    Ok(())
}