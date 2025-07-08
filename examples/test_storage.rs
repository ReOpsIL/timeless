// Example demonstrating the storage system
use timeless::models::*;
use timeless::storage::TeamRepository;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Testing Timeless Storage System");
    
    // Initialize repository
    let repo = TeamRepository::new("./data")?;
    
    // Create a team member
    let member = TeamMember::new(
        "John Doe".to_string(),
        "john@example.com".to_string(),
        "Software Engineer".to_string(),
    ).with_slack_id("@john.doe".to_string());
    
    println!("Created team member: {} ({})", member.name, member.id);
    
    // Save the team member
    repo.save_team_member(&member)?;
    println!("✓ Team member saved to storage");
    
    // Retrieve the team member
    let retrieved_member = repo.get_team_member(member.id)?;
    match retrieved_member {
        Some(m) => println!("✓ Retrieved team member: {} ({})", m.name, m.email),
        None => println!("✗ Failed to retrieve team member"),
    }
    
    // Create a project
    let mut project = Project::new(
        "Team Management Tool".to_string(),
        "AI-powered team management platform".to_string(),
    );
    
    println!("Created project: {} ({})", project.name, project.id);
    repo.save_project(&project)?;
    println!("✓ Project saved to storage");
    
    // Create a status update
    let mut status = StatusUpdate::new(
        member.id,
        "Working on the core data models and storage system".to_string(),
    ).with_mood("productive".to_string());
    
    status.add_achievement("Implemented JSON storage system".to_string());
    status.add_achievement("Created data models with helper methods".to_string());
    
    println!("Created status update: {}", status.content);
    repo.save_status_update(&status)?;
    println!("✓ Status update saved to storage");
    
    // Create team metrics
    let mut metrics = TeamMetrics::new(chrono::Utc::now());
    metrics.active_members = 1;
    metrics.completed_tasks = 2;
    metrics.blockers_count = 0;
    metrics.average_satisfaction = 8.5;
    metrics.velocity = 75.0;
    
    println!("Created team metrics with health score: {:.1}", metrics.calculate_health_score());
    repo.save_team_metrics(&metrics)?;
    println!("✓ Team metrics saved to storage");
    
    // List all team members
    let all_members = repo.list_team_members()?;
    println!("Total team members in storage: {}", all_members.len());
    
    // Get recent status updates
    let recent_updates = repo.get_recent_status_updates(10)?;
    println!("Recent status updates: {}", recent_updates.len());
    
    // Get latest metrics
    if let Some(latest_metrics) = repo.get_latest_team_metrics()? {
        println!("Latest team health score: {:.1}", latest_metrics.calculate_health_score());
        if latest_metrics.has_concerning_metrics() {
            println!("⚠️  Team metrics show concerning trends");
        } else {
            println!("✓ Team metrics look healthy");
        }
    }
    
    println!("\n🎉 Storage system test completed successfully!");
    Ok(())
}