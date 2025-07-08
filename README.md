# Timeless - Smart Team Manager Tool

An intelligent, prompt-driven team management platform powered by Claude CLI that uses natural language to interact with external services via MCP tools.

## 🚀 Architecture Overview

**Timeless** eliminates traditional hardcoded integrations in favor of **natural language prompts** sent to Claude CLI, which internally leverages MCP tools to interact with external services.

### Key Innovation: Prompt-Based Service Interactions

```
┌─────────────────┐    Natural Language    ┌─────────────────┐    MCP Tools    ┌─────────────────┐
│   Timeless CLI  │ ────────prompts───────► │   Claude CLI    │ ──────────────► │ External Services│
│                 │                        │                 │                │ (Jira, Slack,    │
│                 │ ◄──────responses────── │                 │ ◄────────────── │  GitHub, Email)  │
└─────────────────┘                        └─────────────────┘                └─────────────────┘
```

## ✨ Benefits

- **Zero hardcoded business logic** - All service interactions use natural language
- **No server management** - Claude CLI handles all MCP tool orchestration
- **Dynamic adaptation** - Claude adjusts queries based on context
- **Simplified maintenance** - Add new integrations without code changes
- **Natural error handling** - Human-readable error messages and suggestions

## 📋 Features

### Jira Integration via Prompts
```bash
# Get this week's work items for multiple projects
timeless query-jira --projects TIM PROJ AAA --period week

# The system sends this prompt to Claude CLI:
# "Get this week's Jira work items for all users in projects TIM, PROJ, AAA - 
#  Use Jira MCP tools to retrieve information. Please provide a summary of 
#  tickets, their status, assignees, and any blockers."
```

### Slack Integration via Prompts
```bash
# Send team announcement
timeless slack-message --channel "#general" --message "New release deployed!"

# Collect team status
timeless team-status --channel "#engineering" --members john jane mike

# The system automatically creates contextual prompts like:
# "Collect status updates from team members john, jane, mike in channel #engineering - 
#  Use Slack MCP tools. Ask each member for their current work status, blockers, 
#  and plans for today. Provide a summary of all responses."
```

### GitHub Integration via Prompts
```bash
# Query user issues and PRs (implementation ready)
timeless github-issues --user john --repo myproject

# The prompt sent to Claude CLI:
# "Get current open issues assigned to GitHub user john in repository myproject - 
#  Use GitHub MCP tools. Include issue titles, labels, creation dates, and current status."
```

### Email Integration via Prompts
```bash
# Send team reports (implementation ready)
timeless email-report --recipients "manager@company.com" --subject "Weekly Report"

# The prompt sent to Claude CLI:
# "Send an email report to recipients: manager@company.com Subject: 'Weekly Report' 
#  Content: '[generated content]' Use Email MCP tools to send a well-formatted HTML email."
```

## 🛠 Installation

1. **Install Claude CLI** (required dependency)
```bash
# Install Claude CLI with MCP tools support
npm install -g @anthropic-ai/claude-cli
```

2. **Build Timeless**
```bash
git clone <repository>
cd timeless
cargo build --release
```

3. **Configure Environment**
```bash
cp .env.example .env
# Edit .env with your API keys and tokens
```

## 🏗 Project Structure

```
timeless/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── cli/                 # Command-line interface
│   ├── claude/              # Claude CLI integration
│   │   ├── process.rs       # Claude CLI process management
│   │   ├── client.rs        # Claude communication
│   │   └── prompts.rs       # Prompt templates
│   ├── services/            # Prompt-based service layer
│   │   ├── jira.rs          # Jira via prompts
│   │   ├── slack.rs         # Slack via prompts  
│   │   ├── github.rs        # GitHub via prompts
│   │   └── email.rs         # Email via prompts
│   ├── models/              # Data models
│   ├── storage/             # JSON file storage
│   └── intelligence/        # AI decision engine
├── config/                  # Configuration files
└── data/                    # JSON data files
```

## 🔧 Usage Examples

### Basic Commands

```bash
# Initialize team configuration
timeless init --team "Engineering Team" --slack-channel "#engineering"

# Add team member
timeless add-user --name "John Doe" --email "john@company.com"

# Test Claude CLI with MCP tools
timeless test-mcp

# Check system health
timeless health
```

### Prompt-Based Service Interactions

