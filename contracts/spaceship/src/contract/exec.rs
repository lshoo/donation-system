use cosmwasm_std::{
    attr, Addr, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, Uint128,
};
use cw721_base::{state::TokenInfo, Cw721Contract};
// use cw_utils::must_pay;

use crate::{
    msg::ExecuteMsg,
    state::{Extension, Freight},
    ContractError,
};

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
    env: &Env,
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

// pub fn set_minter(
//     deps: DepsMut,
//     info: MessageInfo,
//     minter: String,
// ) -> Result<Response, ContractError> {
//     let minter_addr = deps.api.addr_validate(&minter)?;
//     let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();

//     let minter = contract.minter(deps.as_ref())?.minter;

//     if minter == Some(info.sender.to_string()) {
//         contract.tokens.s;
//     } else {
//         return Err(ContractError::UnauthorizedErr {});
//     }
//     // if contract.minter(deps.as_ref())?.minter == info.sender {
//     //     contract.minter.save(deps.storage, &minter_addr)?;
//     // } else {
//     //     return Err(ContractError::UnauthorizedErr {});
//     // }

//     Ok(Response::new()
//         .add_attribute("action", "set_minter")
//         .add_attribute("sender", info.sender.to_string())
//         .add_attribute("minter", minter))
// }

pub fn load_freight(
    deps: DepsMut,
    token_id: String,
    denom: String,
    amount: Uint128,
    unit_weight: Uint128,
) -> Result<Response, ContractError> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    let mut token = contract.tokens.load(deps.storage, &token_id)?;
    let mut extension = token.extension;

    // q7) Declare candidated_idx iterating extension.freights whose denom is same with parameter's denom
    let candidate_idx = extension.freights.iter().position(|f| f.denom == denom);

    if let Some(idx) = candidate_idx {
        // update token amount
        extension.freights[idx].amount =
            extension.freights[idx].amount.checked_add(amount).unwrap();
    } else {
        // if not, push a new freight data
        extension.freights.push(Freight {
            denom: denom.clone(),
            amount,
            unit_weight,
        })
    }

    token.extension = extension;
    contract.tokens.save(deps.storage, &token_id, &token)?;

    Ok(Response::new()
        .add_attribute("action", "load_freight")
        .add_attribute("token_id", token_id)
        .add_attribute("freight", denom)
        .add_attribute("amount", amount.to_string())
        .add_attribute("unit_weight", unit_weight.to_string()))
}

pub fn unload_freight(
    deps: DepsMut,
    token_id: String,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    let mut token = contract.tokens.load(deps.storage, &token_id)?;
    let mut extension = token.extension;

    let candidate_idx = extension.freights.iter().position(|f| f.denom == denom);

    if let Some(idx) = candidate_idx {
        // q8) Remove freight from extension.freights if result amount is zero.
        // else, just decrease amount.
        if let Ok(sub) = extension.freights[idx].amount.checked_sub(amount) {
            extension.freights[idx].amount = sub;
        } else {
            extension.freights.remove(idx);
        }
    }

    token.extension = extension;

    contract.tokens.save(deps.storage, &token_id, &token)?;

    Ok(Response::new()
        .add_attribute("action", "unload_freight")
        .add_attribute("token_id", token_id)
        .add_attribute("freight", denom)
        .add_attribute("amount", amount.to_string()))
}

pub fn decrease_health(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    value: Uint128,
) -> Result<Response, ContractError> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    let mut token = contract.tokens.load(deps.storage, &token_id)?;

    check_can_send(&contract, deps.as_ref(), &env, &info.sender, &token)?;

    let mut extension = token.extension;

    // handle with negative overflow
    extension.health = extension.health.saturating_sub(value);
    token.extension = extension;
    contract.tokens.save(deps.storage, &token_id, &token)?;

    Ok(Response::new()
        .add_attribute("action", "decrease_health")
        .add_attribute("token_id", token_id)
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("value", value.to_string()))
}

pub fn fuel_up(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();

    let minter = contract.minter(deps.as_ref())?.minter;

    if minter != Some(info.sender.to_string()) {
        return Err(ContractError::UnauthorizedErr {});
    }

    let mut token = contract.tokens.load(deps.storage, &token_id)?;
    let mut extension = token.extension;

    extension.fuel = extension
        .fuel
        .checked_add(amount)
        .map_err(StdError::overflow)?;

    token.extension = extension;
    contract.tokens.save(deps.storage, &token_id, &token)?;

    Ok(Response::new().add_attributes([
        attr("action", "fuel_up"),
        attr("to", token_id),
        attr("amount", amount.to_string()),
    ]))
}

pub fn burn_fuel(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();

    let minter = contract.minter(deps.as_ref())?.minter;

    if minter != Some(info.sender.to_string()) {
        return Err(ContractError::UnauthorizedErr {});
    }

    let mut token = contract.tokens.load(deps.storage, &token_id)?;
    let mut extension = token.extension;

    extension.fuel = extension
        .fuel
        .checked_sub(amount)
        .map_err(StdError::overflow)?;

    token.extension = extension;
    contract.tokens.save(deps.storage, &token_id, &token)?;

    Ok(Response::new().add_attributes([
        attr("action", "burn_fuel"),
        attr("to", token_id),
        attr("amount", amount.to_string()),
    ]))
}
