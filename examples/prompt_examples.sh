#!/bin/bash
# Timeless Prompt-Based Service Integration Examples

echo "=== Timeless Prompt-Based Service Integration Examples ==="
echo ""

# Build the project first
echo "🔧 Building Timeless..."
cargo build --release
echo ""

# Test basic functionality
echo "🏥 Testing system health..."
./target/release/timeless health
echo ""

echo "🧪 Testing Claude CLI with MCP tools..."
./target/release/timeless test-mcp
echo ""

# Jira Integration Examples
echo "📊 === JIRA INTEGRATION EXAMPLES ==="
echo ""

echo "1. Get this week's work items for multiple projects:"
echo "Command: timeless query-jira --projects TIM PROJ AAA --period week"
echo "Prompt sent to Claude: 'Get this week's Jira work items for all users in projects TIM, PROJ, AAA - Use Jira MCP tools to retrieve information...'"
echo ""

echo "2. Get work items for the last month:"
echo "Command: timeless query-jira --projects TIM --period 1month"
echo "Prompt sent to Claude: Uses JQL query with time period filtering via Jira MCP tools"
echo ""

# Slack Integration Examples  
echo "💬 === SLACK INTEGRATION EXAMPLES ==="
echo ""

echo "3. Send team announcement:"
echo "Command: timeless slack-message --channel '#general' --message 'New release v2.1 deployed successfully!'"
echo "Prompt sent to Claude: 'Send a message to Slack channel #general with content: \"New release v2.1 deployed successfully!\" - Use Slack MCP tools...'"
echo ""

echo "4. Collect team status from multiple members:"
echo "Command: timeless team-status --channel '#engineering' --members alice bob charlie"
echo "Prompt sent to Claude: 'Collect status updates from team members alice, bob, charlie in channel #engineering - Use Slack MCP tools...'"
echo ""

# Example with real commands (commented out - replace with your actual data)
echo "🚀 === EXAMPLE EXECUTIONS (Update with your data) ==="
echo ""

echo "# Example 1: Query Jira work items (replace TIM with your project key)"
echo "# ./target/release/timeless query-jira --projects TIM --period week"
echo ""

echo "# Example 2: Send Slack message (replace with your channel)"  
echo "# ./target/release/timeless slack-message --channel '#test' --message 'Hello from Timeless!'"
echo ""

echo "# Example 3: Collect team status (replace with your channel and members)"
echo "# ./target/release/timeless team-status --channel '#engineering' --members john jane mike"
echo ""

# Show the difference from old approach
echo "🔄 === ARCHITECTURE COMPARISON ==="
echo ""

echo "OLD APPROACH (Removed):"
echo "- Direct MCP server management"
echo "- Hardcoded API calls"  
echo "- Complex error handling"
echo "- Server lifecycle management"
echo ""

echo "NEW PROMPT-BASED APPROACH:"
echo "- Natural language instructions to Claude CLI"
echo "- Claude internally uses MCP tools"
echo "- Human-readable responses"
echo "- Zero server management"
echo ""

echo "PROMPT EXAMPLES:"
echo ""
echo "Jira Query Prompt:"
echo "'Get this week's Jira work items for all users in projects TIM, PROJ - Use Jira MCP tools to retrieve information. Please provide a summary of tickets, their status, assignees, and any blockers.'"
echo ""

echo "Slack Message Prompt:"  
echo "'Send a message to Slack channel #engineering with content: \"Daily standup in 5 minutes!\" - Use Slack MCP tools. Confirm the message was sent successfully.'"
echo ""

echo "Team Status Collection Prompt:"
echo "'Collect status updates from team members alice, bob, charlie in channel #engineering - Use Slack MCP tools. Ask each member for their current work status, blockers, and plans for today. Provide a summary of all responses.'"
echo ""

echo "✨ All external service interactions now use natural language prompts!"
echo "✨ Claude CLI handles MCP tool orchestration automatically!"
echo "✨ No more hardcoded business logic or server management!"