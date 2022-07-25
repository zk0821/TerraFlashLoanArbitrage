#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, Uint128, Coin, StdError};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use crate::exchanges::querier::{ query_pool, simulate_token_swap, simulate_native_token_swap };
use crate::exchanges::pools::{ Pool, ASTROPORT_POOLS, TERRASWAP_POOLS };

use terraswap::pair::PoolResponse;
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:arb";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryPool { address }=> to_binary(&try_query_pool(deps, address)?),
        QueryMsg::SimulateNativeTokenSwap { address, amount, denom } => to_binary(&try_simulate_swap(deps, address, amount, denom, true)?),
        QueryMsg::SimulateTokenSwap { address, amount, contract_addr } => to_binary(&try_simulate_swap(deps, address, amount, contract_addr, false)?),
        QueryMsg::EstimateArbitrage {  } => to_binary(&try_estimate_arbitrage(deps)?),
    }
}

fn try_query_pool(deps: Deps,  address: String) -> StdResult<PoolResponse> {
    let valid_address: Addr = deps.api.addr_validate(&address)?;
    let response: PoolResponse = query_pool(deps, valid_address)?;
    Ok(response)
}

fn try_simulate_swap(deps: Deps, address: String, token_amount: String, token_denom: String, native: bool) -> StdResult<Uint128> {
    let valid_address: Addr = deps.api.addr_validate(&address)?;
    let valid_amount: Uint128 = Uint128::try_from(token_amount.as_str())?;
    let response: Uint128;
    if native {
        response = simulate_native_token_swap(deps, valid_address, Coin { amount: valid_amount, denom: token_denom})?;
    } else {
        response = simulate_token_swap(deps, valid_address, Coin { amount: valid_amount, denom: token_denom})?;
    }
    Ok(response)
}

fn try_estimate_arbitrage(deps: Deps) -> StdResult<Uint128> {
    let astroport_pool_number = ASTROPORT_POOLS.len();
    let terraswap_pool_number = TERRASWAP_POOLS.len();
    if astroport_pool_number != terraswap_pool_number {
        return Err(StdError::generic_err("Not an equal number of pools between exchanges!"));
    }
    let arbitrage_amount:Uint128 = Uint128::zero();
    //Hardcoded for now
    let starting_native_token_amount:Uint128 = Uint128::from(100u128);
    for pool_number in 0..astroport_pool_number {
        let astroport_pool: &Pool = &ASTROPORT_POOLS[pool_number];
        let terraswap_pool: &Pool = &TERRASWAP_POOLS[pool_number];
        let valid_astroport_pool_address: Addr = deps.api.addr_validate(astroport_pool.contract_address)?;
        let token_amount: Uint128 = simulate_native_token_swap(deps, valid_astroport_pool_address, Coin { amount: starting_native_token_amount, denom: astroport_pool.native_token.to_string()})?;
        let valid_terraswap_pool_address: Addr = deps.api.addr_validate(terraswap_pool.contract_address)?;
        let native_token_amount: Uint128 = simulate_token_swap(deps, valid_terraswap_pool_address, Coin { amount: token_amount, denom: terraswap_pool.token.to_string()})?;
        arbitrage_amount.checked_add(native_token_amount.checked_sub(starting_native_token_amount)?)?;
    }
    Ok(arbitrage_amount)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryPool { address: "terra1udsua9w6jljwxwgwsegvt6v657rg3ayfvemupnes7lrggd28s0wq7g8azm".to_string() }).unwrap();
        let value: PoolResponse = from_binary(&res).unwrap();
        println!("{}", value.assets[0]);
    }
}
