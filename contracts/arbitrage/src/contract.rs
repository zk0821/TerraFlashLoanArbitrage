#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, Uint128, Coin, CosmosMsg, WasmMsg, BankMsg, Decimal };
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::exchanges::executor::{ swap_message };
use crate::msg::{ MigrateMsg, InstantiateMsg, ExecuteMsg, QueryMsg, ArbitrageResponse };
use crate::state::{ State, STATE, FactoryState, FACTORIES, ARBITRAGE, FLASH_LOAN };
use crate::exchanges::querier::{ query_pool, simulate_token_swap, simulate_native_token_swap, query_pool_address };
use crate::exchanges::tokens::{ convert_string_to_asset_info, NATIVE_TOKEN_LIST, TOKEN_LIST };
use crate::exchanges::arbitrage::{ calculate_optimal_starting_token_amount, calculate_profit };
use crate::flash_loan::msg::{ ExecuteMsg as FlashLoanExecuteMsg };

use terraswap::pair::{ PoolResponse };
use terraswap::querier::{ query_token_balance };
use terraswap::asset::{ Asset, AssetInfo };

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:arbitrage";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    let factory_state_astro = FactoryState {
        contract_addr: msg.astroport_factory_address
    };
    FACTORIES.save(deps.storage, "ASTROPORT".to_string(),&factory_state_astro)?;
    let factory_state_terraswap = FactoryState {
        contract_addr: msg.terraswap_factory_address
    };
    FACTORIES.save(deps.storage, "TERRASWAP".to_string(),&factory_state_terraswap)?;
    FLASH_LOAN.save(deps.storage, &msg.flash_loan_contract_address)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ExecuteSwap { factory_addr, offer_asset, offer_amount, wanted_asset } => try_execute_swap(deps, factory_addr, offer_asset, offer_amount, wanted_asset),
        ExecuteMsg::ExecuteSwapFullAmount { factory_addr, offer_asset, wanted_asset } => try_execute_swap_full_amount(deps, env, factory_addr, offer_asset, wanted_asset),
        ExecuteMsg::ProvideToFlashLoan { offer_asset, offer_amount } => try_provide_to_flash_loan(deps, info, env, offer_asset, offer_amount),
        ExecuteMsg::WithdrawFromFlashLoan { } => try_withdraw_from_flash_loan(deps, info),
        ExecuteMsg::Withdraw { denom } => try_withdraw(deps, env, info, denom),
        ExecuteMsg::ExecuteArbitrage { } => try_execute_arbitrage(deps, info),
        ExecuteMsg::ReceiveLoan { } => try_receive_loan(deps, env),
        ExecuteMsg::AssertBalance { amount } => try_assert_balance(deps, env, amount),
    }
}

fn try_execute_swap(deps: DepsMut, factory_addr: String, offer_asset: String, offer_amount: String, wanted_asset: String) -> Result<Response, ContractError> {
    let verified_factory_address: Addr = deps.api.addr_validate(&factory_addr)?;
    let verified_offer_asset: AssetInfo = convert_string_to_asset_info(deps.as_ref(), &offer_asset)?;
    let verified_amount: Uint128 = Uint128::try_from(offer_amount.as_str())?;
    let verified_wanted_asset: AssetInfo = convert_string_to_asset_info(deps.as_ref(), &wanted_asset)?;
    //Execute the swap
    let swap_msg = swap_message(deps.as_ref(), verified_factory_address, Asset { info: verified_offer_asset, amount: verified_amount }, verified_wanted_asset)?;
    Ok(Response::new()
        .add_message(swap_msg))
}

fn try_execute_swap_full_amount(deps: DepsMut, env: Env, factory_addr: String, offer_asset: String, wanted_asset: String) -> Result<Response, ContractError> {
    let verified_factory_address: Addr = deps.api.addr_validate(&factory_addr)?;
    let verified_offer_asset: AssetInfo = convert_string_to_asset_info(deps.as_ref(), &offer_asset)?;
    let validated_offer_asset_address: Addr = deps.api.addr_validate(&offer_asset)?;
    let full_amount: Uint128 = query_token_balance(&deps.querier, validated_offer_asset_address, env.contract.address)?;
    if full_amount == Uint128::zero() {
       return Err(ContractError::InvalidSwapAmount { amount: full_amount.clone().to_string(), asset: offer_asset.clone() });
    }
    let verified_wanted_asset: AssetInfo = convert_string_to_asset_info(deps.as_ref(), &wanted_asset)?;
    //Execute the swap
    let swap_msg = swap_message(deps.as_ref(), verified_factory_address, Asset { info: verified_offer_asset, amount: full_amount }, verified_wanted_asset)?;
    Ok(Response::new()
        .add_message(swap_msg))
}

