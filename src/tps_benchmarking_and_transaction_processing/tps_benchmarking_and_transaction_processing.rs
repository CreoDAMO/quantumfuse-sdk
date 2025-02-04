use quantumfuse_sdk::blockchain::{Blockchain, Transaction, Shard};
use quantumfuse_sdk::network::{AIEngine, TPSAnalyzer};
use quantumfuse_sdk::consensus::QuantumBridge;
use pqcrypto::sign::dilithium2::{generate_keypair, sign, verify};
use pqcrypto::kem::kyber512::{encapsulate, decapsulate, generate_keypair as kyber_generate};
use tokio::sync::mpsc;
use tokio::task;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::time::Instant;
use log::{info, error};

/// Number of threads for parallel processing
const NUM_THREADS: usize = 8;
/// Benchmark different transaction loads
const TRANSACTION_LOADS: [usize; 3] = [1000, 10_000, 100_000];

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Blockchain::new());
    let ai_engine = AIEngine::new();
    let mut results = Vec::new();

    for &num_transactions in TRANSACTION_LOADS.iter() {
        let (tx, mut rx) = mpsc::channel(num_transactions);
        let mut transactions = Vec::new();

        // Generate transactions with quantum-safe signatures
        for _ in 0..num_transactions {
            let (public_key, secret_key) = generate_keypair();
            let mut transaction = Transaction::new(
                "sender_wallet".to_string(),
                "recipient_wallet".to_string(),
                1.0, // Amount
                0.001, // Fee
                public_key.to_bytes(), // Quantum-safe signature
            );
            let signature = sign(transaction.hash().as_bytes(), &secret_key);
            transaction.attach_signature(signature.to_bytes());

            transactions.push(transaction);
        }

        // Measure TPS
        let start_time = Instant::now();
        let mut handles = Vec::new();

        for _ in 0..NUM_THREADS {
            let blockchain = blockchain.clone();
            let mut rx = rx.clone();
            handles.push(task::spawn(async move {
                while let Some(tx) = rx.recv().await {
                    blockchain.process_transaction(tx).await;
                }
            }));
        }

        // Send transactions for processing
        for transaction in transactions {
            if let Err(e) = tx.send(transaction).await {
                error!("Transaction send error: {:?}", e);
            }
        }
        drop(tx); // Close channel

        // Wait for all transactions to be processed
        for handle in handles {
            let _ = handle.await;
        }

        let elapsed_time = start_time.elapsed();
        let tps = num_transactions as f64 / elapsed_time.as_secs_f64();

        // AI-Powered Performance Analysis
        let bottleneck = ai_engine.detect_bottlenecks(tps);
        info!(
            "Benchmark Completed: Processed {} transactions in {:.2?} seconds. TPS = {:.2}. Bottleneck: {:?}",
            num_transactions, elapsed_time, tps, bottleneck
        );

        results.push((num_transactions, elapsed_time.as_secs_f64(), tps, bottleneck));
    }

    generate_csv_report(&results);
    generate_json_report(&results);
}

/// Generates a CSV report of the TPS benchmarking results
fn generate_csv_report(results: &Vec<(usize, f64, f64, String)>) {
    let mut file = File::create("tps_benchmark_results.csv").expect("Unable to create CSV file");
    writeln!(file, "Transactions,Time (s),TPS,Bottleneck").expect("Unable to write to CSV file");

    for (num_tx, time, tps, bottleneck) in results {
        writeln!(file, "{},{},{},{}", num_tx, time, tps, bottleneck).expect("Unable to write to CSV file");
    }
    info!("CSV report generated: tps_benchmark_results.csv");
}

/// Generates a JSON report of the TPS benchmarking results
fn generate_json_report(results: &Vec<(usize, f64, f64, String)>) {
    let json_data = json!({
        "benchmark_results": results.iter().map(|(num_tx, time, tps, bottleneck)| {
            json!({"transactions": num_tx, "time_seconds": time, "tps": tps, "bottleneck": bottleneck})
        }).collect::<Vec<_>>()
    });

    let mut file = File::create("tps_benchmark_results.json").expect("Unable to create JSON file");
    writeln!(file, "{}", json_data.to_string()).expect("Unable to write to JSON file");
    info!("JSON report generated: tps_benchmark_results.json");
}
