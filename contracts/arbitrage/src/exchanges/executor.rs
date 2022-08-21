use cosmwasm_std::{ to_binary, coin, Deps, Addr, StdResult, WasmMsg, CosmosMsg, Decimal };

use cw20::Cw20ExecuteMsg;

use terraswap::asset::{ Asset, AssetInfo };
use terraswap::pair::{ Cw20HookMsg, ExecuteMsg };

use crate::exchanges::querier::{ query_pair_information };
use crate::state::{ FACTORIES, FactoryState };


pub fn swap_message(deps: Deps, factory_addr: Addr, offer_asset: Asset, wanted_asset: AssetInfo) -> StdResult<CosmosMsg> {
    let pool_address: String = query_pair_information(deps.querier, factory_addr.clone(), &[offer_asset.info.clone(), wanted_asset])?.contract_addr;

    let astroport_factory: FactoryState = FACTORIES.load(deps.storage, "ASTROPORT".to_string())?;
    let terraswap_factory: FactoryState = FACTORIES.load(deps.storage, "TERRASWAP".to_string())?;

    let mut max_spread: Option<Decimal> = None;

    if factory_addr.clone().to_string() == astroport_factory.contract_addr {
        max_spread = Some(Decimal::percent(50));
    } else {
        max_spread = Some(Decimal::percent(100));
    }

    match offer_asset.info.clone() {
        AssetInfo::Token { contract_addr } => {
            let message = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: contract_addr.to_string(),
                funds: vec![],
                msg: to_binary(&Cw20ExecuteMsg::Send {
                    contract: pool_address,
                    amount: offer_asset.amount,
                    msg: to_binary(&Cw20HookMsg::Swap {
                        max_spread: max_spread,
                        belief_price: None,
                        to: None
                    })?
                })?
            });
            Ok(message)
        }
        AssetInfo::NativeToken { denom } => {
            let message = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: pool_address,
                msg: to_binary(&ExecuteMsg::Swap {
                    offer_asset: offer_asset.clone(), 
                    belief_price: None,
                    max_spread: max_spread,
                    to: None
                })?,
                funds: vec![coin(offer_asset.amount.u128(), denom)] 
            });
            Ok(message)
        }
    }
}