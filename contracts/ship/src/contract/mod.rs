pub mod exec;
pub mod query;
pub mod reply;

use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;
use cw721_base::{Cw721Contract, InstantiateMsg, QueryMsg};

use crate::{contract::exec::BaseExecute, msg::ExecuteMsg, state::Extension, ContractError};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let sender = info.sender.to_string();

    let cw721_contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    cw721_contract.instantiate(deps, env, info, msg)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("sender", sender))
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    let contract = Cw721Contract::default();

    match msg {
        // SetMinter { minter } => exec::set_minter(deps, info, minter),
        // msg to load  cw20-helper token data on nft
        LoadFreight {
            token_id,
            denom,
            amount,
            unit_weight,
        } => exec::load_freight(deps, token_id, denom, amount, unit_weight),
        // msg to unload cw20 helper token data on nft
        UnloadFreight {
            token_id,
            denom,
            amount,
        } => exec::unload_freight(deps, token_id, denom, amount),
        // msg to decrease health when playing games
        DecreaseHealth { token_id, value } => {
            exec::decrease_health(deps, env, info, token_id, value)
        }
        FuelUp { token_id, amount } => exec::fuel_up(deps, info, token_id, amount),
        BurnFuel { token_id, amount } => exec::burn_fuel(deps, info, token_id, amount),
        _ => contract.base_execute(deps, env, info, msg),
    }
}

pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Minter {} => to_binary(&query::minter(deps)?),
        OwnerOf {
            token_id,
            include_expired,
        } => to_binary(&query::owner_of(deps, env, token_id, include_expired)?),
        Approval {
            token_id,
            spender,
            include_expired,
        } => to_binary(&query::approval(
            deps,
            env,
            token_id,
            spender,
            include_expired,
        )?),
        Approvals {
            token_id,
            include_expired,
        } => to_binary(&query::approvals(deps, env, token_id, include_expired)?),
        NftInfo { token_id } => to_binary(&query::nft_info(deps, token_id)?),
        AllNftInfo {
            token_id,
            include_expired,
        } => to_binary(&query::all_nft_info(deps, env, token_id, include_expired)?),
        NumTokens {} => to_binary(&query::num_tokens(deps)?),
        Tokens {
            owner,
            start_after,
            limit,
        } => to_binary(&query::tokens(deps, owner, start_after, limit)?),
        ContractInfo {} => to_binary(&query::contract_info(deps)?),

        _ => StdResult::Ok(Default::default()),
    }
}

pub fn reply(_deps: DepsMut, _env: Env, _reply: Reply) -> Result<Response, ContractError> {
    todo!()
}
