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
    let treasury_state = Arc::new(RwLock::new(TreasuryAI::new()));

    let app = Router::new()
        .route("/treasury/forecast", get(get_treasury_forecast))
        .route("/treasury/ai_optimize", get(ai_optimize_treasury))
        .layer(axum::AddExtensionLayer::new(treasury_state));

    println!("💰 Quantum Treasury API running at http://127.0.0.1:8083/");
    axum::Server::bind(&"127.0.0.1:8083".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 🔮 **AI-Powered Staking Forecasting**
async fn get_treasury_forecast(
    state: axum::Extension<Arc<RwLock<TreasuryAI>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "staking_yield_30_days": state.predict_staking_yields(30),
        "staking_yield_90_days": state.predict_staking_yields(90),
    }))
}

// 🔹 **Optimize Treasury Allocation**
async fn ai_optimize_treasury(
    state: axum::Extension<Arc<RwLock<TreasuryAI>>>,
) -> Json<serde_json::Value> {
    let mut state = state.write().await;
    state.optimize_treasury_reserves();
    Json(json!({ "status": "Treasury optimized successfully" }))
}
