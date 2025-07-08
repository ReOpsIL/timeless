use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub slack_id: Option<String>,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TeamMember {
    pub fn new(name: String, email: String, role: String) -> Self {
        let now = Utc::now();
        TeamMember {
            id: Uuid::new_v4(),
            name,
            email,
            slack_id: None,
            role,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn with_slack_id(mut self, slack_id: String) -> Self {
        self.slack_id = Some(slack_id);
        self.updated_at = Utc::now();
        self
    }
    
    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(name: String, description: String) -> Self {
        let now = Utc::now();
        Project {
            id: Uuid::new_v4(),
            name,
            description,
            status: ProjectStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn set_status(&mut self, status: ProjectStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    
    pub fn is_active(&self) -> bool {
        matches!(self.status, ProjectStatus::Active)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Active,
    Completed,
    OnHold,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusUpdate {
    pub id: Uuid,
    pub member_id: Uuid,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub mood: Option<String>,
    pub blockers: Vec<String>,
    pub achievements: Vec<String>,
}

impl StatusUpdate {
    pub fn new(member_id: Uuid, content: String) -> Self {
        StatusUpdate {
            id: Uuid::new_v4(),
            member_id,
            content,
            timestamp: Utc::now(),
            mood: None,
            blockers: Vec::new(),
            achievements: Vec::new(),
        }
    }
    
    pub fn with_mood(mut self, mood: String) -> Self {
        self.mood = Some(mood);
        self
    }
    
    pub fn add_blocker(&mut self, blocker: String) {
        self.blockers.push(blocker);
    }
    
    pub fn add_achievement(&mut self, achievement: String) {
        self.achievements.push(achievement);
    }
    
    pub fn has_blockers(&self) -> bool {
        !self.blockers.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub member_id: Uuid,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIDecision {
    pub id: Uuid,
    pub decision_type: String,
    pub context: String,
    pub recommendation: String,
    pub confidence: f32,
    pub created_at: DateTime<Utc>,
    pub outcome: Option<String>,
}

impl AIDecision {
    pub fn new(decision_type: String, context: String, recommendation: String, confidence: f32) -> Self {
        AIDecision {
            id: Uuid::new_v4(),
            decision_type,
            context,
            recommendation,
            confidence,
            created_at: Utc::now(),
            outcome: None,
        }
    }
    
    pub fn set_outcome(&mut self, outcome: String) {
        self.outcome = Some(outcome);
    }
    
    pub fn is_high_confidence(&self) -> bool {
        self.confidence >= 0.8
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMetrics {
    pub id: Uuid,
    pub date: DateTime<Utc>,
    pub active_members: u32,
    pub completed_tasks: u32,
    pub blockers_count: u32,
    pub average_satisfaction: f32,
    pub velocity: f32,
}

impl TeamMetrics {
    pub fn new(date: DateTime<Utc>) -> Self {
        TeamMetrics {
            id: Uuid::new_v4(),
            date,
            active_members: 0,
            completed_tasks: 0,
            blockers_count: 0,
            average_satisfaction: 0.0,
            velocity: 0.0,
        }
    }
    
    pub fn calculate_health_score(&self) -> f32 {
        let satisfaction_weight = 0.4;
        let velocity_weight = 0.3;
        let blocker_penalty = 0.3;
        
        let satisfaction_score = self.average_satisfaction / 10.0; // Normalize to 0-1
        let velocity_score = (self.velocity / 100.0).min(1.0); // Normalize to 0-1, cap at 1
        let blocker_score = if self.active_members > 0 {
            1.0 - (self.blockers_count as f32 / self.active_members as f32).min(1.0)
        } else {
            1.0
        };
        
        (satisfaction_score * satisfaction_weight 
         + velocity_score * velocity_weight 
         + blocker_score * blocker_penalty) * 10.0
    }
    
    pub fn has_concerning_metrics(&self) -> bool {
        self.average_satisfaction < 5.0 
            || (self.active_members > 0 && (self.blockers_count as f32 / self.active_members as f32) > 0.5)
            || self.velocity < 10.0
    }
}