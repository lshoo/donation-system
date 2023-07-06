use cosmwasm_std::{
    coins, ensure, to_binary, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg, Uint128,
    WasmMsg,
};
use cw_utils::must_pay;

use crate::{
    contract::WITHDRAW_REPLAY_ID,
    msg::{DistributionExecMsg, MembershipExecMsg},
    state::{
        WithdrawData, CONFIG, DONATIONS, HALFTIME, LAST_UPDATED, OWNER, PENDING_WITHDRAWAL, WEIGHT,
    },
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
        msg: to_binary(&distribute_msg)?,
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
    env: Env,
    info: MessageInfo,
    receiver: Option<String>,
    amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    ensure!(owner == info.sender, ContractError::UnauthorizedErr {});

    let weight = WEIGHT.load(deps.storage)?;
    let donation = DONATIONS.load(deps.storage)?;
    let diff = donation as i64 - weight as i64;
    WEIGHT.save(deps.storage, &donation)?;
    DONATIONS.save(deps.storage, &1)?;
    LAST_UPDATED.save(deps.storage, &env.block.time.seconds())?;

    let receiver = receiver
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    PENDING_WITHDRAWAL.save(deps.storage, &WithdrawData { receiver, amount })?;

    let config = CONFIG.load(deps.storage)?;

    let withdraw_msg = DistributionExecMsg::Withdraw { weight, diff };
    let withdraw_msg = WasmMsg::Execute {
        contract_addr: config.distribution_contract.into_string(),
        msg: to_binary(&withdraw_msg)?,
        funds: vec![],
    };

    let withdraw_msg = SubMsg::reply_on_success(withdraw_msg, WITHDRAW_REPLAY_ID);

    let resp = Response::new()
        .add_submessage(withdraw_msg)
        .add_attribute("action", "withdraw")
        .add_attribute("sender", info.sender)
        .add_attribute("new weight", weight.to_string());

    Ok(resp)
}

pub fn close(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    ensure!(owner == info.sender, ContractError::UnauthorizedErr {});

    CONFIG.update(deps.storage, |mut c| -> StdResult<_> {
        c.is_closed = true;
        Ok(c)
    })?;

    Ok(Response::new().add_attribute("action", "close"))
}

pub fn proposer_member(deps: DepsMut, addr: String) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    ensure!(owner == addr, ContractError::UnauthorizedErr {});

    let config = CONFIG.load(deps.storage)?;

    let msg = MembershipExecMsg::ProposerMember { addr: addr.clone() };
    let membership_msg = WasmMsg::Execute {
        contract_addr: config.membership_contract.into_string(),
        msg: to_binary(&msg)?,
        funds: vec![],
    };

    let resp = Response::new()
        .add_message(membership_msg)
        .add_attribute("action", "proposer_member")
        .add_attribute("sender", owner.to_string())
        .add_attribute("member", addr);

    Ok(resp)
}

pub fn update_weight(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let last_updated = LAST_UPDATED.load(deps.storage)?;
    let halftime = HALFTIME.load(deps.storage)?;

    let resp = Response::new()
        .add_attribute("action", "update_weight")
        .add_attribute("sender", info.sender);

    let elapsed = env.block.time.seconds() - last_updated;
    if elapsed < halftime {
        let resp = resp.add_attribute("performed", "no");
        return Ok(resp);
    }

    let config = CONFIG.load(deps.storage)?;

    let weight = WEIGHT.load(deps.storage)?;
    let diff = -(weight as i64) / 2;

    let withdraw_msg = DistributionExecMsg::Withdraw { weight, diff };
    let withdraw_msg = WasmMsg::Execute {
        contract_addr: config.distribution_contract.into_string(),
        msg: to_binary(&withdraw_msg)?,
        funds: vec![],
    };

    let weight = (weight as i64 + diff) as u64;
    WEIGHT.save(deps.storage, &weight)?;

    let resp = resp
        .add_attribute("performed", "yes")
        .add_message(withdraw_msg)
        .add_attribute("new weight", weight.to_string());

    Ok(resp)
}
