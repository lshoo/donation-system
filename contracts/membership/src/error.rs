use cosmwasm_std::{Addr, StdError};
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    StdErr(#[from] StdError),

    #[error("Unauthorized")]
    UnauthorizedErr {},

    #[error("Not enough initial members")]
    NotEnoughInitialMembersErr {},

    #[error("Not enough required acceptances")]
    NotEnoughRequiredAcceptancesErr {},

    #[error("Un recognized reply id {id}")]
    UnRecognizedReplyIdErr { id: u64 },

    #[error("Data missing")]
    DataMissingErr {},

    #[error("{0}")]
    ParseErr(#[from] ParseReplyError),

    #[error("Proxy {addr} is a member")]
    AlreadyAMemberErr { addr: Addr },

    #[error("Already voted")]
    AlreadyVotedErr {},
}
