use cosmwasm_std::{Binary, Deps, Env, MessageInfo, StdResult, Uint128};

pub fn minter(deps: Deps) -> StdResult<Binary> {
    todo!()
}

pub fn owner_of(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: Option<bool>,
) -> StdResult<Binary> {
    todo!()
}

pub fn approval(
    deps: Deps,
    env: Env,
    token_id: String,
    spender: String,
    include_expired: Option<bool>,
) -> StdResult<Binary> {
    todo!()
}

pub fn approvals(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: Option<bool>,
) -> StdResult<Binary> {
    todo!()
}

pub fn nft_info(deps: Deps, token_id: String) -> StdResult<Binary> {
    todo!()
}

pub fn all_nft_info(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: Option<bool>,
) -> StdResult<Binary> {
    todo!()
}

pub fn num_tokens(deps: Deps) -> StdResult<Binary> {
    todo!()
}

pub fn tokens(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Binary> {
    todo!()
}

pub fn contract_info(deps: Deps) -> StdResult<Binary> {
    todo!()
}
