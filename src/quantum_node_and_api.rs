use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use log::{info, error, warn};
use chrono::{DateTime, Utc};
use quantumfuse_sdk::{
    error::NodeError,
    crypto::{Hash, KeyPair},
    metrics::NodeMetrics,
    state::StateAccess,
    consensus::QuantumFuseConsensus,
    qkd::QKDManager,
    did::DIDRegistry
};

// Node Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub node_id: String,
    pub api_port: u16,
    pub p2p_port: u16,
    pub bootstrap_nodes: Vec<String>,
    pub quantum_backend: String,
    pub pqc_backend: String,
    pub storage_path: String,
    pub log_level: String,
    pub metrics_enabled: bool,
}

// API Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRequest {
    pub miner_wallet: String,
    pub transactions: Vec<Transaction>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockResponse {
    pub block: Option<Block>,
    pub status: ResponseStatus,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: Hash,
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub timestamp: DateTime<Utc>,
    pub signature: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub quantum_signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success,
    Error(String),
}

// Quantum Node Implementation
pub struct QuantumNode {
    config: NodeConfig,
    consensus: Arc<RwLock<QuantumFuseConsensus>>,
    peer_manager: Arc<RwLock<PeerManager>>,
    transaction_pool: Arc<RwLock<TransactionPool>>,
    storage: Arc<RwLock<QuantumStorage>>,
    metrics: Arc<RwLock<NodeMetrics>>,
}

impl QuantumNode {
    pub async fn new(config: NodeConfig) -> Result<Self, NodeError> {
        // Initialize components
        let consensus = Arc::new(RwLock::new(Self::initialize_consensus(&config).await?));
        let peer_manager = Arc::new(RwLock::new(PeerManager::new(&config)?));
        let transaction_pool = Arc::new(RwLock::new(TransactionPool::new(&config)?));
        let storage = Arc::new(RwLock::new(QuantumStorage::new(&config)?));
        let metrics = Arc::new(RwLock::new(NodeMetrics::default()));

        Ok(Self {
            config,
            consensus,
            peer_manager,
            transaction_pool,
            storage,
            metrics,
        })
    }

    async fn initialize_consensus(config: &NodeConfig) -> Result<QuantumFuseConsensus, NodeError> {
        let qkd_manager = Arc::new(QKDManager::new().await?);
        let did_registry = Arc::new(DIDRegistry::new().await?);

        QuantumFuseConsensus::new(
            ConsensusConfig::default(),
            qkd_manager,
            did_registry,
        ).await.map_err(NodeError::ConsensusError)
    }

    pub async fn start(&self) -> Result<(), NodeError> {
        info!("Starting Quantum Node with ID: {}", self.config.node_id);

        // Start P2P networking
        self.peer_manager.write().await.start().await?;

        // Start consensus
        self.consensus.write().await.start().await?;

        // Start API server
        self.start_api_server().await?;

        info!("Quantum Node started successfully");
        Ok(())
    }

    async fn start_api_server(&self) -> Result<(), NodeError> {
        let consensus = self.consensus.clone();
        let transaction_pool = self.transaction_pool.clone();
        let metrics = self.metrics.clone();

        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(consensus.clone()))
                .app_data(web::Data::new(transaction_pool.clone()))
                .app_data(web::Data::new(metrics.clone()))
                .service(
                    web::scope("/api/v1")
                        .route("/block/mine", web::post().to(handle_mine_block))
                        .route("/block/validate", web::post().to(handle_validate_block))
                        .route("/transaction/submit", web::post().to(handle_submit_transaction))
                        .route("/node/status", web::get().to(handle_node_status))
                        .route("/metrics", web::get().to(handle_metrics))
                )
        })
        .bind(format!("0.0.0.0:{}", self.config.api_port))?
        .run();

        tokio::spawn(server);
        Ok(())
    }
}

// API Handlers
async fn handle_mine_block(
    req: web::Json<BlockRequest>,
    consensus: web::Data<Arc<RwLock<QuantumFuseConsensus>>>,
    transaction_pool: web::Data<Arc<RwLock<TransactionPool>>>,
) -> impl Responder {
    let consensus = consensus.read().await;
    let mut tx_pool = transaction_pool.write().await;

    match consensus.mine_block(&req.miner_wallet, req.transactions.clone()).await {
        Ok(block) => {
            // Remove mined transactions from pool
            tx_pool.remove_transactions(&block.transactions).await?;
            
            HttpResponse::Ok().json(BlockResponse {
                block: Some(block),
                status: ResponseStatus::Success,
                message: "Block mined successfully".to_string(),
            })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(BlockResponse {
                block: None,
                status: ResponseStatus::Error(e.to_string()),
                message: "Failed to mine block".to_string(),
            })
        }
    }
}

async fn handle_validate_block(
    block: web::Json<Block>,
    consensus: web::Data<Arc<RwLock<QuantumFuseConsensus>>>,
) -> impl Responder {
    let consensus = consensus.read().await;
    
    match consensus.validate_block(&block).await {
        Ok(is_valid) => HttpResponse::Ok().json(json!({
            "valid": is_valid,
            "status": "success"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "valid": false,
            "status": "error",
            "message": e.to_string()
        }))
    }
}

async fn handle_submit_transaction(
    transaction: web::Json<Transaction>,
    transaction_pool: web::Data<Arc<RwLock<TransactionPool>>>,
) -> impl Responder {
    let mut tx_pool = transaction_pool.write().await;
    
    match tx_pool.add_transaction(transaction.0).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Transaction submitted successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": e.to_string()
        }))
    }
}

async fn handle_node_status(
    metrics: web::Data<Arc<RwLock<NodeMetrics>>>,
) -> impl Responder {
    let metrics = metrics.read().await;
    HttpResponse::Ok().json(json!({
        "status": "active",
        "uptime": metrics.uptime,
        "peers": metrics.peer_count,
        "transactions": metrics.transaction_count,
        "blocks": metrics.block_count
    }))
}

async fn handle_metrics(
    metrics: web::Data<Arc<RwLock<NodeMetrics>>>,
) -> impl Responder {
    let metrics = metrics.read().await;
    HttpResponse::Ok().json(metrics.clone())
}

#[tokio::main]
async fn main() -> Result<(), NodeError> {
    // Load configuration
    let config = NodeConfig::load("config.yaml")?;

    // Initialize and start node
    let node = QuantumNode::new(config).await?;
    node.start().await?;

    // Keep the main thread alive
    tokio::signal::ctrl_c().await?;
    info!("Shutting down Quantum Node");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_initialization() {
        let config = NodeConfig {
            node_id: "test-node".to_string(),
            api_port: 8080,
            p2p_port: 9000,
            bootstrap_nodes: vec![],
            quantum_backend: "simulator".to_string(),
            pqc_backend: "dilithium".to_string(),
            storage_path: "./test-data".to_string(),
            log_level: "debug".to_string(),
            metrics_enabled: true,
        };

        let node = QuantumNode::new(config).await;
        assert!(node.is_ok());
    }

    #[tokio::test]
    async fn test_block_mining() {
        let config = NodeConfig::default();
        let node = QuantumNode::new(config).await.unwrap();
        
        let request = BlockRequest {
            miner_wallet: "test-wallet".to_string(),
            transactions: vec![],
            timestamp: Utc::now(),
        };

        let consensus = node.consensus.clone();
        let tx_pool = node.transaction_pool.clone();
        
        let response = handle_mine_block(
            web::Json(request),
            web::Data::new(consensus),
            web::Data::new(tx_pool),
        ).await;
        
        assert!(response.status().is_success());
    }
}
