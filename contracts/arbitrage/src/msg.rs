use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub astroport_factory_address: String,
    pub terraswap_factory_address: String,
    pub flash_loan_contract_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ExecuteSwap { factory_addr: String, offer_asset: String, offer_amount: String, wanted_asset: String },
    ExecuteSwapFullAmount { factory_addr: String, offer_asset: String, wanted_asset: String },
    ProvideToFlashLoan { offer_asset: String, offer_amount: String },
    WithdrawFromFlashLoan {},
    Withdraw { denom: String },
    ExecuteArbitrage {},
    ReceiveLoan {},
    AssertBalance { amount: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryBalances {},
    QueryPool { factory_addr: String, offer_asset: String, wanted_asset: String },
    SimulateSwap { factory_addr: String, offer_asset: String, amount: String, wanted_asset: String },
    EstimateArbitrage {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ArbitrageResponse {
    pub native_token: String,
    pub token: String,
    pub starting_exchange: String,
    pub starting_exchange_factory: String,
    pub ending_exchange: String,
    pub ending_exchange_factory: String,
    pub optimal_starting_amount: Uint128,
    pub calculated_profit: Uint128,
    pub simulated_profit: Uint128
}