fn try_provide_to_flash_loan(deps: DepsMut, info: MessageInfo, env: Env, offer_asset: String, offer_amount: String) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {  });
    }
    //Load flash loan contract address
    let flash_loan_contract_address: String = FLASH_LOAN.load(deps.storage)?;
    //Verify the data
    let verified_contract_addr = deps.api.addr_validate(&flash_loan_contract_address)?;
    let verified_offer_asset: AssetInfo = convert_string_to_asset_info(deps.as_ref(), &offer_asset)?;
    let verified_amount: Uint128 = Uint128::try_from(offer_amount.as_str())?;
    let queried_balance: Coin = match verified_offer_asset {
        AssetInfo::NativeToken { denom } => try_query_balance(deps.as_ref(), env, denom)?,
        AssetInfo::Token { contract_addr } => try_query_balance(deps.as_ref(), env, contract_addr)?
    };
    if queried_balance.amount < verified_amount {
        return Err(ContractError::BalanceTooSmall { token: offer_asset, available: queried_balance.amount.to_string(), expected: offer_amount });
    }
    let provide_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: verified_contract_addr.to_string(),
        msg: to_binary(&FlashLoanExecuteMsg::Provide {  })?,
        funds: vec![Coin {
            denom: offer_asset,
            amount: verified_amount,
        }]
    });
    Ok(Response::new().add_message(provide_msg))
}

fn try_withdraw_from_flash_loan(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {  });
    }
    //Load flash loan contract address
    let flash_loan_contract_address: String = FLASH_LOAN.load(deps.storage)?;
    //Verify the contract address
    let verified_contract_addr = deps.api.addr_validate(&flash_loan_contract_address)?;
    let withdraw_msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: verified_contract_addr.to_string(),
        msg: to_binary(&FlashLoanExecuteMsg::Withdraw {  })?,
        funds: vec![]
    });
    Ok(Response::new().add_message(withdraw_msg))
}

fn try_withdraw(deps: DepsMut, env: Env, info: MessageInfo, denom: String) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {  });
    }
    let _verified_asset = convert_string_to_asset_info(deps.as_ref(), &denom)?;
    let current_balance = deps.querier.query_balance(env.contract.address, denom.clone())?;
    if current_balance.amount == Uint128::zero() {
        return Err(ContractError::BalanceTooSmall { token: denom.clone(), available: current_balance.amount.to_string(), expected: " > 0".to_string() })
    }
    let withdraw_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![current_balance],
    });
    Ok(Response::new().add_message(withdraw_msg))
}

fn try_execute_arbitrage(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.owner != info.sender {
        return Err(ContractError::Unauthorized {  });
    }
    //Estimate the arbitrage
    let arbitrage_estimation = try_estimate_arbitrage(deps.as_ref())?;
    let mut biggest_arbitrage_opportunity: ArbitrageResponse = arbitrage_estimation[0].clone();
    for opportunity in &arbitrage_estimation[1..]{
        if opportunity.simulated_profit > biggest_arbitrage_opportunity.simulated_profit {
            biggest_arbitrage_opportunity = opportunity.clone()
        }
    }
    //Save the arbitrage to state
    ARBITRAGE.save(deps.storage, &biggest_arbitrage_opportunity)?;
    //Load flash loan contract address
    let flash_loan_contract_addr: String = FLASH_LOAN.load(deps.storage)?;
    //Get the funds from the flash loan
    let _verified_flash_loan_contract_address = deps.api.addr_validate(&flash_loan_contract_addr)?;
    let loan_receive_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: flash_loan_contract_addr.clone(),
        msg: to_binary(&FlashLoanExecuteMsg::Loan { amount: biggest_arbitrage_opportunity.optimal_starting_amount.to_string() })?,
        funds: vec![],
    });
    Ok(Response::new()
        .add_message(loan_receive_msg))

}

