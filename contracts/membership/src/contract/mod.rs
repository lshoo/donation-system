pub mod exec;
pub mod query;
pub mod reply;

use cosmwasm_std::{
    ensure, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
    SubMsg, WasmMsg,
};
use cw2::set_contract_version;

use crate::{
    msg::{ExecMsg, InstantiateMsg, QueryMsg},
    state::{Config, AWAITING_INITIAL_REPS, CONFIG},
    ContractError,
};

use proxy::msg::InstantiateMsg as ProxyInstantiateMsg;

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INITIAL_PROXY_INSTANTIATION_REPLY_ID: u64 = 1;
const PROXY_INSTANTIATION_REPLY_ID: u64 = 2;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ensure!(
        msg.minimal_acceptances >= 2,
        ContractError::NotEnoughRequiredAcceptancesErr {}
    );

    ensure!(
        msg.minimal_acceptances <= msg.initial_members.len() as u64,
        ContractError::NotEnoughInitialMembersErr {}
    );

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        starting_weight: msg.starting_weight,
        denom: msg.denom.clone(),
        direct_part: msg.direct_part,
        halftime: msg.halftime,
        proxy_code_id: msg.proxy_code_id,
        distribution_contract: Addr::unchecked(""),
        minimal_acceptance: msg.minimal_acceptances,
    };

    CONFIG.save(deps.storage, &config)?;

    let msgs: Vec<_> = msg
        .initial_members
        .into_iter()
        .map(|m| {
            let addr = deps.api.addr_validate(&m)?;
            let init_msg = ProxyInstantiateMsg {
                owner: addr.to_string(),
                weight: msg.starting_weight,
                denom: msg.denom.clone(),
                direct_part: msg.direct_part,
                distribution_contract: "".to_string(),
                membership_contract: env.contract.address.to_string(),
                halftime: msg.halftime,
            };

            let msg = WasmMsg::Instantiate {
                admin: Some(env.contract.address.to_string()),
                code_id: msg.proxy_code_id,
                msg: to_binary(&init_msg)?,
                funds: vec![],
                label: format!("{} proxy", addr),
            };

            let msg = SubMsg::reply_on_success(msg, INITIAL_PROXY_INSTANTIATION_REPLY_ID);
            Ok(msg)
        })
        .collect::<Result<_, ContractError>>()?;

    AWAITING_INITIAL_REPS.save(deps.storage, &(msgs.len() as _))?;

    // Instatiate the proxy
    let resp = Response::new().add_submessages(msgs);

    Ok(resp)
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    use ExecMsg::*;

    match msg {
        ProposeMember { candidate: addr } => exec::propose_member(deps, env, info, addr),
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        IsMember { addr } => query::is_member(deps, addr).and_then(|data| to_binary(&data)),
    }
}

pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    match reply.id {
        INITIAL_PROXY_INSTANTIATION_REPLY_ID => {
            reply::initial_proxy_instantiated(deps, reply.result.into_result())
        }
        PROXY_INSTANTIATION_REPLY_ID => reply::proxy_instantiated(deps, reply.result.into_result()),
        id => Err(ContractError::UnRecognizedReplyIdErr { id }),
    }
}
