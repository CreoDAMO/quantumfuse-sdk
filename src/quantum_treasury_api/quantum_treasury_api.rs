use axum::{routing::get, Router, Json};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::json;
use quantumfuse_sdk::{
    finance::{QuantumTreasury, GovernanceBond},
    ai::PolicyAI,
};

#[tokio::main]
async fn main() {
    let treasury_state = Arc::new(RwLock::new(TreasuryAnalytics::new()));

    let app = Router::new()
        .route("/treasury/reserves", get(get_treasury_reserves))
        .route("/treasury/bonds", get(get_bond_metrics))
        .route("/treasury/ai_optimize", get(ai_optimize_treasury))
        .layer(axum::AddExtensionLayer::new(treasury_state));

    println!("💰 Quantum Treasury API running at http://127.0.0.1:8083/");
    axum::Server::bind(&"127.0.0.1:8083".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 💳 **Get Treasury Reserves**
async fn get_treasury_reserves(
    state: axum::Extension<Arc<RwLock<TreasuryAnalytics>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "total_reserves": state.treasury.total_reserves,
        "cross_chain_holdings": state.treasury.cross_chain_holdings,
        "liquidity_health": state.treasury.liquidity_ratio,
    }))
}

// 📈 **Get Bond Issuance & Returns**
async fn get_bond_metrics(
    state: axum::Extension<Arc<RwLock<TreasuryAnalytics>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "total_bonds_issued": state.bonds.total_issued,
        "average_interest_rate": state.bonds.avg_interest_rate,
        "institutional_investors": state.bonds.institutional_investor_count,
    }))
}

// 🔮 **AI-Powered Treasury Optimization**
async fn ai_optimize_treasury(
    state: axum::Extension<Arc<RwLock<TreasuryAnalytics>>>,
) -> Json<serde_json::Value> {
    let mut state = state.write().await;
    let optimized_allocation = state.policy_ai.optimize_treasury_allocation(state.treasury.total_reserves);
    state.treasury.total_reserves = optimized_allocation;
    Json(json!({ "new_reserve_allocation": optimized_allocation }))
}
