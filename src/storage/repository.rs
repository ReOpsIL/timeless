use crate::models::*;
use crate::storage::{JsonStore, Storage};
use crate::storage::json_store::DataCollection;
use anyhow::Result;
use uuid::Uuid;

pub struct TeamRepository {
    store: JsonStore,
}

impl TeamRepository {
    pub fn new(data_dir: &str) -> Result<Self> {
        let store = JsonStore::new(data_dir)?;
        Ok(TeamRepository { store })
    }
    
    // Team member operations
    pub fn save_team_member(&self, member: &TeamMember) -> Result<()> {
        let mut members: DataCollection<TeamMember> = self.store
            .load("team_members")?
            .unwrap_or_default();
        
        members.insert(member.id.to_string(), member.clone());
        self.store.save("team_members", &members)?;
        Ok(())
    }
    
    pub fn get_team_member(&self, id: Uuid) -> Result<Option<TeamMember>> {
        let members: Option<DataCollection<TeamMember>> = self.store.load("team_members")?;
        Ok(members.and_then(|m| m.get(&id.to_string()).cloned()))
    }
    
    pub fn list_team_members(&self) -> Result<Vec<TeamMember>> {
        let members: Option<DataCollection<TeamMember>> = self.store.load("team_members")?;
        Ok(members.map(|m| m.items.into_values().collect()).unwrap_or_default())
    }
    
    pub fn remove_team_member(&self, id: Uuid) -> Result<Option<TeamMember>> {
        let mut members: DataCollection<TeamMember> = self.store
            .load("team_members")?
            .unwrap_or_default();
        
        let removed = members.remove(&id.to_string());
        self.store.save("team_members", &members)?;
        Ok(removed)
    }
    
    // Project operations
    pub fn save_project(&self, project: &Project) -> Result<()> {
        let mut projects: DataCollection<Project> = self.store
            .load("projects")?
            .unwrap_or_default();
        
        projects.insert(project.id.to_string(), project.clone());
        self.store.save("projects", &projects)?;
        Ok(())
    }
    
    pub fn get_project(&self, id: Uuid) -> Result<Option<Project>> {
        let projects: Option<DataCollection<Project>> = self.store.load("projects")?;
        Ok(projects.and_then(|p| p.get(&id.to_string()).cloned()))
    }
    
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        let projects: Option<DataCollection<Project>> = self.store.load("projects")?;
        Ok(projects.map(|p| p.items.into_values().collect()).unwrap_or_default())
    }
    
    // Status update operations
    pub fn save_status_update(&self, update: &StatusUpdate) -> Result<()> {
        let mut updates: DataCollection<StatusUpdate> = self.store
            .load("status_updates")?
            .unwrap_or_default();
        
        updates.insert(update.id.to_string(), update.clone());
        self.store.save("status_updates", &updates)?;
        Ok(())
    }
    
    pub fn get_status_updates_for_member(&self, member_id: Uuid) -> Result<Vec<StatusUpdate>> {
        let updates: Option<DataCollection<StatusUpdate>> = self.store.load("status_updates")?;
        Ok(updates
            .map(|u| u.items.into_values()
                .filter(|update| update.member_id == member_id)
                .collect())
            .unwrap_or_default())
    }
    
    pub fn get_recent_status_updates(&self, limit: usize) -> Result<Vec<StatusUpdate>> {
        let updates: Option<DataCollection<StatusUpdate>> = self.store.load("status_updates")?;
        let mut all_updates: Vec<StatusUpdate> = updates
            .map(|u| u.items.into_values().collect())
            .unwrap_or_default();
        
        // Sort by timestamp descending
        all_updates.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        all_updates.truncate(limit);
        Ok(all_updates)
    }
    
    // Conversation operations
    pub fn save_conversation(&self, conversation: &Conversation) -> Result<()> {
        let mut conversations: DataCollection<Conversation> = self.store
            .load("conversations")?
            .unwrap_or_default();
        
        conversations.insert(conversation.id.to_string(), conversation.clone());
        self.store.save("conversations", &conversations)?;
        Ok(())
    }
    
    pub fn get_conversation(&self, id: Uuid) -> Result<Option<Conversation>> {
        let conversations: Option<DataCollection<Conversation>> = self.store.load("conversations")?;
        Ok(conversations.and_then(|c| c.get(&id.to_string()).cloned()))
    }
    
    pub fn get_conversations_for_member(&self, member_id: Uuid) -> Result<Vec<Conversation>> {
        let conversations: Option<DataCollection<Conversation>> = self.store.load("conversations")?;
        Ok(conversations
            .map(|c| c.items.into_values()
                .filter(|conv| conv.member_id == member_id)
                .collect())
            .unwrap_or_default())
    }
    
    // AI Decision operations
    pub fn save_ai_decision(&self, decision: &AIDecision) -> Result<()> {
        let mut decisions: DataCollection<AIDecision> = self.store
            .load("ai_decisions")?
            .unwrap_or_default();
        
        decisions.insert(decision.id.to_string(), decision.clone());
        self.store.save("ai_decisions", &decisions)?;
        Ok(())
    }
    
    pub fn get_recent_ai_decisions(&self, limit: usize) -> Result<Vec<AIDecision>> {
        let decisions: Option<DataCollection<AIDecision>> = self.store.load("ai_decisions")?;
        let mut all_decisions: Vec<AIDecision> = decisions
            .map(|d| d.items.into_values().collect())
            .unwrap_or_default();
        
        // Sort by timestamp descending
        all_decisions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        all_decisions.truncate(limit);
        Ok(all_decisions)
    }
    
    // Team metrics operations
    pub fn save_team_metrics(&self, metrics: &TeamMetrics) -> Result<()> {
        let mut all_metrics: DataCollection<TeamMetrics> = self.store
            .load("team_metrics")?
            .unwrap_or_default();
        
        all_metrics.insert(metrics.id.to_string(), metrics.clone());
        self.store.save("team_metrics", &all_metrics)?;
        Ok(())
    }
    
    pub fn get_latest_team_metrics(&self) -> Result<Option<TeamMetrics>> {
        let metrics: Option<DataCollection<TeamMetrics>> = self.store.load("team_metrics")?;
        Ok(metrics.and_then(|m| {
            m.items.into_values()
                .max_by_key(|metrics| metrics.date)
        }))
    }
    
    pub fn get_team_metrics_range(&self, start_date: chrono::DateTime<chrono::Utc>, end_date: chrono::DateTime<chrono::Utc>) -> Result<Vec<TeamMetrics>> {
        let metrics: Option<DataCollection<TeamMetrics>> = self.store.load("team_metrics")?;
        Ok(metrics
            .map(|m| m.items.into_values()
                .filter(|metric| metric.date >= start_date && metric.date <= end_date)
                .collect())
            .unwrap_or_default())
    }
}