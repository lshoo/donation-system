use cosmwasm_std::{Deps, Empty, Env, StdResult};
use cw721::*;
use cw721_base::{Cw721Contract, MinterResponse};

use crate::state::Extension;

pub fn minter(deps: Deps) -> StdResult<MinterResponse> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    contract.minter(deps)
}

pub fn owner_of(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: Option<bool>,
) -> StdResult<OwnerOfResponse> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    contract.owner_of(deps, env, token_id, include_expired.unwrap_or_default())
}

pub fn approval(
    deps: Deps,
    env: Env,
    token_id: String,
    spender: String,
    include_expired: Option<bool>,
) -> StdResult<ApprovalResponse> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    contract.approval(
        deps,
        env,
        token_id,
        spender,
        include_expired.unwrap_or_default(),
    )
}

pub fn approvals(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: Option<bool>,
) -> StdResult<ApprovalsResponse> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    contract.approvals(deps, env, token_id, include_expired.unwrap_or_default())
}

pub fn nft_info(deps: Deps, token_id: String) -> StdResult<NftInfoResponse<Extension>> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    contract.nft_info(deps, token_id)
}

pub fn all_nft_info(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: Option<bool>,
) -> StdResult<AllNftInfoResponse<Extension>> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    contract.all_nft_info(deps, env, token_id, include_expired.unwrap_or_default())
}

pub fn num_tokens(deps: Deps) -> StdResult<NumTokensResponse> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    contract.num_tokens(deps)
}

pub fn tokens(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    contract.tokens(deps, owner, start_after, limit)
}

pub fn contract_info(deps: Deps) -> StdResult<ContractInfoResponse> {
    let contract: Cw721Contract<Extension, Empty, Empty, Empty> = Cw721Contract::default();
    contract.contract_info(deps)
}
