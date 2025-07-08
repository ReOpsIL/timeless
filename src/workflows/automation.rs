use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomatedWorkflow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub triggers: Vec<WorkflowTrigger>,
    pub conditions: Vec<WorkflowCondition>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: Uuid,
    pub name: String,
    pub action: WorkflowAction,
    pub parameters: HashMap<String, String>,
    pub retry_count: u32,
    pub timeout_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowAction {
    SendSlackMessage,
    SendEmail,
    CollectStatus,
    GenerateReport,
    AnalyzeTeam,
    UpdateJira,
    CreateBackup,
    NotifyManager,
    RunClaudeAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowTrigger {
    TimeSchedule(String), // Cron expression
    StatusUpdate,
    TeamMemberJoined,
    ProjectCompleted,
    RiskDetected,
    MetricThreshold(String, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCondition {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    NotContains,
}

impl AutomatedWorkflow {
    pub fn new(name: String, description: String) -> Self {
        AutomatedWorkflow {
            id: Uuid::new_v4(),
            name,
            description,
            steps: Vec::new(),
            triggers: Vec::new(),
            conditions: Vec::new(),
            enabled: true,
        }
    }
    
    pub fn add_step(&mut self, step: WorkflowStep) {
        self.steps.push(step);
    }
    
    pub fn add_trigger(&mut self, trigger: WorkflowTrigger) {
        self.triggers.push(trigger);
    }
    
    pub fn add_condition(&mut self, condition: WorkflowCondition) {
        self.conditions.push(condition);
    }
    
    pub async fn execute(&self, context: &HashMap<String, String>) -> Result<WorkflowResult> {
        if !self.enabled {
            return Ok(WorkflowResult::Skipped("Workflow is disabled".to_string()));
        }
        
        // Check conditions
        if !self.check_conditions(context) {
            return Ok(WorkflowResult::Skipped("Conditions not met".to_string()));
        }
        
        let mut step_results = Vec::new();
        
        for step in &self.steps {
            match self.execute_step(step, context).await {
                Ok(result) => {
                    step_results.push(StepResult {
                        step_id: step.id,
                        success: true,
                        result: Some(result),
                        error: None,
                    });
                }
                Err(e) => {
                    step_results.push(StepResult {
                        step_id: step.id,
                        success: false,
                        result: None,
                        error: Some(e.to_string()),
                    });
                    
                    // Stop execution on error
                    break;
                }
            }
        }
        
        Ok(WorkflowResult::Completed(step_results))
    }
    
    async fn execute_step(&self, step: &WorkflowStep, _context: &HashMap<String, String>) -> Result<String> {
        match &step.action {
            WorkflowAction::SendSlackMessage => {
                let default_channel = "#general".to_string();
                let channel = step.parameters.get("channel").unwrap_or(&default_channel);
                let default_message = "Automated message".to_string();
                let message = step.parameters.get("message").unwrap_or(&default_message);
                Ok(format!("Sent Slack message to {}: {}", channel, message))
            }
            WorkflowAction::SendEmail => {
                let default_to = "team@company.com".to_string();
                let to = step.parameters.get("to").unwrap_or(&default_to);
                let default_subject = "Automated Email".to_string();
                let subject = step.parameters.get("subject").unwrap_or(&default_subject);
                Ok(format!("Sent email to {} with subject: {}", to, subject))
            }
            WorkflowAction::CollectStatus => {
                Ok("Status collection initiated".to_string())
            }
            WorkflowAction::GenerateReport => {
                let default_type = "daily".to_string();
                let report_type = step.parameters.get("type").unwrap_or(&default_type);
                Ok(format!("Generated {} report", report_type))
            }
            WorkflowAction::AnalyzeTeam => {
                Ok("Team analysis completed".to_string())
            }
            WorkflowAction::UpdateJira => {
                let default_ticket = "UNKNOWN".to_string();
                let ticket = step.parameters.get("ticket").unwrap_or(&default_ticket);
                Ok(format!("Updated Jira ticket: {}", ticket))
            }
            WorkflowAction::CreateBackup => {
                Ok("Data backup created".to_string())
            }
            WorkflowAction::NotifyManager => {
                let default_manager = "manager@company.com".to_string();
                let manager = step.parameters.get("manager").unwrap_or(&default_manager);
                Ok(format!("Notified manager: {}", manager))
            }
            WorkflowAction::RunClaudeAnalysis => {
                let default_analysis = "general".to_string();
                let analysis_type = step.parameters.get("analysis_type").unwrap_or(&default_analysis);
                Ok(format!("Claude analysis completed: {}", analysis_type))
            }
        }
    }
    
    fn check_conditions(&self, context: &HashMap<String, String>) -> bool {
        for condition in &self.conditions {
            if !self.evaluate_condition(condition, context) {
                return false;
            }
        }
        true
    }
    
    fn evaluate_condition(&self, condition: &WorkflowCondition, context: &HashMap<String, String>) -> bool {
        let default_value = String::new();
        let context_value = context.get(&condition.field).unwrap_or(&default_value);
        
        match condition.operator {
            ConditionOperator::Equals => context_value == &condition.value,
            ConditionOperator::NotEquals => context_value != &condition.value,
            ConditionOperator::GreaterThan => {
                context_value.parse::<f64>().unwrap_or(0.0) > condition.value.parse::<f64>().unwrap_or(0.0)
            }
            ConditionOperator::LessThan => {
                context_value.parse::<f64>().unwrap_or(0.0) < condition.value.parse::<f64>().unwrap_or(0.0)
            }
            ConditionOperator::Contains => context_value.contains(&condition.value),
            ConditionOperator::NotContains => !context_value.contains(&condition.value),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowResult {
    Completed(Vec<StepResult>),
    Failed(String),
    Skipped(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_id: Uuid,
    pub success: bool,
    pub result: Option<String>,
    pub error: Option<String>,
}