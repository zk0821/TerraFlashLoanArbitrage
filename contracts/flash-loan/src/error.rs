use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Incorrect funds provided: {msg:?}")]
    IncorrectFunds {msg: String},

    #[error("No CW20 Token support!")]
    IncorrectToken {},

    #[error("Requested amount is too large. Loan cannot be given!")]
    RequestedAmountTooLarge {},

    #[error("Loan has not been returned! Expected balance: {expected:?}, available balance: {available:?}")]
    LoanNotReturned { expected: String, available: String},

    #[error("Balance is empty!")]
    EmptyBalance {},

    #[error("Custom Error val: {val:?}")]
    CustomError{val: String},

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
