pub mod exec;

pub mod reply;

use cosmwasm_std::{ensure, Addr, Decimal, DepsMut, Env, MessageInfo, Reply, Response};
use cw2::set_contract_version;

use crate::{
    msg::{ExecMsg, InstantiateMsg},
    
    ContractError,
};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


pub fn instantiate(
    deps: DepsMut,
    env: Env,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
   
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;


    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    use ExecMsg::*;

    Ok(Response::new())
}

pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    todo!()
}
