// ✅ Import everything correctly from `lib.rs`
use quantumfuse_sdk::ai_analytics_dashboard::run as run_ai_analytics_dashboard;
use quantumfuse_sdk::ai_defi_yield_execution_smart_contract::deploy as deploy_ai_defi_yield_execution_smart_contract;
use quantumfuse_sdk::ai_defi_yield_optimization_api::get_optimization_data as get_defi_yield_optimization_data;
use quantumfuse_sdk::ai_defi_yield_optimization::optimize_yields as optimize_defi_yields;
use quantumfuse_sdk::ai_execution_speed_benchmarking::run_benchmarks as run_ai_execution_speed_benchmarks;
use quantumfuse_sdk::ai_forecasting_api::get_forecasts as get_ai_forecasts;
use quantumfuse_sdk::ai_metaverse_economy_dashboard::display_metrics as display_metaverse_economy_metrics;
use quantumfuse_sdk::ai_metaverse_market_simulation::simulate_market as simulate_metaverse_market;
use quantumfuse_sdk::ai_metaverse_nft_and_land_valuation::valuate_assets as valuate_metaverse_assets;
use quantumfuse_sdk::ai_metaverse_npc_agents::spawn_agents as spawn_metaverse_npc_agents;
use quantumfuse_sdk::ai_quantum_governance_system::apply_governance_rules as apply_quantum_governance_rules;
use quantumfuse_sdk::ai_treasury_api::get_treasury_data as get_ai_treasury_data;
use quantumfuse_sdk::ai_treasury_execution_smart_contract::execute_treasury_operations as execute_ai_treasury_operations;
use quantumfuse_sdk::ai_treasury_forecasting::forecast_treasury_balance;
use quantumfuse_sdk::ai_quantum_governance::manage_quantum_governance;
use quantumfuse_sdk::block::create_block;
use quantumfuse_sdk::blockchain::add_block;
use quantumfuse_sdk::consensus_mechanism::validate_consensus;
use quantumfuse_sdk::cross_chain_treasury_analytics_api::get_cross_chain_analytics;
use quantumfuse_sdk::qfc_streaming_payments_smart_contract::handle_streaming_payments;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    println!("🚀 QuantumFuse SDK is starting...");

    // ✅ Call all necessary functions
    run_ai_analytics_dashboard()?;
    deploy_ai_defi_yield_execution_smart_contract()?;
    get_defi_yield_optimization_data()?;
    optimize_defi_yields()?;
    run_ai_execution_speed_benchmarks()?;
    get_ai_forecasts()?;
    display_metaverse_economy_metrics()?;
    simulate_metaverse_market()?;
    valuate_metaverse_assets()?;
    spawn_metaverse_npc_agents()?;
    apply_quantum_governance_rules()?;
    get_ai_treasury_data()?;
    execute_ai_treasury_operations()?;
    forecast_treasury_balance()?;
    manage_quantum_governance()?;
    create_block()?;
    add_block()?;
    validate_consensus()?;
    get_cross_chain_analytics()?;
    handle_streaming_payments()?;

    println!("✅ QuantumFuse SDK Execution Completed!");
    Ok(())
}
