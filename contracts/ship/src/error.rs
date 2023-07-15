use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

use cw721_base::ContractError as Cw721ContractError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdErr(#[from] StdError),

    #[error("Unauthorized")]
    UnauthorizedErr {},

    #[error("Operation not implemented")]
    UnimplementedErr {},

    #[error("token_id already claimed")]
    ClaimedErr {},

    #[error("Expired")]
    ExpiredErr {},

    #[error("NotFound")]
    NotFoundErr {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFoundErr { spender: String },

    #[error("{0}")]
    PaymentErr(#[from] PaymentError),

    #[error("Invalid reply {id}")]
    UnrecognizedReplyErr { id: u64 },
}

impl TryFrom<Cw721ContractError> for ContractError {
    type Error = ContractError;

    fn try_from(err: Cw721ContractError) -> Result<Self, Self::Error> {
        use Cw721ContractError::*;

        match err {
            Unauthorized {} => Ok(ContractError::UnauthorizedErr {}),
            Claimed {} => Ok(ContractError::ClaimedErr {}),
            Expired {} => Ok(ContractError::ExpiredErr {}),
            ApprovalNotFound { spender } => Ok(ContractError::ApprovalNotFoundErr { spender }),
            _ => Err(ContractError::UnimplementedErr {}),
        }
    }
}
