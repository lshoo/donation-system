use cosmwasm_std::{Addr, Deps, DepsMut, Empty, Env, MessageInfo, Response, Uint128};
use cw721_base::{state::TokenInfo, Cw721Contract};
// use cw_utils::must_pay;

use crate::{msg::ExecuteMsg, state::Extension, ContractError};

pub trait BaseExecute {
    fn base_execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError>;
}

impl<'a> BaseExecute for Cw721Contract<'a, Extension, Empty, Empty, Empty> {
    fn base_execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let cw721_msg = msg.try_into()?;

        let execute_res = self.execute(deps, env, info, cw721_msg);

        match execute_res {
            Ok(res) => Ok(res),
            Err(err) => Err(ContractError::try_from(err)?),
        }
    }
}

pub fn check_can_send(
    contract: &Cw721Contract<Extension, Empty, Empty, Empty>,
    deps: Deps,
    env: Env,
    sender: &Addr,
    token: &TokenInfo<Extension>,
) -> Result<(), ContractError> {
    if token.owner == sender.as_ref() {
        return Ok(());
    }

    if token
        .approvals
        .iter()
        .any(|approval| approval.spender == sender.as_ref() && !approval.is_expired(&env.block))
    {
        return Ok(());
    }

    let operators = contract
        .operators
        .may_load(deps.storage, (&token.owner, sender))?;

    match operators {
        Some(expiration) => {
            if expiration.is_expired(&env.block) {
                Err(ContractError::UnauthorizedErr {})
            } else {
                Ok(())
            }
        }
        None => Err(ContractError::UnauthorizedErr {}),
    }
}

pub fn set_minter(
    deps: DepsMut,
    info: MessageInfo,
    minter: String,
) -> Result<Response, ContractError> {
    todo!()
}

pub fn load_freight(
    deps: DepsMut,
    token_id: String,
    denom: String,
    amount: Uint128,
    unit_weight: Uint128,
) -> Result<Response, ContractError> {
    todo!()
}

pub fn unload_freight(
    deps: DepsMut,
    token_id: String,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    todo!()
}

pub fn decrease_health(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    value: Uint128,
) -> Result<Response, ContractError> {
    todo!()
}

pub fn fuel_up(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    todo!()
}

pub fn burn_fuel(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    todo!()
}
