use rust_bert::pipelines::sentiment::{SentimentModel, SentimentPolarity};
use ark_groth16::{Proof, VerifyingKey, verify_proof};  // ✅ Fixed import
use ark_serialize::CanonicalDeserialize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::Utc;

/// 🔹 AI-Governed Blockchain Voting System
#[derive(Debug)]
pub struct QuantumGovernance {
    proposals: HashMap<String, GovernanceProposal>,
    ai_policy_engine: AIEngine,
}

/// 🔹 Governance Proposal Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub id: String,
    pub proposer: String,
    pub description: String,
    pub ai_risk_score: f64,
    pub ai_sentiment: SentimentPolarity,
    pub voting_end_time: chrono::DateTime<Utc>,
}

/// 🔹 AI Engine for Proposal Analysis
pub struct AIEngine {
    sentiment_model: SentimentModel,
}

impl AIEngine {
    pub fn new() -> Self {
        Self {
            sentiment_model: SentimentModel::new(Default::default()).unwrap(),
        }
    }

    pub fn analyze_proposal(&self, proposal: &str) -> (SentimentPolarity, f64) {
        let sentiment = self.sentiment_model.predict(&[proposal]);
        let risk_score = match sentiment[0] {
            SentimentPolarity::Positive => 0.1,
            SentimentPolarity::Neutral => 0.5,
            SentimentPolarity::Negative => 0.9,
        };
        (sentiment[0], risk_score)
    }
}

impl QuantumGovernance {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            ai_policy_engine: AIEngine::new(),
        }
    }

    pub fn submit_proposal(&mut self, proposer: &str, description: &str) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let (sentiment, risk_score) = self.ai_policy_engine.analyze_proposal(description);
        let proposal = GovernanceProposal {
            id: id.clone(),
            proposer: proposer.to_string(),
            description: description.to_string(),
            ai_risk_score: risk_score,
            ai_sentiment: sentiment,
            voting_end_time: Utc::now() + chrono::Duration::days(7),
        };
        self.proposals.insert(id.clone(), proposal);
        id
    }
    }