```bash
# Query Jira work items using natural language prompts
timeless query-jira --projects TIM PROJ --period week
# → Sends: "Get this week's Jira work items for all users in projects TIM, PROJ..."

# Send Slack messages via prompts
timeless slack-message --channel "#general" --message "Daily standup in 5 minutes!"
# → Sends: "Send a message to Slack channel #general with content: 'Daily standup in 5 minutes!'..."

# Collect team status via prompts
timeless team-status --channel "#engineering" --members alice bob charlie
# → Sends: "Collect status updates from team members alice, bob, charlie in channel #engineering..."
```

## 🧠 How Prompt-Based Integration Works

### Traditional Approach (Removed)
```rust
// OLD: Direct MCP server integration
let mcp_client = McpClient::connect("jira-server").await?;
let response = mcp_client.call_tool("get_issues", params).await?;
// Requires: Server management, error handling, API knowledge
```

### New Prompt-Based Approach
```rust
// NEW: Natural language prompts
let claude = ClaudeCliManager::get_instance().await?;
let prompt = "Get this week's Jira work items for projects TIM, PROJ - Use Jira MCP tools";
let response = claude.send_prompt(&prompt).await?;
// Claude CLI handles: MCP tool selection, execution, error handling, response formatting
```

## 🎯 Service Implementation Examples

### Jira Service (src/services/jira.rs)

```rust
pub async fn get_work_items_for_week(&self, projects: &[String]) -> Result<String> {
    let claude = ClaudeCliManager::get_instance().await?;
    
    let prompt = format!(
        "Get this week's Jira work items for all users in projects {} - \
         Use Jira MCP tools to retrieve information. Please provide a summary \
         of tickets, their status, assignees, and any blockers.",
        projects.join(", ")
    );
    
    claude.send_prompt(&prompt).await
}
```

### Slack Service (src/services/slack.rs)

```rust
pub async fn collect_team_status(&self, channel: &str, team_members: &[String]) -> Result<String> {
    let claude = ClaudeCliManager::get_instance().await?;
    
    let prompt = format!(
        "Collect status updates from team members {} in channel {} - \
         Use Slack MCP tools. Ask each member for their current work status, \
         blockers, and plans for today. Provide a summary of all responses.",
        team_members.join(", "), channel
    );
    
    claude.send_prompt(&prompt).await
}
```

## 🔒 Configuration

### Environment Variables (.env)
```bash
# Claude CLI configuration
CLAUDE_API_KEY=sk-ant-your-api-key-here

# MCP tool configurations (handled by Claude CLI)
SLACK_BOT_TOKEN=xoxb-your-bot-token
JIRA_TOKEN=your-jira-api-token
GITHUB_TOKEN=ghp_your-github-token
EMAIL_USER=your-email@company.com
```

### Application Configuration (config.toml)
```toml
[app]
name = "Smart Team Manager"
data_dir = "./data"

[claude]
enabled = true
model = "claude-3-5-sonnet-20241022"
max_tokens = 4000

[team]
name = "Engineering Team"
timezone = "UTC"
```

## 🚀 Getting Started

1. **Quick Test**
```bash
# Test basic functionality
timeless health
timeless test-mcp
```

2. **Initialize Your Team**
```bash
timeless init --team "My Team" --slack-channel "#general"
timeless add-user --name "Alice" --email "alice@company.com"
```

3. **Try Prompt-Based Queries**
```bash
# Query Jira (replace with your project keys)
timeless query-jira --projects MYPROJ --period week

# Send Slack message (replace with your channel)
timeless slack-message --channel "#test" --message "Hello from Timeless!"
```

## 📚 Key Concepts

### Prompt Engineering for Services
- **Specific instructions**: "Use Jira MCP tools to retrieve information"
- **Clear expectations**: "Provide a summary of tickets, their status, assignees"
- **Context inclusion**: Project names, time periods, user lists
- **Output formatting**: "Formatted list", "HTML email", "summary"

### Natural Language Error Handling
- Claude CLI provides human-readable error messages
- Context-aware suggestions for resolving issues
- No need for API error code handling

### Zero Configuration Service Integration
- No MCP server setup required
- No API endpoint configuration
- Claude CLI manages all MCP tool connections

## 🎯 Next Steps

1. **Extend service prompts** - Add more sophisticated natural language queries
2. **Add new services** - Create prompt-based integrations for Calendar, Notion, etc.
3. **Enhance intelligence** - Use Claude for analysis and decision-making prompts
4. **Automate workflows** - Create scheduled prompt-based automations

## 🤝 Contributing

This project demonstrates the power of prompt-based service integration. Contributions that extend the natural language approach are welcome!

## 📄 License

MIT License - See LICENSE file for details.