use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{ Item, Map };

use crate::msg::ArbitrageResponse;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FactoryState {
    pub contract_addr: String
}

pub const FACTORIES: Map<String, FactoryState> = Map::new("factories");

pub const FLASH_LOAN: Item<String> = Item::new("flash_loan");

pub const ARBITRAGE: Item<ArbitrageResponse> = Item::new("arbitrage");