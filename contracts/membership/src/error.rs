use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdErr(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Not enough initial members")]
    NotEnoughInitialMembersErr {},

    #[error("Not enough required acceptances")]
    NotEnoughRequiredAcceptancesErr {},
}