fn try_receive_loan(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let arbitrage_opportunity: ArbitrageResponse = ARBITRAGE.load(deps.storage)?;
    //Look at starting balance
    let starting_balance: Uint128 = try_query_balance(deps.as_ref(), env.clone(), arbitrage_opportunity.native_token.clone())?.amount - arbitrage_opportunity.optimal_starting_amount;
    //Loan has been received -> Begin swapping
    let swap_message_a = WasmMsg::Execute {
        contract_addr: env.contract.address.clone().to_string(),
        msg: to_binary(&ExecuteMsg::ExecuteSwap { factory_addr: arbitrage_opportunity.starting_exchange_factory.clone(), offer_asset: arbitrage_opportunity.native_token.clone(), offer_amount: arbitrage_opportunity.optimal_starting_amount.to_string(), wanted_asset: arbitrage_opportunity.token.clone() })?,
        funds: vec![],
    };
    //Execute the swap B
    let swap_message_b = WasmMsg::Execute {
        contract_addr: env.contract.address.clone().to_string(),
        msg: to_binary(&ExecuteMsg::ExecuteSwapFullAmount { factory_addr: arbitrage_opportunity.ending_exchange_factory.clone(), offer_asset: arbitrage_opportunity.token.clone(), wanted_asset: arbitrage_opportunity.native_token.clone() })?,
        funds: vec![],
    };
    //Load flash loan contract address
    let flash_loan_contract_address = FLASH_LOAN.load(deps.storage)?;
    //Repay the loan = optimal_starting_amount + (optimal_staring_amount * fee)
    let amount_to_repay: Uint128 = arbitrage_opportunity.optimal_starting_amount + (arbitrage_opportunity.optimal_starting_amount * Decimal::permille(9));
    let loan_repay_msg: BankMsg =  BankMsg::Send {
        to_address: flash_loan_contract_address.clone(),
        amount: vec![Coin { denom: arbitrage_opportunity.native_token.clone(), amount: amount_to_repay }],
    };
    //Assert finishing balance is bigger than before arbitrage
    let assert_arbitrage_successful_msg = WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_binary(&ExecuteMsg::AssertBalance { amount: starting_balance.to_string() })?,
        funds: vec![],
    };
    Ok(Response::new()
        .add_message(swap_message_a)
        .add_message(swap_message_b)
        .add_message(loan_repay_msg)
        .add_message(assert_arbitrage_successful_msg))
}

