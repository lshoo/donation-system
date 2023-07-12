use cosmwasm_std::{coins, BankMsg, DepsMut, Env, Response, StdError, SubMsgResponse};

use crate::{
    state::{CONFIG, PENDING_WITHDRAWAL},
    ContractError,
};

pub fn withdraw(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let withdraw_info = PENDING_WITHDRAWAL.load(deps.storage)?;

    let config = CONFIG.load(deps.storage)?;

    let total_amount = deps
        .querier
        .query_balance(env.contract.address, &config.denom)?;

    let amount = withdraw_info.amount.unwrap_or(total_amount.amount);

    let send_msg = BankMsg::Send {
        to_address: withdraw_info.receiver.into_string(),
        amount: coins(amount.u128(), &config.denom),
    };

    let resp = Response::new()
        .add_message(send_msg)
        .add_attribute("actoin", "reply_withdraw")
        .add_attribute("amount", amount.to_string());

    Ok(resp)
}

pub fn propose_member(reply: Result<SubMsgResponse, String>) -> Result<Response, ContractError> {
    let response = reply.map_err(StdError::generic_err)?;
    if let Some(data) = response.data {
        let resp = Response::new().set_data(data);
        Ok(resp)
    } else {
        Ok(Response::new())
    }
}
