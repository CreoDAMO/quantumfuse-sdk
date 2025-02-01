use axum::{routing::get, Router, Json};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::json;
use quantumfuse_sdk::{
    finance::{QuantumTreasury, StakingPool},
    ai::PolicyAI,
};

#[tokio::main]
async fn main() {
    let defi_state = Arc::new(RwLock::new(DeFiOptimizer::new()));

    let app = Router::new()
        .route("/defi/yield_forecast", get(get_yield_forecast))
        .route("/defi/ai_optimize", get(ai_optimize_yield))
        .layer(axum::AddExtensionLayer::new(defi_state));

    println!("💰 Multi-Chain DeFi Optimization API running at http://127.0.0.1:8085/");
    axum::Server::bind(&"127.0.0.1:8085".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 🔮 **AI-Powered Multi-Chain Staking Forecast**
async fn get_yield_forecast(
    state: axum::Extension<Arc<RwLock<DeFiOptimizer>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "staking_yield_ethereum": state.predict_yield("Ethereum", 30),
        "staking_yield_polkadot": state.predict_yield("Polkadot", 30),
        "staking_yield_cosmos": state.predict_yield("Cosmos", 30),
    }))
}

// 🔹 **Optimize Multi-Chain Liquidity**
async fn ai_optimize_yield(
    state: axum::Extension<Arc<RwLock<DeFiOptimizer>>>,
) -> Json<serde_json::Value> {
    let mut state = state.write().await;
    state.optimize_liquidity_distribution();
    Json(json!({ "status": "Liquidity optimized successfully" }))
}
