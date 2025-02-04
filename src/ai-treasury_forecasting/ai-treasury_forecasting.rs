use quantumfuse_sdk::{
    finance::{QuantumTreasury, GovernanceBond},
    ai::PolicyAI,
};
use chrono::{Utc, Duration};
use rand::Rng;

pub struct TreasuryAI {
    pub treasury: QuantumTreasury,
    pub ai_engine: PolicyAI,
}

impl TreasuryAI {
    pub fn new() -> Self {
        TreasuryAI {
            treasury: QuantumTreasury::default(),
            ai_engine: PolicyAI::default(),
        }
    }

    pub fn predict_staking_yields(&self, days_ahead: i64) -> f64 {
        let base_rate = self.treasury.get_current_staking_rate();
        let market_trend = rand::thread_rng().gen_range(-0.01..0.02); 
        let prediction = base_rate * (1.0 + market_trend * days_ahead as f64 / 365.0);
        prediction
    }

    pub fn optimize_treasury_reserves(&mut self) {
        let optimized_allocation = self.ai_engine.optimize_treasury_allocation(self.treasury.total_reserves);
        self.treasury.update_reserves(optimized_allocation);
        println!("🚀 Treasury Reserves Optimized: {:?}", optimized_allocation);
    }
}