fn try_assert_balance(deps: DepsMut, env: Env, amount: String) -> Result<Response, ContractError> {
    let arbitrage_opportunity: ArbitrageResponse = ARBITRAGE.load(deps.storage)?;
    let current_balance = deps.querier.query_balance(env.contract.address, arbitrage_opportunity.native_token.clone())?;
    let validated_previous_amount = Uint128::try_from(amount.as_str())?;
    if current_balance.amount <= validated_previous_amount {
        return Err(ContractError::ArbitrageUnsuccessful { current: current_balance.to_string(), previous: amount });
    }
    Ok(Response::new()
        .add_attribute("method", "try assert balance"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryBalances {  } => to_binary(&try_query_balances(deps, env)?),
        QueryMsg::QueryPool { factory_addr, offer_asset, wanted_asset } => to_binary(&try_query_pool(deps, factory_addr, offer_asset, wanted_asset)?),
        QueryMsg::SimulateSwap { factory_addr, offer_asset, amount, wanted_asset } => to_binary(&try_simulate_swap(deps, factory_addr, offer_asset, amount, wanted_asset)?),
        QueryMsg::EstimateArbitrage {  } => to_binary(&try_estimate_arbitrage(deps)?),
    }
}

fn try_query_balance(deps: Deps, env: Env, token: String) -> StdResult<Coin> {
    let balance: Coin = deps.querier.query_balance(&env.contract.address, token)?;
    Ok(balance)
}

fn try_query_balances(deps: Deps, env: Env) -> StdResult<Vec<Coin>> {
    let all_balances: Vec<Coin> = deps.querier.query_all_balances(&env.contract.address)?;
    Ok(all_balances)
}

fn try_query_pool(deps: Deps, factory_addr:String, offer_asset: String, wanted_asset: String) -> StdResult<PoolResponse> {
    let pool_address:Addr = query_pool_address(deps, factory_addr.as_str(), offer_asset.as_str(), wanted_asset.as_str())?;
    let response: PoolResponse = query_pool(deps, pool_address)?;
    Ok(response)
}

fn try_simulate_swap(deps: Deps, factory_addr: String, offer_asset: String, offer_amount: String, wanted_asset: String) -> StdResult<Uint128> {
    let valid_amount: Uint128 = Uint128::try_from(offer_amount.as_str())?;
    let pool_address: Addr = query_pool_address(deps, factory_addr.as_str(), offer_asset.as_str(), wanted_asset.as_str())?;
    let response: Uint128;
    let validated_offer_asset: AssetInfo = convert_string_to_asset_info(deps, offer_asset.as_str())?;
    match validated_offer_asset {
        AssetInfo::NativeToken { denom } => {
            response = simulate_native_token_swap(deps, pool_address, Coin { amount: valid_amount, denom: denom})?;
        },
        AssetInfo::Token { contract_addr } => {
            response = simulate_token_swap(deps, pool_address, Coin { amount: valid_amount, denom: contract_addr})?;
        }
    }
    Ok(response)
}

fn try_estimate_arbitrage(deps: Deps) -> StdResult<Vec<ArbitrageResponse>> {
    let astroport_factory: FactoryState = FACTORIES.load(deps.storage, "ASTROPORT".to_string())?;
    let terraswap_factory: FactoryState = FACTORIES.load(deps.storage, "TERRASWAP".to_string())?;
    let mut arbitrage_list:Vec<ArbitrageResponse> = Vec::new();
    for native_token in NATIVE_TOKEN_LIST {
        for token in TOKEN_LIST {
            //Check Liquidity Pools
            let astro_pool_response: PoolResponse = try_query_pool(deps.clone(), astroport_factory.contract_addr.clone(), native_token.id.to_string(), token.id.to_string())?;
            let terraswap_pool_response: PoolResponse = try_query_pool(deps.clone(), terraswap_factory.contract_addr.clone(), native_token.id.to_string(), token.id.to_string())?;
            if astro_pool_response.total_share == Uint128::zero() || terraswap_pool_response.total_share == Uint128::zero() {
                continue;
            }
            //Read Liquidity pools into variables
            let (mut astroport_native_token_amount, mut astroport_token_amount, mut terraswap_native_token_amount, mut terraswap_token_amount): (Uint128, Uint128, Uint128, Uint128) = (Uint128::zero(), Uint128::zero(), Uint128::zero(), Uint128::zero());
            for asset in astro_pool_response.assets {
                match asset.info {
                    AssetInfo::NativeToken { denom: _ } => astroport_native_token_amount = asset.amount,
                    AssetInfo::Token { contract_addr: _ } => astroport_token_amount = asset.amount,
                }
            }
            for asset in terraswap_pool_response.assets {
                match asset.info {
                    AssetInfo::NativeToken { denom: _ } => terraswap_native_token_amount = asset.amount,
                    AssetInfo::Token { contract_addr: _ } => terraswap_token_amount = asset.amount,
                }
            }
            let starting_amount_from_astroport_to_terraswap: Uint128 = match calculate_optimal_starting_token_amount(astroport_native_token_amount, astroport_token_amount, terraswap_native_token_amount, terraswap_token_amount) {
                Ok(value) => value,
                Err(..) => Uint128::zero(),
            };
            let starting_amount_from_terraswap_to_astroport = match calculate_optimal_starting_token_amount(terraswap_native_token_amount, terraswap_token_amount, astroport_native_token_amount, astroport_token_amount) {
                Ok(value) => value,
                Err(..) => Uint128::zero(),
            };
            let profit_from_astroport_to_terraswap: Uint128 = calculate_profit(starting_amount_from_astroport_to_terraswap, astroport_native_token_amount, astroport_token_amount, terraswap_native_token_amount, terraswap_token_amount)?;
            let profit_from_terraswap_to_astroport: Uint128 = calculate_profit(starting_amount_from_terraswap_to_astroport, terraswap_native_token_amount, terraswap_token_amount, astroport_native_token_amount, astroport_token_amount)?;
            //Finalize the arbitrage estimation
            let (starting_exchange_factory, ending_exchange_factory, starting_exchange, ending_exchange, starting_amount, calculated_profit): (String, String, String, String, Uint128, Uint128);
            if starting_amount_from_astroport_to_terraswap > Uint128::zero() {
                starting_exchange_factory = astroport_factory.contract_addr.clone();
                ending_exchange_factory = terraswap_factory.contract_addr.clone();
                starting_exchange = String::from("Astroport");
                ending_exchange = String::from("Terraswap");
                starting_amount = starting_amount_from_astroport_to_terraswap;
                calculated_profit = profit_from_astroport_to_terraswap;
            } else if starting_amount_from_terraswap_to_astroport > Uint128::zero() {
                starting_exchange_factory = terraswap_factory.contract_addr.clone();
                ending_exchange_factory = astroport_factory.contract_addr.clone();
                starting_exchange = String::from("Terraswap");
                ending_exchange = String::from("Astroport");
                starting_amount = starting_amount_from_terraswap_to_astroport;
                calculated_profit = profit_from_terraswap_to_astroport;
            } else {
                //No arbitrage opportunity found
                continue;
            }
            //Native token -> Token
            let exchanged_token_amount: Uint128 = try_simulate_swap(deps, starting_exchange_factory.clone(), native_token.id.to_string(), starting_amount.to_string(), token.id.to_string())?;
            //Token -> Native token
            let exchange_native_token_amount: Uint128 = try_simulate_swap(deps, ending_exchange_factory.clone(), token.id.to_string(), exchanged_token_amount.to_string(), native_token.id.to_string())?;
            //Calculate simulated profit
            let simulated_profit = exchange_native_token_amount - starting_amount;
            arbitrage_list.push(ArbitrageResponse { native_token: native_token.id.to_string(), token: token.id.to_string(), starting_exchange: starting_exchange, starting_exchange_factory: starting_exchange_factory.clone(), ending_exchange: ending_exchange, ending_exchange_factory: ending_exchange_factory.clone(), optimal_starting_amount: starting_amount, calculated_profit: calculated_profit, simulated_profit: simulated_profit });
        }
    }
    Ok(arbitrage_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { astroport_factory_address: String::from("a"), terraswap_factory_address: String::from("b"), flash_loan_contract_address: String::from("c") };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
