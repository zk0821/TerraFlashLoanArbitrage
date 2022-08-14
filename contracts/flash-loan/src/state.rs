use cosmwasm_std::{ Addr, Decimal, Uint128 };

use cw_storage_plus::{ Item };

use terraswap::asset::{ AssetInfo };

pub const OWNER: Item<Addr> = Item::new("owner");
pub const FEE: Item<Decimal> = Item::new("fee");
pub const LOAN_DENOM: Item<AssetInfo> = Item::new("loan_denom");
pub const BALANCE: Item<Uint128> = Item::new("balance");
