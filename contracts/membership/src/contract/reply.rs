use common::msg::ProposeMemberData;
use cosmwasm_std::{
    to_binary, Addr, DepsMut, Empty, Order, Response, StdError, StdResult, SubMsgResponse,
};
use cw_utils::parse_instantiate_response_data;

use crate::{
    msg::InstantiationData,
    state::{AWAITING_INITIAL_REPS, MEMBERS},
    ContractError,
};

pub fn initial_proxy_instantiated(
    deps: DepsMut,
    reply: Result<SubMsgResponse, String>,
) -> Result<Response, ContractError> {
    println!("replying proxy instantiated....");

    let response = reply.map_err(StdError::generic_err)?;
    let data = response.data.ok_or(ContractError::DataMissingErr {})?;
    let response = parse_instantiate_response_data(&data)?;

    let addr = Addr::unchecked(response.contract_address);

    MEMBERS.save(deps.storage, &addr, &Empty {})?;

    let awaiting = AWAITING_INITIAL_REPS.load(deps.storage)? - 1;

    if awaiting > 0 {
        AWAITING_INITIAL_REPS.save(deps.storage, &awaiting)?; // TODO check data is 0?

        let resp = Response::new().add_attribute("proxy_addr", &addr);

        return Ok(resp);
    }

    let members: Vec<_> = MEMBERS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|m| {
            let (member, _) = m?;
            let owner = proxy::state::OWNER.query(&deps.querier, member.clone())?;
            let data = ProposeMemberData {
                owner_addr: owner.into(),
                proxy_addr: member.into(),
            };

            Ok(data)
        })
        .collect::<StdResult<_>>()?;

    let data = InstantiationData { members };

    let resp = Response::new()
        .add_attribute("proxy_addr", addr)
        .set_data(to_binary(&data)?);

    Ok(resp)
}
