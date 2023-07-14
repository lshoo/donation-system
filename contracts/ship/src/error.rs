use cosmwasm_std::{Decimal, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdErr(#[from] StdError),

    #[error("Unauthorized")]
    UnauthorizedErr {},

    #[error("Invalid direct part: {direct_part}")]
    InvalidDirectPartErr { direct_part: Decimal },

    #[error("{0}")]
    PaymentErr(#[from] PaymentError),

    #[error("Invalid reply {id}")]
    UnrecognizedReplyErr { id: u64 },
}
