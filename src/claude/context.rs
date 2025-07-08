use crate::models::{Message, MessageRole};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub id: Uuid,
    pub member_id: Option<Uuid>,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub context_data: ContextData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextData {
    pub team_name: String,
    pub current_project: Option<String>,
    pub recent_activities: Vec<String>,
    pub preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub communication_style: CommunicationStyle,
    pub notification_frequency: NotificationFrequency,
    pub preferred_channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Formal,
    Casual,
    Brief,
    Detailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationFrequency {
    Minimal,
    Normal,
    Frequent,
}

impl ConversationContext {
    pub fn new(member_id: Option<Uuid>) -> Self {
        ConversationContext {
            id: Uuid::new_v4(),
            member_id,
            messages: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            context_data: ContextData::default(),
        }
    }
    
    pub fn add_message(&mut self, role: MessageRole, content: String) {
        let message = Message {
            id: Uuid::new_v4(),
            role,
            content,
            timestamp: Utc::now(),
        };
        self.messages.push(message);
        self.updated_at = Utc::now();
    }
    
    pub fn get_recent_messages(&self, count: usize) -> Vec<&Message> {
        self.messages.iter().rev().take(count).collect()
    }
    
    pub fn clear_old_messages(&mut self, keep_last: usize) {
        if self.messages.len() > keep_last {
            let start_index = self.messages.len() - keep_last;
            self.messages.drain(0..start_index);
            self.updated_at = Utc::now();
        }
    }
}

impl Default for ContextData {
    fn default() -> Self {
        ContextData {
            team_name: "Default Team".to_string(),
            current_project: None,
            recent_activities: Vec::new(),
            preferences: UserPreferences::default(),
        }
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        UserPreferences {
            communication_style: CommunicationStyle::Casual,
            notification_frequency: NotificationFrequency::Normal,
            preferred_channels: vec!["slack".to_string()],
        }
    }
}