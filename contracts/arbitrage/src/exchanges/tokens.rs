use terraswap::querier::{ query_token_info, query_native_decimals };
use terraswap::asset::{ AssetInfo };

use cosmwasm_std::{ Deps, StdResult, Addr, StdError };

use cw20::{ TokenInfoResponse };

use crate::state::{ FACTORIES, FactoryState };

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

pub const TOKEN_LIST: [CustomAssetInfo; 5] = [
    CustomAssetInfo {
        id: "terra14xsm2wzvu7xaf567r693vgfkhmvfs08l68h4tjj5wjgyn5ky8e2qvzyanh",
        name: "LunaX",
        is_native: false,
    },
    CustomAssetInfo {
        id: "terra1ee4g63c3sus9hnyyp3p2u3tulzdv5ag68l55q8ej64y4qpwswvus5mtag2",
        name: "LIRA",
        is_native: false,
    },
    CustomAssetInfo {
        id: "terra1xumzh893lfa7ak5qvpwmnle5m5xp47t3suwwa9s0ydqa8d8s5faqn6x7al",
        name: "STEAK",
        is_native: false,
    },
    CustomAssetInfo {
        id: "terra1cmf8ytutcwrjrv08zskj9phuh46a3w3nkjax7en4hxezsrdr58lqvzy05q",
        name: "Alem",
        is_native: false,
    },
    CustomAssetInfo {
        id: "terra1ecgazyd0waaj3g7l9cmy5gulhxkps2gmxu9ghducvuypjq68mq2s5lvsct",
        name: "ampLUNA",
        is_native: false,
    },

];

pub fn convert_string_to_asset_info(deps: Deps, token: &str) -> StdResult<AssetInfo> {
    let terraswap_factory: FactoryState = FACTORIES.load(deps.storage, "TERRASWAP".to_string())?;
    match check_token_address(deps, token) {
        Ok(..) => {
            return Ok(AssetInfo::Token {
                 contract_addr: token.to_string()
            });
        },
        Err(..) => {
            //Check native token on terraswap - astroport does not support this feature
            let _decimals = check_native_token(deps, &terraswap_factory.contract_addr, token)?;
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