use cosmwasm_std::{ to_binary, Addr, Coin, Decimal, Deps, QueryRequest, StdResult, Uint128, WasmQuery, QuerierWrapper };

use terraswap::asset::{ Asset, AssetInfo, PairInfo };
use terraswap::pair::{ PoolResponse, QueryMsg, SimulationResponse };
use terraswap::factory::{ QueryMsg as FactoryQueryMsg };
use terraswap::querier::{ query_balance, query_token_balance };

use crate::exchanges::tokens::{ convert_string_to_asset_info };
use crate::exchanges::pair::{ PairInformation };

pub fn simulate_native_token_swap(deps: Deps, pool_address: Addr, offer_coin: Coin) -> StdResult<Uint128> {
    let response: SimulationResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: pool_address.to_string(),
        msg: to_binary(&QueryMsg::Simulation {
            offer_asset: Asset {
                info: AssetInfo::NativeToken { denom: offer_coin.denom },
                amount: offer_coin.amount
            }
        })?
    }))?;
    Ok(response.return_amount)
}

pub fn simulate_token_swap(deps: Deps, pool_address: Addr, offer_coin: Coin) -> StdResult<Uint128> {
    let response: SimulationResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: pool_address.to_string(),
        msg: to_binary(&QueryMsg::Simulation {
            offer_asset: Asset {
                info: AssetInfo::Token { contract_addr: offer_coin.denom },
                amount: offer_coin.amount
            }
        })?
    }))?;
    Ok(response.return_amount)
}

pub fn query_pool(deps: Deps, pool_address: Addr) -> StdResult<PoolResponse> {
    let response: PoolResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: pool_address.to_string(),
        msg: to_binary(&QueryMsg::Pool {})?
    }))?;
    Ok(response)
}

pub fn query_lp_token(deps: Deps, pool_address: Addr) -> StdResult<String> {
    let response: PairInfo = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: pool_address.to_string(),
        msg: to_binary(&QueryMsg::Pair {})?
    }))?;
    Ok(response.liquidity_token)
}

pub fn pool_ratio(deps: Deps, pool_address: Addr) -> StdResult<Decimal> {
    let response: PoolResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: pool_address.to_string(),
        msg: to_binary(&QueryMsg::Pool {})?
    }))?;
    let ratio = Decimal::from_ratio(response.assets[0].amount, response.assets[1].amount);
    Ok(ratio)
}

pub fn query_asset_balance(deps: Deps, asset_info: &AssetInfo, address: Addr) -> StdResult<Uint128> {
    let response: Uint128 = match asset_info.clone() {
        AssetInfo::NativeToken { denom } => query_balance(&deps.querier, address, denom)?,
        AssetInfo::Token { contract_addr } => query_token_balance(&deps.querier, deps.api.addr_validate(contract_addr.as_str())?, address)?,
    };
    Ok(response)
}

pub fn query_pool_address(deps: Deps, factory_addr: &str, offer_asset: &str, wanted_asset: &str) -> StdResult<Addr> {
    let valid_factory_addr = deps.api.addr_validate(&factory_addr)?;
    let validated_offer_asset: AssetInfo = convert_string_to_asset_info(deps, offer_asset)?;
    let validated_wanted_asset: AssetInfo = convert_string_to_asset_info(deps, wanted_asset)?;
    let pair:PairInformation = query_pair_information(deps.querier, valid_factory_addr,&[validated_offer_asset, validated_wanted_asset])?;
    let pool_contract_address = pair.contract_addr;
    let validated_pool_contract_address: Addr = deps.api.addr_validate(&pool_contract_address)?;
    Ok(validated_pool_contract_address)
}

pub fn query_pair_information(querier: QuerierWrapper, factory_addr: Addr, asset_infos: &[AssetInfo;2]) -> StdResult<PairInformation> {
    let result: PairInformation = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
         contract_addr: factory_addr.to_string(),
         msg: to_binary(&FactoryQueryMsg::Pair {
            asset_infos: asset_infos.clone()
         })?,
        }))?;
    Ok(result)
}