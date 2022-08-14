use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Decimal, Addr, Coin, BankMsg, Uint128, CosmosMsg, WasmMsg };
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ MigrateMsg, InstantiateMsg, ExecuteMsg, QueryMsg, ConfigResponse };
use crate::state::{ OWNER, FEE, LOAN_DENOM, BALANCE };
use crate::loans::tokens::{ ALLOWED_TOKENS };
use crate::arbitrage::msg::{ ArbitrageMsg };

use terraswap::asset::{ AssetInfo };

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:flash_loan";
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
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
   
    OWNER.save(deps.storage, &info.sender)?;

    let validated_fee: Decimal = Decimal::from_str(msg.fee.as_str())?;
    FEE.save(deps.storage, &validated_fee)?;

    if ALLOWED_TOKENS.contains(&msg.loan_denom.clone().as_str()) {
        LOAN_DENOM.save(deps.storage, &AssetInfo::NativeToken { denom: msg.loan_denom.clone() })?;
    } else {
        return Err(ContractError::IncorrectToken {  });
    }

    BALANCE.save(deps.storage, &Uint128::zero())?;
    
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("fee", msg.fee)
        .add_attribute("loan_denom", msg.loan_denom))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { owner, fee } => try_update_config(deps, info, owner, fee),
        ExecuteMsg::Provide {  } => try_provide(deps, info),
        ExecuteMsg::Withdraw {  } => try_withdraw(deps, info),
        ExecuteMsg::Loan { amount } => try_loan(deps, env, info, amount),
        ExecuteMsg::AssertBalance { amount } => try_assert_balance(deps, env, amount),
    }
}

fn try_update_config(deps: DepsMut, info: MessageInfo, owner: String, fee: String) -> Result<Response, ContractError> {
    let current_owner = OWNER.load(deps.storage)?;
    if info.sender != current_owner {
        return Err(ContractError::Unauthorized {  });
    }
    let validated_new_owner = deps.api.addr_validate(&owner)?;
    OWNER.save(deps.storage, &validated_new_owner)?;

    let validated_decimal = Decimal::from_str(fee.as_str())?;
    FEE.save(deps.storage, &validated_decimal)?;
    Ok(Response::new()
        .add_attribute("method", "try update config")
        .add_attribute("new owner",  owner)
        .add_attribute("new fee", fee))
}

fn try_provide(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let MessageInfo { sender, funds } = info.clone();
    let current_owner: Addr = OWNER.load(deps.storage)?;
    if current_owner != sender {
        return Err(ContractError::Unauthorized {  });
    }
    let current_loan_denom = LOAN_DENOM.load(deps.storage)?;
    if funds.len() != 1 {
        return Err(ContractError::IncorrectFunds { msg: "Only a single token should be provided!".to_string() });
    }
    let provided_funds: Coin = funds.into_iter().next().unwrap();
    match current_loan_denom {
        AssetInfo::NativeToken { denom } => {
            if provided_funds.denom != denom {
                return Err(ContractError::IncorrectFunds { msg: format!("Denom ({}) does not match current loan denom ({})", provided_funds.denom, denom) });
            }
        },
        AssetInfo::Token { .. } => {
            return Err(ContractError::IncorrectToken {  });
        }
    }
    BALANCE.update(deps.storage, |old| -> StdResult<_> { Ok(old.checked_add(provided_funds.amount)?) })?;
    Ok(Response::new()
        .add_attribute("method", "try provide")
        .add_attribute("provider", info.sender)
        .add_attribute("provided", provided_funds.amount))
}

