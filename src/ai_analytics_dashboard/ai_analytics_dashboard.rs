use axum::{routing::get, Router, Json};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::json;
use quantumfuse_sdk::{
    ai::{PolicyAI, DisputeResolver, JudicialAI},
    finance::DecentralizedGovernanceBonds,
    did::ReputationSystem,
    consensus::QuantumConsensus,
    metrics::GovernanceMetrics,
};

#[tokio::main]
async fn main() {
    let dashboard_state = Arc::new(RwLock::new(DashboardState::new()));

    let app = Router::new()
        .route("/metrics/governance", get(get_governance_metrics))
        .route("/metrics/treasury", get(get_treasury_metrics))
        .route("/metrics/judiciary", get(get_judiciary_metrics))
        .route("/metrics/reputation", get(get_reputation_scores))
        .layer(axum::AddExtensionLayer::new(dashboard_state));

    println!("📊 On-Chain Analytics API running at http://127.0.0.1:8081/");
    axum::Server::bind(&"127.0.0.1:8081".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 🚀 **Governance Analytics**
async fn get_governance_metrics(
    state: axum::Extension<Arc<RwLock<DashboardState>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "active_proposals": state.governance.active_proposals,
        "votes_cast": state.governance.votes_cast,
        "reputation_weighted_votes": state.governance.reputation_weighted_votes,
    }))
}

// 💰 **Treasury Analytics**
async fn get_treasury_metrics(
    state: axum::Extension<Arc<RwLock<DashboardState>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "treasury_balance": state.treasury.treasury_balance,
        "bond_issuance": state.treasury.bond_issuance,
        "staking_rewards": state.treasury.staking_rewards,
    }))
}

// ⚖️ **Judiciary Analytics**
async fn get_judiciary_metrics(
    state: axum::Extension<Arc<RwLock<DashboardState>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "active_cases": state.judiciary.active_cases,
        "resolved_cases": state.judiciary.resolved_cases,
        "avg_resolution_time": state.judiciary.avg_resolution_time,
    }))
}

// 🏆 **Reputation System Analytics**
async fn get_reputation_scores(
    state: axum::Extension<Arc<RwLock<DashboardState>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "top_reputation_users": state.reputation.top_users,
        "avg_reputation_score": state.reputation.avg_reputation,
    }))
}

// 🏗️ **Dashboard State Struct**
#[derive(Debug)]
struct DashboardState {
    governance: GovernanceMetrics,
    treasury: TreasuryMetrics,
    judiciary: JudiciaryMetrics,
    reputation: ReputationMetrics,
}

impl DashboardState {
    fn new() -> Self {
        Self {
            governance: GovernanceMetrics::default(),
            treasury: TreasuryMetrics::default(),
            judiciary: JudiciaryMetrics::default(),
            reputation: ReputationMetrics::default(),
        }
    }
}
