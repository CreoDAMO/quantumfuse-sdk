// ✅ Import all necessary QuantumFuse SDK modules
use quantumfuse_sdk::{
    run_ai_analytics_dashboard,
    deploy_ai_defi_yield_execution_smart_contract,
    get_defi_yield_optimization_data,
    optimize_defi_yields,
    run_ai_execution_speed_benchmarks,
    get_ai_forecasts,
    display_metaverse_economy_metrics,
    simulate_metaverse_market,
    valuate_metaverse_assets,
    spawn_metaverse_npc_agents,
    apply_quantum_governance_rules,
    get_ai_treasury_data,
    execute_ai_treasury_operations,
    forecast_treasury_balance,
    manage_quantum_governance,
    create_block,
    add_block,
    validate_consensus,
    get_cross_chain_analytics,
    handle_streaming_payments,
};

use tokio::task;
use env_logger;
use std::error::Error;
use std::time::Instant;

/// 🚀 **Main entry point**
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ✅ Initialize logging
    env_logger::init();
    println!("🚀 QuantumFuse SDK is starting...");

    // ✅ Start execution timer
    let start_time = Instant::now();

    // ✅ Run all QuantumFuse SDK functionalities **concurrently**  
    let results = tokio::join!(
        task::spawn_blocking(run_ai_analytics_dashboard),
        task::spawn_blocking(deploy_ai_defi_yield_execution_smart_contract),
        task::spawn_blocking(get_defi_yield_optimization_data),
        task::spawn_blocking(optimize_defi_yields),
        task::spawn_blocking(run_ai_execution_speed_benchmarks),
        task::spawn_blocking(get_ai_forecasts),
        task::spawn_blocking(display_metaverse_economy_metrics),
        task::spawn_blocking(simulate_metaverse_market),
        task::spawn_blocking(valuate_metaverse_assets),
        task::spawn_blocking(spawn_metaverse_npc_agents),
        task::spawn_blocking(apply_quantum_governance_rules),
        task::spawn_blocking(get_ai_treasury_data),
        task::spawn_blocking(execute_ai_treasury_operations),
        task::spawn_blocking(forecast_treasury_balance),
        task::spawn_blocking(manage_quantum_governance),
        task::spawn_blocking(create_block),
        task::spawn_blocking(add_block),
        task::spawn_blocking(validate_consensus),
        task::spawn_blocking(get_cross_chain_analytics),
        task::spawn_blocking(handle_streaming_payments),
    );

    // ✅ Error handling for async tasks
    for result in results {
        match result {
            Ok(Ok(_)) => {} // Task completed successfully
            Ok(Err(e)) => eprintln!("❌ Error: {}", e), // Function returned an error
            Err(e) => eprintln!("⚠️ Task failed: {}", e), // Tokio task failed
        }
    }

    // ✅ Execution time tracking
    let elapsed_time = start_time.elapsed();
    println!("✅ QuantumFuse SDK Execution Completed in {:.2?}!", elapsed_time);

    Ok(())
}