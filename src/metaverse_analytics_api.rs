use axum::{routing::get, Router, Json};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::json;
use quantumfuse_sdk::{
    metaverse::{MetaverseRegistry, NFTMarketplace, VirtualLandRegistry},
    governance::QuantumGovernance,
    consensus::QuantumConsensus,
};

#[tokio::main]
async fn main() {
    let metaverse_state = Arc::new(RwLock::new(MetaverseAnalytics::new()));

    let app = Router::new()
        .route("/metrics/metaverse/nft_sales", get(get_nft_sales))
        .route("/metrics/metaverse/virtual_land", get(get_virtual_land_sales))
        .route("/metrics/metaverse/dao_voting", get(get_dao_voting))
        .layer(axum::AddExtensionLayer::new(metaverse_state));

    println!("🌍 Metaverse Analytics API running at http://127.0.0.1:8082/");
    axum::Server::bind(&"127.0.0.1:8082".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 🖼️ **NFT Marketplace Sales**
async fn get_nft_sales(
    state: axum::Extension<Arc<RwLock<MetaverseAnalytics>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "total_nft_sales": state.nft_marketplace.total_sales,
        "average_nft_price": state.nft_marketplace.avg_price,
        "top_nft_trends": state.nft_marketplace.trending_assets,
    }))
}

// 🏠 **Virtual Land Transactions**
async fn get_virtual_land_sales(
    state: axum::Extension<Arc<RwLock<MetaverseAnalytics>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "total_land_sales": state.virtual_land.total_sales,
        "land_price_index": state.virtual_land.price_index,
        "top_selling_regions": state.virtual_land.hotspots,
    }))
}

// 🗳️ **DAO Voting in the Metaverse**
async fn get_dao_voting(
    state: axum::Extension<Arc<RwLock<MetaverseAnalytics>>>,
) -> Json<serde_json::Value> {
    let state = state.read().await;
    Json(json!({
        "active_dao_proposals": state.dao_voting.active_proposals,
        "voter_participation_rate": state.dao_voting.participation_rate,
        "cross_chain_voting": state.dao_voting.cross_chain_activity,
    }))
}
