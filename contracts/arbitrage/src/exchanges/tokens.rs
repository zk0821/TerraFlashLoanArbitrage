use terraswap::querier::{ query_token_info, query_native_decimals };
use terraswap::asset::{ AssetInfo };

use cosmwasm_std::{ Deps, StdResult, Addr, StdError };

use cw20::{ TokenInfoResponse };

pub struct CustomAssetInfo<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub is_native: bool,
}

pub const NATIVE_TOKEN_LIST: [CustomAssetInfo; 1] = [
    CustomAssetInfo {
        id: "uluna",
        name: "LUNA",
        is_native: true,
    },
];

pub const TOKEN_LIST: [CustomAssetInfo; 1] = [
    CustomAssetInfo {
        id: "terra167dsqkh2alurx997wmycw9ydkyu54gyswe3ygmrs4lwume3vmwks8ruqnv",
        name: "ASTRO",
        is_native: false,
    }
];

pub fn convert_string_to_asset_info(deps: Deps, token: &str) -> StdResult<AssetInfo> {
    match check_token_address(deps, token) {
        Ok(..) => {
            return Ok(AssetInfo::Token {
                 contract_addr: token.to_string()
            });
        },
        Err(..) => {
            //Check native token on terraswap - astroport does not support this feature
            let _decimals = check_native_token(deps, "terra1jha5avc92uerwp9qzx3flvwnyxs3zax2rrm6jkcedy2qvzwd2k7qk7yxcl", token)?;
            for native_token in NATIVE_TOKEN_LIST {
                if native_token.id == token {
                    return Ok(AssetInfo::NativeToken {
                         denom: token.to_string()
                    });
                }
            }
            return Err(StdError::generic_err("Token is not supported!"));
        }
    }
}

pub fn check_token_address(deps: Deps, contract_addr: &str) -> StdResult<Addr> {
    let valid_contract_addr: Addr = deps.api.addr_validate(&contract_addr)?;
    let _token_info: TokenInfoResponse = query_token_info(&deps.querier, valid_contract_addr)?;
    for token in TOKEN_LIST {
        if token.id == contract_addr {
           return Ok(deps.api.addr_validate(contract_addr)?);
        }
    }
    Err(StdError::generic_err("Token is not supported!"))
}

pub fn check_native_token(deps: Deps, factory_address: &str, denom: &str) -> StdResult<u8> {
    let valid_factory_address = deps.api.addr_validate(factory_address)?;
    let native_token_decimals: u8 = query_native_decimals(&deps.querier, valid_factory_address, denom.to_string())?;
    Ok(native_token_decimals)
}