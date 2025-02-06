use axum::{routing::get, Router, Json, extract::State};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, task, time::sleep};
use quantumfuse_sdk::{
    ai::{PolicyAI, DisputeResolver, JudicialAI},
    finance::DecentralizedGovernanceBonds,
    did::ReputationSystem,
    consensus::QuantumConsensus,
    metrics::{GovernanceMetrics, TreasuryMetrics, JudiciaryMetrics, ReputationMetrics},
    blockchain::BlockchainClient, // Assuming a client to interact with QuantumFuse blockchain
};
use dotenv::dotenv;
use std::env;

/// 📊 **Dashboard State Struct**
#[derive(Debug, Serialize, Deserialize)]
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load environment variables
    env_logger::init();

    let port = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let address = format!("127.0.0.1:{}", port);

    let dashboard_state = Arc::new(RwLock::new(DashboardState::new()));

    // ✅ Start periodic blockchain data updates
    let state_clone = dashboard_state.clone();
    task::spawn(async move {
        loop {
            if let Err(e) = update_dashboard_state(state_clone.clone()).await {
                eprintln!("❌ Error updating dashboard state: {}", e);
            }
            sleep(Duration::from_secs(60)).await; // Update every 60 seconds
        }
    });

    // ✅ Define API routes
    let app = Router::new()
        .route("/metrics/governance", get(get_governance_metrics))
        .route("/metrics/treasury", get(get_treasury_metrics))
        .route("/metrics/judiciary", get(get_judiciary_metrics))
        .route("/metrics/reputation", get(get_reputation_scores))
        .with_state(dashboard_state);

    println!("📊 On-Chain Analytics API running at http://{}/", address);
    axum::Server::bind(&address.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// 🔄 **Periodic Blockchain Data Fetch**
async fn update_dashboard_state(state: Arc<RwLock<DashboardState>>) -> Result<(), Box<dyn std::error::Error>> {
    let client = BlockchainClient::connect("https://quantumfuse-node.com").await?; // Example node URL

    // ✅ Fetch live data from the QuantumFuse blockchain
    let governance_data = client.get_governance_metrics().await?;
    let treasury_data = client.get_treasury_metrics().await?;
    let judiciary_data = client.get_judiciary_metrics().await?;
    let reputation_data = client.get_reputation_scores().await?;

    // ✅ Write data to shared state
    let mut state = state.write().await;
    state.governance = governance_data;
    state.treasury = treasury_data;
    state.judiciary = judiciary_data;
    state.reputation = reputation_data;

    println!("✅ Dashboard state updated from blockchain.");
    Ok(())
}

/// 🚀 **Governance Analytics**
async fn get_governance_metrics(State(state): State<Arc<RwLock<DashboardState>>>) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "active_proposals": state.governance.active_proposals,
        "votes_cast": state.governance.votes_cast,
        "reputation_weighted_votes": state.governance.reputation_weighted_votes,
    }))
}

/// 💰 **Treasury Analytics**
async fn get_treasury_metrics(State(state): State<Arc<RwLock<DashboardState>>>) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "treasury_balance": state.treasury.treasury_balance,
        "bond_issuance": state.treasury.bond_issuance,
        "staking_rewards": state.treasury.staking_rewards,
    }))
}

/// ⚖️ **Judiciary Analytics**
async fn get_judiciary_metrics(State(state): State<Arc<RwLock<DashboardState>>>) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "active_cases": state.judiciary.active_cases,
        "resolved_cases": state.judiciary.resolved_cases,
        "avg_resolution_time": state.judiciary.avg_resolution_time,
    }))
}

/// 🏆 **Reputation System Analytics**
async fn get_reputation_scores(State(state): State<Arc<RwLock<DashboardState>>>) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "top_reputation_users": state.reputation.top_users,
        "avg_reputation_score": state.reputation.avg_reputation,
    }))
}