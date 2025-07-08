# Prompt-Based Service Architecture

This document explains the architectural shift from direct MCP server integration to prompt-based service interactions via Claude CLI.

## Architecture Philosophy

**Traditional Integration**: Write code to manage MCP servers, handle API calls, parse responses, manage errors.

**Prompt-Based Integration**: Send natural language instructions to Claude CLI, which uses its MCP tools to fulfill requests.

## Benefits of Prompt-Based Approach

### 1. Zero Server Management
- **Before**: Start MCP servers, manage connections, handle restarts
- **After**: Claude CLI manages all MCP tool connections internally

### 2. Natural Language Interface
- **Before**: Learn API schemas, construct JSON payloads, handle responses
- **After**: Write natural language descriptions of what you want

### 3. Dynamic Adaptation
- **Before**: Hardcoded logic for different scenarios
- **After**: Claude adapts queries based on context and requirements

### 4. Simplified Error Handling
- **Before**: Parse error codes, handle various failure modes
- **After**: Receive natural language error descriptions and suggestions

## Implementation Patterns

### Service Method Pattern

```rust
pub async fn service_action(&self, params: &Type) -> Result<String> {
    let claude = ClaudeCliManager::get_instance().await?;
    
    let prompt = format!(
        "Perform [specific action] with [parameters] - Use [Service] MCP tools. 
         [Specific instructions about output format, error handling, etc.]"
    );
    
    claude.send_prompt(&prompt).await
}
```

### Prompt Construction Guidelines

1. **Be Specific**: Clearly state what action to perform
2. **Name the Tools**: Explicitly mention which MCP tools to use
3. **Define Output**: Specify the format and content of expected response
4. **Include Context**: Provide relevant parameters and constraints
5. **Handle Edge Cases**: Mention error conditions or special scenarios

## Service Implementation Examples

### Jira Service Prompts

```rust
// Weekly work items query
let prompt = format!(
    "Get this week's Jira work items for all users in projects {} - 
     Use Jira MCP tools to retrieve information. Please provide a summary 
     of tickets, their status, assignees, and any blockers.",
    projects.join(", ")
);

// User-specific tickets
let prompt = format!(
    "Get current active Jira tickets for user {} in project {} - 
     Use Jira MCP tools. Include ticket status, priority, and estimated 
     completion time.",
    user_email, project
);

// Status update
let prompt = format!(
    "Update Jira ticket {} to status '{}'{} - Use Jira MCP tools to 
     perform the update. Confirm the update was successful and provide 
     the new ticket status.",
    ticket_key, new_status, comment_part
);
```

### Slack Service Prompts

```rust
// Message sending
let prompt = format!(
    "Send a message to Slack channel {} with content: '{}' - 
     Use Slack MCP tools. Confirm the message was sent successfully.",
    channel, message
);

// Status collection
let prompt = format!(
    "Collect status updates from team members {} in channel {} - 
     Use Slack MCP tools. Ask each member for their current work status, 
     blockers, and plans for today. Provide a summary of all responses.",
    members_list, channel
);
```

### GitHub Service Prompts

```rust
// Issue retrieval
let prompt = format!(
    "Get current open issues assigned to GitHub user {}{} - 
     Use GitHub MCP tools. Include issue titles, labels, creation dates, 
     and current status.",
    username, repo_filter
);

// Pull request queries
let prompt = format!(
    "Get open pull requests for GitHub user {}{} - Use GitHub MCP tools. 
     Include PR titles, status, review status, and target branch.",
    username, repo_filter
);
```

### Email Service Prompts

```rust
// Report distribution
let prompt = format!(
    "Send an email report to recipients: {} Subject: '{}' Content: '{}' 
     Use Email MCP tools to send a well-formatted HTML email. 
     Confirm delivery status.",
    recipients_str, subject, content
);

// Alert notifications
let prompt = format!(
    "Send an urgent alert email to {} about: {} Details: {} 
     Use Email MCP tools to send a high-priority email with clear action items.",
    recipients_str, alert_type, details
);
```

## Claude CLI Process Management

