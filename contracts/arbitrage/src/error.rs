use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Token balance for {token:?} is too small. Expected {expected:?}, but only have {available:?}")]
    BalanceTooSmall { token: String, available: String, expected: String },

    #[error("Arbitrage not successful. Current amount {current:?}, previous amount {previous:?}")]
    ArbitrageUnsuccessful { current: String, previous: String },

    #[error("Swap amount for token {asset:?} is too small {amount:?}. Expected > 0.")]
    InvalidSwapAmount { amount: String, asset: String },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
