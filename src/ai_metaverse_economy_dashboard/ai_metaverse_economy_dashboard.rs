use axum::{routing::get, Router, Json};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::json;
use quantumfuse_sdk::{finance::QuantumTreasury, ai::PolicyAI};

#[tokio::main]
async fn main() {
    let metaverse_state = Arc::new(RwLock::new(MetaverseAnalytics::new()));

    let app = Router::new()
        .route("/metrics/metaverse/economy", get(get_economy_stats))
        .route("/metrics/metaverse/traders", get(get_npc_trader_stats))
        .layer(axum::AddExtensionLayer::new(metaverse_state));

    println!("🌍 Metaverse Economy API running at http://127.0.0.1:8082/");
    axum::Server::bind(&"127.0.0.1:8082".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_economy_stats(
    state: axum::Extension<Arc<RwLock<MetaverseAnalytics>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "inflation_rate": state.economy.inflation_rate,
        "nft_market_cap": state.nft_market.market_cap,
        "dao_activity": state.dao_gov.total_active_proposals,
    }))
}
