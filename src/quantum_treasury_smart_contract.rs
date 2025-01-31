use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use serde::{Deserialize, Serialize};
use quantumfuse_sdk::{
    ai::PolicyAI,
    finance::{TreasuryState, GovernanceBond},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub initial_reserves: Uint128,
    pub ai_optimized_allocation: bool,
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = TreasuryState {
        reserves: msg.initial_reserves,
        bonds: vec![],
        ai_enabled: msg.ai_optimized_allocation,
    };
    deps.storage.save("treasury_state", &state)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::IssueBond { amount, interest_rate, duration } => issue_bond(deps, info, amount, interest_rate, duration),
        ExecuteMsg::AdjustReserves { adjustment } => adjust_reserves(deps, adjustment),
        ExecuteMsg::AIOptimizeFunds {} => ai_optimize_funds(deps),
    }
}

// 🔹 Issue Governance Bonds
pub fn issue_bond(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
    interest_rate: f64,
    duration: u64,
) -> StdResult<Response> {
    let mut state: TreasuryState = deps.storage.load("treasury_state")?;
    let bond = GovernanceBond {
        bond_id: format!("bond-{}", state.bonds.len() + 1),
        investor: info.sender.to_string(),
        amount,
        interest_rate,
        maturity_date: _env.block.time.plus_seconds(duration),
    };
    state.bonds.push(bond);
    deps.storage.save("treasury_state", &state)?;
    Ok(Response::new().add_attribute("action", "issue_bond"))
}

// 🔹 Adjust Treasury Reserves (Admin)
pub fn adjust_reserves(deps: DepsMut, adjustment: Uint128) -> StdResult<Response> {
    let mut state: TreasuryState = deps.storage.load("treasury_state")?;
    state.reserves += adjustment;
    deps.storage.save("treasury_state", &state)?;
    Ok(Response::new().add_attribute("action", "adjust_reserves"))
}

// 🔹 AI-Optimized Fund Allocation
pub fn ai_optimize_funds(deps: DepsMut) -> StdResult<Response> {
    let mut state: TreasuryState = deps.storage.load("treasury_state")?;
    if !state.ai_enabled {
        return Err(StdError::generic_err("AI optimization is disabled"));
    }

    let optimized_allocation = PolicyAI::default().optimize_treasury_allocation(state.reserves);
    state.reserves = optimized_allocation;
    deps.storage.save("treasury_state", &state)?;

    Ok(Response::new().add_attribute("action", "ai_optimize_funds"))
}
