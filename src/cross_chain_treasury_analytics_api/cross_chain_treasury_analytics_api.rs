use axum::{routing::get, Router, Json};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::json;
use quantumfuse_sdk::{
    finance::{QuantumTreasury, DecentralizedGovernanceBonds},
    ai::PolicyAI,
};

#[tokio::main]
async fn main() {
    let treasury_state = Arc::new(RwLock::new(TreasuryAnalytics::new()));

    let app = Router::new()
        .route("/metrics/treasury/reserves", get(get_treasury_reserves))
        .route("/metrics/treasury/bonds", get(get_bond_metrics))
        .route("/metrics/treasury/staking", get(get_staking_yields))
        .layer(axum::AddExtensionLayer::new(treasury_state));

    println!("💰 Treasury Analytics API running at http://127.0.0.1:8083/");
    axum::Server::bind(&"127.0.0.1:8083".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 💳 **Quantum Treasury Reserves**
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

// 📈 **Bond Issuance & Returns**
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

// 🔮 **AI-Powered Staking Yield Predictions**
async fn get_staking_yields(
    state: axum::Extension<Arc<RwLock<TreasuryAnalytics>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "current_staking_rate": state.staking.current_rate,
        "ai_predicted_staking_rate": state.staking.ai_forecasted_rate,
        "validator_performance": state.staking.validator_health,
    }))
}

// Make get_cross_chain_analytics public
pub fn get_cross_chain_analytics() -> Result<(), Box<dyn std::error::Error>> {
    println!("Retrieving cross-chain treasury analytics...");
    let data = json!({
        "total_cross_chain_transfers": 1000,
        "total_value_transferred": 10_000_000.0,
        "interoperability_score": 85
    });
    println!("Cross-chain analytics data: {}", data);
    Ok(())
}