fn try_withdraw(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let request_sender = info.sender;
    let current_owner = OWNER.load(deps.storage)?;
    if request_sender != current_owner {
        return Err(ContractError::Unauthorized {  });
    }
    let current_balance = BALANCE.load(deps.storage)?;
    if current_balance == Uint128::zero() {
        return Err(ContractError::EmptyBalance {  });
    }
    let current_loan_denom = LOAN_DENOM.load(deps.storage)?;
    let withdraw_msg: CosmosMsg = match current_loan_denom {
        AssetInfo::NativeToken { denom } => BankMsg::Send{
            to_address: request_sender.to_string(),
            amount: vec![Coin {
                amount: current_balance,
                denom: denom,
            }]
        }.into(),
        AssetInfo::Token { .. } => {
            return Err(ContractError::IncorrectToken {  });
        }
    };
    BALANCE.save(deps.storage, &Uint128::zero())?;
    Ok(Response::new()
        .add_attribute("method", "try_withdraw")
        .add_attribute("receiver", request_sender)
        .add_attribute("amount withdrawn", current_balance)
        .add_message(withdraw_msg))
}

fn try_loan(deps: DepsMut, env: Env, info: MessageInfo, amount: String ) -> Result<Response, ContractError> {
    let current_loan_denom = LOAN_DENOM.load(deps.storage)?;
    let available_balance = match current_loan_denom.clone() {
        AssetInfo::NativeToken { denom } => deps.querier.query_balance(env.contract.address.clone(), denom)?,
        AssetInfo::Token { .. } => {
            return Err(ContractError::IncorrectToken {  });
        }
    };
    let validated_amount: Uint128 = Uint128::try_from(amount.as_str())?;
    if validated_amount > available_balance.amount {
        return Err(ContractError::RequestedAmountTooLarge {  });
    }
    let loan_msg = match current_loan_denom {
        AssetInfo::NativeToken { denom } => WasmMsg::Execute{
            contract_addr: info.sender.to_string(),
            msg: to_binary(&ArbitrageMsg::ReceiveLoan {  })?,
            funds: vec![Coin { denom: denom, amount: validated_amount }]
        },
        AssetInfo::Token { .. } => {
            return Err(ContractError::IncorrectToken {  });
        }
    };
    let current_fee = FEE.load(deps.storage)?;
    let expected_balance_after_return = available_balance.amount + (current_fee * validated_amount);

    let loan_repaid_msg = WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_binary(&ExecuteMsg::AssertBalance { amount: expected_balance_after_return.to_string() })?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_attribute("method", "try loan")
        .add_attribute("receiver", info.sender)
        .add_message(loan_msg)
        .add_message(loan_repaid_msg))
}

fn try_assert_balance(deps: DepsMut, env: Env, amount: String) -> Result<Response, ContractError> {
    let current_loan_denom = LOAN_DENOM.load(deps.storage)?;
    let available_loan_denom = match current_loan_denom {
        AssetInfo::NativeToken { denom } => deps.querier.query_balance(env.contract.address, denom)?,
        AssetInfo::Token { .. } => {
            return Err(ContractError::IncorrectToken {  });
        }
    };
    let validated_expected_amount: Uint128 = Uint128::try_from(amount.as_str())?;
    if validated_expected_amount != available_loan_denom.amount {
        return Err(ContractError::LoanNotReturned { expected: amount, available: available_loan_denom.amount.to_string() });
    }
    Ok(Response::new()
        .add_attribute("method", "try assert balance"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {  } => to_binary(&try_get_config(deps)?),
        QueryMsg::GetBalance {  } => to_binary(&try_get_balance(deps)?),
    }
}

fn try_get_config(deps: Deps) -> StdResult<ConfigResponse> {
    let current_owner = OWNER.load(deps.storage)?;
    let current_fee = FEE.load(deps.storage)?;
    let current_loan_denom = LOAN_DENOM.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: current_owner,
        fee: current_fee,
        loan_denom: current_loan_denom
    })
}

fn try_get_balance(deps: Deps) -> StdResult<Uint128> {
    let current_balance = BALANCE.load(deps.storage)?;
    Ok(current_balance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { fee: "0.1".to_string(), loan_denom: "uluna".to_string() };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
