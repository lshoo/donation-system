use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::ContractError;

pub fn propose_member(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _member: String,
) -> Result<Response, ContractError> {
    todo!()
}
