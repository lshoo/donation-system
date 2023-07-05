use cosmwasm_std::{
    coins, to_binary, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg,
};
use cw_utils::must_pay;

use crate::{
    msg::DistributionExecMsg,
    state::{CONFIG, DONATIONS},
    ContractError,
};

pub fn donate(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let amount = must_pay(&info, &config.denom)?;

    let direct_amount = config.direct_part * amount;
    let to_distribute = amount - direct_amount;

    let distribute_msg = DistributionExecMsg::Distribute {};
    let distribute_msg = WasmMsg::Execute {
        contract_addr: config.distribution_contract.into_string(),
        msg: to_binary(&to_distribute)?,
        funds: coins(to_distribute.u128(), &config.denom),
    };

    let resp = Response::new()
        .add_message(distribute_msg)
        .add_attribute("action", "donate")
        .add_attribute("sender", info.sender)
        .add_attribute("amount", to_distribute.to_string());

    DONATIONS.update(deps.storage, |donations| -> StdResult<_> {
        Ok(donations + 1)
    })?;

    Ok(resp)
}

pub fn withdraw(
    deps: DepsMut,
    receiver: Option<String>,
    amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    unimplemented!()
}

pub fn close(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    unimplemented!()
}

pub fn proposer_member(deps: DepsMut, addr: String) -> Result<Response, ContractError> {
    unimplemented!()
}

pub fn update_weight(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    unimplemented!()
}
