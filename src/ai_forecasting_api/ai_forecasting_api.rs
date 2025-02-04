use axum::{routing::get, Router, Json};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::json;
use quantumfuse_sdk::{
    ai::{PolicyAI, JudicialAI},
    finance::QuantumTreasury,
};

#[tokio::main]
async fn main() {
    let ai_forecast_state = Arc::new(RwLock::new(AIForecasting::new()));

    let app = Router::new()
        .route("/metrics/ai/staking_forecast", get(get_staking_forecast))
        .route("/metrics/ai/legal_predictions", get(get_legal_forecasts))
        .layer(axum::AddExtensionLayer::new(ai_forecast_state));

    println!("🤖 AI Forecasting API running at http://127.0.0.1:8084/");
    axum::Server::bind(&"127.0.0.1:8084".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 🔮 **Staking Yield Predictions**
async fn get_staking_forecast(
    state: axum::Extension<Arc<RwLock<AIForecasting>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "next_6_months_yield_forecast": state.policy_ai.predict_staking_yields(180),
    }))
}

// ⚖️ **AI-Based Legal Outcome Predictions**
async fn get_legal_forecasts(
    state: axum::Extension<Arc<RwLock<AIForecasting>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "ai_predicted_verdicts": state.judicial_ai.forecast_outcomes(),
    }))
}
