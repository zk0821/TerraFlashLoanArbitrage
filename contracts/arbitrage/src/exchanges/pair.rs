use terraswap::asset::{ AssetInfo };

use schemars::JsonSchema;

use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct PairInformation {
    pub asset_infos: [AssetInfo; 2],
    pub contract_addr: String,
    pub liquidity_token: String,
}