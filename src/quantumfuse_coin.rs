use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use log::{info, error, warn};
use quantumfuse_sdk::{
    error::QFCError,
    wallet::QuantumWallet,
    crypto::{Hash, KeyPair},
    state::StateAccess,
    metrics::TokenMetrics
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumFuseCoin {
    total_supply: u64,
    circulating_supply: u64,
    allocation: HashMap<String, AllocationDetails>,
    balances: HashMap<String, Balance>,
    staking: HashMap<String, StakingInfo>,
    vesting_schedules: HashMap<String, VestingSchedule>,
    metrics: TokenMetrics,
    last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationDetails {
    allocation_type: AllocationType,
    allocation_percentage: f64,
    vesting_years: u32,
    allocated_tokens: u64,
    claimed_tokens: u64,
    last_claim: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllocationType {
    FoundersTeam,
    Advisors,
    PrivateSale,
    PublicSale,
    EcosystemFund,
    StakingRewards,
    Treasury,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    available: u64,
    locked: u64,
    staked: u64,
    last_transaction: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingInfo {
    amount: u64,
    start_time: DateTime<Utc>,
    unlock_time: DateTime<Utc>,
    rewards_earned: u64,
    compound_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingSchedule {
    total_amount: u64,
    released_amount: u64,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    cliff_duration: Duration,
    release_interval: Duration,
    next_release: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRecord {
    hash: Hash,
    transaction_type: TransactionType,
    from: String,
    to: String,
    amount: u64,
    timestamp: DateTime<Utc>,
    status: TransactionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Transfer,
    Mint,
    Burn,
    Stake,
    Unstake,
    ClaimRewards,
    VestingRelease,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed(String),
}

impl QuantumFuseCoin {
    pub fn new() -> Self {
        let mut allocation = HashMap::new();
        allocation.insert(
            "FoundersTeam".to_string(),
            AllocationDetails {
                allocation_type: AllocationType::FoundersTeam,
                allocation_percentage: 0.15,
                vesting_years: 4,
                allocated_tokens: 0,
                claimed_tokens: 0,
                last_claim: None,
            },
        );
        // Add other allocations...

        Self {
            total_supply: 1_000_000_000,
            circulating_supply: 0,
            allocation,
            balances: HashMap::new(),
            staking: HashMap::new(),
            vesting_schedules: HashMap::new(),
            metrics: TokenMetrics::default(),
            last_updated: Utc::now(),
        }
    }

    pub fn mint(&mut self, wallet_id: &str, amount: u64) -> Result<TransactionRecord, QFCError> {
        if amount == 0 {
            return Err(QFCError::InvalidAmount);
        }

        if self.circulating_supply + amount > self.total_supply {
            return Err(QFCError::ExceedsTotalSupply);
        }

        let balance = self.balances.entry(wallet_id.to_string())
            .or_insert(Balance {
                available: 0,
                locked: 0,
                staked: 0,
                last_transaction: Utc::now(),
            });

        balance.available += amount;
        self.circulating_supply += amount;

        let transaction = TransactionRecord {
            hash: self.generate_transaction_hash()?,
            transaction_type: TransactionType::Mint,
            from: "0x0".to_string(),
            to: wallet_id.to_string(),
            amount,
            timestamp: Utc::now(),
            status: TransactionStatus::Completed,
        };

        self.update_metrics()?;
        Ok(transaction)
    }

    pub fn transfer(
        &mut self,
        sender: &str,
        recipient: &str,
        amount: u64,
    ) -> Result<TransactionRecord, QFCError> {
        if amount == 0 {
            return Err(QFCError::InvalidAmount);
        }

        let sender_balance = self.balances.get(sender)
            .ok_or(QFCError::InsufficientBalance)?;

        if sender_balance.available < amount {
            return Err(QFCError::InsufficientBalance);
        }

        // Update sender balance
        if let Some(balance) = self.balances.get_mut(sender) {
            balance.available -= amount;
            balance.last_transaction = Utc::now();
        }

        // Update recipient balance
        let recipient_balance = self.balances.entry(recipient.to_string())
            .or_insert(Balance {
                available: 0,
                locked: 0,
                staked: 0,
                last_transaction: Utc::now(),
            });
        recipient_balance.available += amount;

        let transaction = TransactionRecord {
            hash: self.generate_transaction_hash()?,
            transaction_type: TransactionType::Transfer,
            from: sender.to_string(),
            to: recipient.to_string(),
            amount,
            timestamp: Utc::now(),
            status: TransactionStatus::Completed,
        };

        self.update_metrics()?;
        Ok(transaction)
    }

    pub fn stake(&mut self, wallet_id: &str, amount: u64) -> Result<TransactionRecord, QFCError> {
        if amount == 0 {
            return Err(QFCError::InvalidAmount);
        }

        let balance = self.balances.get_mut(wallet_id)
            .ok_or(QFCError::InsufficientBalance)?;

        if balance.available < amount {
            return Err(QFCError::InsufficientBalance);
        }

        balance.available -= amount;
        balance.staked += amount;

        let staking_info = StakingInfo {
            amount,
            start_time: Utc::now(),
            unlock_time: Utc::now() + Duration::days(30),
            rewards_earned: 0,
            compound_rate: 0.1,
        };

        self.staking.insert(wallet_id.to_string(), staking_info);

        let transaction = TransactionRecord {
            hash: self.generate_transaction_hash()?,
            transaction_type: TransactionType::Stake,
            from: wallet_id.to_string(),
            to: "Staking Pool".to_string(),
            amount,
            timestamp: Utc::now(),
            status: TransactionStatus::Completed,
        };

        self.update_metrics()?;
        Ok(transaction)
    }

    pub fn create_vesting_schedule(
        &mut self,
        wallet_id: &str,
        amount: u64,
        duration_years: u32,
    ) -> Result<VestingSchedule, QFCError> {
        if amount == 0 {
            return Err(QFCError::InvalidAmount);
        }

        let start_time = Utc::now();
        let end_time = start_time + Duration::days(365 * duration_years as i64);
        let cliff_duration = Duration::days(365);
        let release_interval = Duration::days(30);

        let schedule = VestingSchedule {
            total_amount: amount,
            released_amount: 0,
            start_time,
            end_time,
            cliff_duration,
            release_interval,
            next_release: start_time + cliff_duration,
        };

        self.vesting_schedules.insert(wallet_id.to_string(), schedule.clone());
        Ok(schedule)
    }

    pub fn claim_vested_tokens(&mut self, wallet_id: &str) -> Result<TransactionRecord, QFCError> {
        let schedule = self.vesting_schedules.get_mut(wallet_id)
            .ok_or(QFCError::NoVestingSchedule)?;

        let now = Utc::now();
        if now < schedule.next_release {
            return Err(QFCError::VestingNotReady);
        }

        let claimable_amount = self.calculate_vested_amount(schedule)?;
        if claimable_amount == 0 {
            return Err(QFCError::NoTokensToRelease);
        }

        schedule.released_amount += claimable_amount;
        schedule.next_release += schedule.release_interval;

        self.mint(wallet_id, claimable_amount)
    }

    pub fn get_metrics(&self) -> TokenMetrics {
        self.metrics.clone()
    }

    // Private helper methods
    fn calculate_vested_amount(&self, schedule: &VestingSchedule) -> Result<u64, QFCError> {
        let now = Utc::now();
        if now < schedule.start_time + schedule.cliff_duration {
            return Ok(0);
        }

        let total_duration = (schedule.end_time - schedule.start_time).num_seconds() as f64;
        let elapsed = (now - schedule.start_time).num_seconds() as f64;
        let vesting_ratio = (elapsed / total_duration).min(1.0);

        let total_vested = (schedule.total_amount as f64 * vesting_ratio) as u64;
        Ok(total_vested.saturating_sub(schedule.released_amount))
    }

    fn generate_transaction_hash(&self) -> Result<Hash, QFCError> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&Utc::now().timestamp().to_le_bytes());
        Ok(Hash::from(hasher.finalize()))
    }

    fn update_metrics(&mut self) -> Result<(), QFCError> {
        self.metrics = TokenMetrics {
            total_supply: self.total_supply,
            circulating_supply: self.circulating_supply,
            total_staked: self.calculate_total_staked()?,
            holder_count: self.balances.len(),
            last_updated: Utc::now(),
        };
        Ok(())
    }

    fn calculate_total_staked(&self) -> Result<u64, QFCError> {
        Ok(self.staking.values().map(|info| info.amount).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let qfc = QuantumFuseCoin::new();
        assert_eq!(qfc.total_supply, 1_000_000_000);
        assert_eq!(qfc.circulating_supply, 0);
    }

    #[test]
    fn test_token_minting() {
        let mut qfc = QuantumFuseCoin::new();
        let result = qfc.mint("test_wallet", 1000);
        assert!(result.is_ok());
        
        let balance = qfc.balances.get("test_wallet").unwrap();
        assert_eq!(balance.available, 1000);
    }

    #[test]
    fn test_token_transfer() {
        let mut qfc = QuantumFuseCoin::new();
        qfc.mint("sender", 1000).unwrap();
        
        let result = qfc.transfer("sender", "recipient", 500);
        assert!(result.is_ok());
        
        let sender_balance = qfc.balances.get("sender").unwrap();
        let recipient_balance = qfc.balances.get("recipient").unwrap();
        
        assert_eq!(sender_balance.available, 500);
        assert_eq!(recipient_balance.available, 500);
    }

    #[test]
    fn test_vesting_schedule() {
        let mut qfc = QuantumFuseCoin::new();
        let result = qfc.create_vesting_schedule("test_wallet", 1000, 2);
        assert!(result.is_ok());
        
        let schedule = qfc.vesting_schedules.get("test_wallet").unwrap();
        assert_eq!(schedule.total_amount, 1000);
        assert_eq!(schedule.released_amount, 0);
    }
}
