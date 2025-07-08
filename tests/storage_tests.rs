use timeless::models::*;
use timeless::storage::TeamRepository;
use tempfile::TempDir;
use uuid::Uuid;

#[tokio::test]
async fn test_team_member_crud() {
    let temp_dir = TempDir::new().unwrap();
    let repo = TeamRepository::new(temp_dir.path().to_str().unwrap()).unwrap();
    
    // Create a team member
    let member = TeamMember::new(
        "John Doe".to_string(),
        "john@example.com".to_string(),
        "Developer".to_string(),
    );
    let member_id = member.id;
    
    // Save team member
    repo.save_team_member(&member).unwrap();
    
    // Retrieve team member
    let retrieved = repo.get_team_member(member_id).unwrap();
    assert!(retrieved.is_some());
    let retrieved_member = retrieved.unwrap();
    assert_eq!(retrieved_member.name, "John Doe");
    assert_eq!(retrieved_member.email, "john@example.com");
    assert_eq!(retrieved_member.role, "Developer");
    
    // List team members
    let members = repo.list_team_members().unwrap();
    assert_eq!(members.len(), 1);
    
    // Remove team member
    let removed = repo.remove_team_member(member_id).unwrap();
    assert!(removed.is_some());
    
    // Verify removal
    let members_after_removal = repo.list_team_members().unwrap();
    assert_eq!(members_after_removal.len(), 0);
}

#[tokio::test]
async fn test_project_operations() {
    let temp_dir = TempDir::new().unwrap();
    let repo = TeamRepository::new(temp_dir.path().to_str().unwrap()).unwrap();
    
    // Create a project
    let mut project = Project::new(
        "Test Project".to_string(),
        "A test project".to_string(),
    );
    let project_id = project.id;
    
    // Save project
    repo.save_project(&project).unwrap();
    
    // Retrieve project
    let retrieved = repo.get_project(project_id).unwrap();
    assert!(retrieved.is_some());
    let retrieved_project = retrieved.unwrap();
    assert_eq!(retrieved_project.name, "Test Project");
    assert!(retrieved_project.is_active());
    
    // Update project status
    project.set_status(ProjectStatus::Completed);
    repo.save_project(&project).unwrap();
    
    // Verify status update
    let updated_project = repo.get_project(project_id).unwrap().unwrap();
    assert!(!updated_project.is_active());
    assert!(matches!(updated_project.status, ProjectStatus::Completed));
}

#[tokio::test]
async fn test_status_updates() {
    let temp_dir = TempDir::new().unwrap();
    let repo = TeamRepository::new(temp_dir.path().to_str().unwrap()).unwrap();
    
    let member_id = Uuid::new_v4();
    
    // Create multiple status updates
    let mut status1 = StatusUpdate::new(
        member_id,
        "Working on feature A".to_string(),
    );
    status1.add_achievement("Completed design".to_string());
    status1.add_blocker("Waiting for API".to_string());
    
    let status2 = StatusUpdate::new(
        member_id,
        "Working on feature B".to_string(),
    );
    
    // Save status updates
    repo.save_status_update(&status1).unwrap();
    repo.save_status_update(&status2).unwrap();
    
    // Get status updates for member
    let member_updates = repo.get_status_updates_for_member(member_id).unwrap();
    assert_eq!(member_updates.len(), 2);
    
    // Get recent status updates
    let recent_updates = repo.get_recent_status_updates(10).unwrap();
    assert_eq!(recent_updates.len(), 2);
    
    // Verify blocker detection
    assert!(status1.has_blockers());
    assert!(!status2.has_blockers());
}

#[tokio::test]
async fn test_team_metrics() {
    let temp_dir = TempDir::new().unwrap();
    let repo = TeamRepository::new(temp_dir.path().to_str().unwrap()).unwrap();
    
    // Create team metrics
    let mut metrics = TeamMetrics::new(chrono::Utc::now());
    metrics.active_members = 5;
    metrics.completed_tasks = 20;
    metrics.blockers_count = 1;
    metrics.average_satisfaction = 8.0;
    metrics.velocity = 80.0;
    
    // Save metrics
    repo.save_team_metrics(&metrics).unwrap();
    
    // Retrieve latest metrics
    let latest = repo.get_latest_team_metrics().unwrap();
    assert!(latest.is_some());
    let latest_metrics = latest.unwrap();
    
    // Test health score calculation
    let health_score = latest_metrics.calculate_health_score();
    assert!(health_score > 7.0); // Should be healthy
    assert!(!latest_metrics.has_concerning_metrics());
    
    // Test concerning metrics
    let mut concerning_metrics = TeamMetrics::new(chrono::Utc::now());
    concerning_metrics.average_satisfaction = 3.0; // Low satisfaction
    concerning_metrics.velocity = 5.0; // Low velocity
    
    assert!(concerning_metrics.has_concerning_metrics());
}

#[tokio::test]
async fn test_ai_decisions() {
    let temp_dir = TempDir::new().unwrap();
    let repo = TeamRepository::new(temp_dir.path().to_str().unwrap()).unwrap();
    
    // Create AI decision
    let mut decision = AIDecision::new(
        "task_prioritization".to_string(),
        "Team has multiple urgent tasks".to_string(),
        "Focus on critical bug fixes first".to_string(),
        0.85,
    );
    
    assert!(decision.is_high_confidence());
    
    // Save decision
    repo.save_ai_decision(&decision).unwrap();
    
    // Add outcome
    decision.set_outcome("Recommendation followed, bugs fixed".to_string());
    repo.save_ai_decision(&decision).unwrap();
    
    // Get recent decisions
    let recent_decisions = repo.get_recent_ai_decisions(5).unwrap();
    assert_eq!(recent_decisions.len(), 1);
    assert!(recent_decisions[0].outcome.is_some());
}