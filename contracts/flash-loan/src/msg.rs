use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{ Addr, Decimal };

use terraswap::asset::{ AssetInfo };

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub fee: String,
    pub loan_denom: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig { owner: String, fee: String },
    Loan { amount: String },
    AssertBalance { amount: String },
    Provide { },
    Withdraw { },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig { },
    GetBalance { },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub owner: Addr,
    pub fee: Decimal,
    pub loan_denom: AssetInfo
}