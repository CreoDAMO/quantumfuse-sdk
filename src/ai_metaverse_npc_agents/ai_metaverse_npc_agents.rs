use bevy::prelude::*;
use quantumfuse_sdk::ai::PolicyAI;

#[derive(Component)]
struct VirtualTrader {
    wealth: f64,
    risk_tolerance: f64,
}

fn ai_trading_agents(mut traders: Query<&mut VirtualTrader>, policy_ai: Res<PolicyAI>) {
    for mut trader in traders.iter_mut() {
        let new_investment = policy_ai.recommend_staking_amount(trader.wealth);
        trader.wealth += new_investment;
        println!("💰 Trader Updated: New Wealth -> {:?}", trader.wealth);
    }
}