### Singleton Pattern
- Single Claude CLI process shared across all service calls
- Managed via `ClaudeCliManager::get_instance()`
- Automatic process lifecycle management

### Communication Protocol
- Pseudo-TTY for interactive communication
- Request/response correlation using unique IDs
- Timeout handling (30 seconds default)
- Graceful error recovery

```rust
pub struct ClaudeCliProcess {
    pty_master: Arc<Mutex<Box<dyn portable_pty::MasterPty + Send>>>,
    writer: Arc<Mutex<FramedWrite<portable_pty::PtyWriter, LinesCodec>>>,
    pending_responses: Arc<Mutex<HashMap<String, oneshot::Sender<String>>>>,
    is_running: Arc<Mutex<bool>>,
}
```

## CLI Command Integration

### Command Structure
Commands translate user inputs into service method calls:

```rust
// CLI: timeless query-jira --projects TIM PROJ --period week
Commands::QueryJira { projects, period } => {
    let jira = JiraService::new();
    match period.as_str() {
        "week" | "weekly" => {
            let response = jira.get_work_items_for_week(&projects).await?;
            println!("{}", response);
        }
        // ... other periods
    }
}
```

### User Experience
```bash
$ timeless query-jira --projects TIM AAA BBB --period week
✓ Querying Jira for work items...
📊 Weekly Jira Report:
This week's work items for projects TIM, AAA, BBB:

[Claude's natural language response with ticket summaries,
 status updates, assignee information, and blocker analysis]
```

## Error Handling Strategy

### Natural Language Errors
Claude CLI provides context-aware error messages:
- "Unable to connect to Jira. Please check your authentication token."
- "Project 'INVALID' not found. Available projects: TIM, AAA, BBB"
- "Channel '#nonexistent' not accessible. Verify channel name and bot permissions."

### Graceful Degradation
```rust
match claude.send_prompt(&prompt).await {
    Ok(response) => {
        println!("✓ Success: {}", response);
    }
    Err(e) => {
        println!("✗ Error: {}", e);
        // Claude provides actionable error information
    }
}
```

## Future Extensions

### Adding New Services
To add a new service (e.g., Calendar):

1. **Create service module** (`src/services/calendar.rs`)
2. **Implement prompt-based methods**:
   ```rust
   pub async fn schedule_meeting(&self, attendees: &[String], title: &str) -> Result<String> {
       let prompt = format!(
           "Schedule a meeting with attendees {} titled '{}' - 
            Use Calendar MCP tools. Find a suitable time slot and send invitations.",
           attendees.join(", "), title
       );
       claude.send_prompt(&prompt).await
   }
   ```
3. **Add CLI command**:
   ```rust
   ScheduleMeeting {
       attendees: Vec<String>,
       title: String,
   }
   ```

### Enhanced Prompting
- **Context injection**: Include previous conversation history
- **Multi-step workflows**: Chain prompts for complex operations
- **Conditional logic**: Let Claude decide which tools to use
- **Learning integration**: Improve prompts based on response quality

## Best Practices

### Prompt Design
1. **Start with action verb**: "Get", "Send", "Update", "Create"
2. **Specify tools explicitly**: "Use Jira MCP tools"
3. **Define success criteria**: "Confirm the update was successful"
4. **Include output format**: "Provide a summary", "Return the ticket key"
5. **Handle edge cases**: "If project not found, list available projects"

### Service Implementation
1. **Keep methods focused**: One primary action per method
2. **Use Result<String>**: Consistent return type for all responses
3. **Include parameter validation**: Check inputs before sending prompts
4. **Add helpful error context**: Enhance error messages with suggestions
5. **Document prompt patterns**: Include examples in code comments

### Testing Strategy
1. **Integration tests**: Verify full prompt-to-response flow
2. **Mock Claude responses**: Test service logic with known responses
3. **Error scenario testing**: Validate graceful error handling
4. **Performance testing**: Ensure reasonable response times
5. **Prompt effectiveness**: Measure quality of Claude's responses

This prompt-based architecture represents a paradigm shift from traditional API integration to natural language service orchestration, eliminating complexity while maintaining full functionality.