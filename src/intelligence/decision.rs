use crate::models::AIDecision;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionEngine {
    pub decisions: Vec<AIDecision>,
    pub learning_data: LearningData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningData {
    pub feedback_scores: Vec<FeedbackScore>,
    pub outcome_tracking: Vec<OutcomeRecord>,
    pub pattern_recognition: PatternData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackScore {
    pub decision_id: Uuid,
    pub score: f32,
    pub feedback_text: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomeRecord {
    pub decision_id: Uuid,
    pub actual_outcome: String,
    pub success_metric: f32,
    pub lessons_learned: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternData {
    pub successful_patterns: Vec<Pattern>,
    pub failed_patterns: Vec<Pattern>,
    pub context_correlations: Vec<ContextCorrelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub pattern_type: String,
    pub conditions: Vec<String>,
    pub success_rate: f32,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCorrelation {
    pub context_factor: String,
    pub correlation_strength: f32,
    pub impact_direction: ImpactDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactDirection {
    Positive,
    Negative,
    Neutral,
}

impl DecisionEngine {
    pub fn new() -> Self {
        DecisionEngine {
            decisions: Vec::new(),
            learning_data: LearningData::default(),
        }
    }
    
    pub async fn make_decision(&mut self, context: &str, decision_type: &str) -> Result<AIDecision> {
        // TODO: Implement actual decision making with Claude
        let decision = AIDecision {
            id: Uuid::new_v4(),
            decision_type: decision_type.to_string(),
            context: context.to_string(),
            recommendation: "Placeholder recommendation".to_string(),
            confidence: 0.7,
            created_at: chrono::Utc::now(),
            outcome: None,
        };
        
        self.decisions.push(decision.clone());
        Ok(decision)
    }
    
    pub async fn record_feedback(&mut self, decision_id: Uuid, score: f32, feedback: Option<String>) -> Result<()> {
        let feedback_score = FeedbackScore {
            decision_id,
            score,
            feedback_text: feedback,
            timestamp: chrono::Utc::now(),
        };
        
        self.learning_data.feedback_scores.push(feedback_score);
        Ok(())
    }
    
    pub async fn record_outcome(&mut self, decision_id: Uuid, outcome: String, success_metric: f32) -> Result<()> {
        // Update the decision with the outcome
        if let Some(decision) = self.decisions.iter_mut().find(|d| d.id == decision_id) {
            decision.outcome = Some(outcome.clone());
        }
        
        // Record the outcome for learning
        let outcome_record = OutcomeRecord {
            decision_id,
            actual_outcome: outcome,
            success_metric,
            lessons_learned: vec![],
        };
        
        self.learning_data.outcome_tracking.push(outcome_record);
        Ok(())
    }
    
    pub fn get_decision_confidence(&self, decision_type: &str) -> f32 {
        let relevant_decisions: Vec<_> = self.decisions
            .iter()
            .filter(|d| d.decision_type == decision_type)
            .collect();
        
        if relevant_decisions.is_empty() {
            return 0.5; // Default confidence
        }
        
        let avg_confidence: f32 = relevant_decisions
            .iter()
            .map(|d| d.confidence)
            .sum::<f32>() / relevant_decisions.len() as f32;
        
        avg_confidence
    }
    
    pub fn get_success_rate(&self, decision_type: &str) -> f32 {
        let feedback_scores: Vec<_> = self.decisions
            .iter()
            .filter(|d| d.decision_type == decision_type)
            .filter_map(|d| {
                self.learning_data.feedback_scores
                    .iter()
                    .find(|f| f.decision_id == d.id)
                    .map(|f| f.score)
            })
            .collect();
        
        if feedback_scores.is_empty() {
            return 0.5; // Default success rate
        }
        
        feedback_scores.iter().sum::<f32>() / feedback_scores.len() as f32
    }
}

impl Default for LearningData {
    fn default() -> Self {
        LearningData {
            feedback_scores: Vec::new(),
            outcome_tracking: Vec::new(),
            pattern_recognition: PatternData::default(),
        }
    }
}

impl Default for PatternData {
    fn default() -> Self {
        PatternData {
            successful_patterns: Vec::new(),
            failed_patterns: Vec::new(),
            context_correlations: Vec::new(),
        }
    }
}