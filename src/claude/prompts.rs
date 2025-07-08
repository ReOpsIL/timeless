use std::collections::HashMap;

pub struct PromptTemplate {
    template: String,
    variables: HashMap<String, String>,
}

impl PromptTemplate {
    pub fn new(template: &str) -> Self {
        PromptTemplate {
            template: template.to_string(),
            variables: HashMap::new(),
        }
    }
    
    pub fn set_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }
    
    pub fn render(&self) -> String {
        let mut result = self.template.clone();
        for (key, value) in &self.variables {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }
}

pub const STATUS_COLLECTION_PROMPT: &str = r#"
You are a smart team manager assistant. Based on the following context:
- Team member: {name}
- Current tasks: {tasks}
- Recent activity: {activity}
- Team context: {team_context}

Generate a personalized, contextual check-in message that:
1. Acknowledges their current work
2. Asks about progress and blockers
3. Maintains a supportive tone
4. Keeps it concise (2-3 sentences)
"#;

pub const TEAM_ANALYSIS_PROMPT: &str = r#"
You are an AI team management analyst. Analyze the following team data:
- Team status updates: {status_updates}
- Recent metrics: {metrics}
- Current projects: {projects}

Provide insights on:
1. Team performance trends
2. Potential risks or blockers
3. Recommendations for improvement
4. Individual team member highlights
"#;

pub const REPORT_GENERATION_PROMPT: &str = r#"
Generate a {report_type} report for the team based on:
- Time period: {time_period}
- Team data: {team_data}
- Metrics: {metrics}

Format the report with:
1. Executive summary
2. Key achievements
3. Challenges and blockers
4. Recommendations
5. Next steps
"#;