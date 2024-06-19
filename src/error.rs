use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Stock already exists")]
    StockAlreadyExists,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("No balance")]
    NoBalance,

    #[error("Insufficient shares")]
    InsufficientShares,

    #[error("Order already exists")]
    OrderAlreadyExists,

    #[error("Invalid input provided")]
    InvalidInput,

    #[error("Internal error")]
    InternalError,

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Not enough shares")]
    NotEnoughShares,

    #[error("Stock Not Found")]
    StockNotFound,

    #[error("Already Instantiated")]
    AlreadyInstantiated,

    #[error("No Field")]
    NoField,
}
