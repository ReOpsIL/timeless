use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "timeless")]
#[command(about = "Smart Team Manager - AI-powered team management tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, default_value = "./config/config.toml")]
    config: String,
    
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize team configuration
    Init {
        #[arg(short, long)]
        team_name: String,
        #[arg(short, long)]
        slack_channel: Option<String>,
    },
    /// Add a team member
    AddUser {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        slack_id: Option<String>,
    },
    /// Run status collection
    Status {
        #[arg(short, long)]
        mode: Option<String>,
        #[arg(short, long)]
        notify_slack: bool,
    },
    /// Generate team reports
    Report {
        #[arg(short, long)]
        report_type: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Test MCP server connections
    TestMcp,
    /// Show system status
    Health,
    /// Query Jira for work items using prompts
    QueryJira {
        #[arg(short, long)]
        projects: Vec<String>,
        #[arg(short = 'e', long, default_value = "week")]
        period: String,
    },
    /// Send message to Slack using prompts
    SlackMessage {
        #[arg(short, long)]
        channel: String,
        #[arg(short, long)]
        message: String,
    },
    /// Collect team status via Slack using prompts
    TeamStatus {
        #[arg(short, long)]
        channel: String,
        #[arg(short, long)]
        members: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    env_logger::init();
    
    // Load configuration
    let _config = timeless::config::load_config(&cli.config)?;
    
    // Execute command
    match cli.command {
        Commands::Init { team_name, slack_channel } => {
            timeless::cli::commands::init(team_name, slack_channel).await
        },
        Commands::AddUser { name, email, slack_id } => {
            timeless::cli::commands::add_user(name, email, slack_id).await
        },
        Commands::Status { mode, notify_slack } => {
            timeless::cli::commands::status(mode, notify_slack).await
        },
        Commands::Report { report_type, output } => {
            timeless::cli::commands::report(report_type, output).await
        },
        Commands::TestMcp => {
            timeless::cli::commands::test_mcp().await
        },
        Commands::Health => {
            timeless::cli::commands::health().await
        },
        Commands::QueryJira { projects, period } => {
            timeless::cli::commands::query_jira(projects, period).await
        },
        Commands::SlackMessage { channel, message } => {
            timeless::cli::commands::send_slack_message(channel, message).await
        },
        Commands::TeamStatus { channel, members } => {
            timeless::cli::commands::collect_team_status(channel, members).await
        },
    }
}
