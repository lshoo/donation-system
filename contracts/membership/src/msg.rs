use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    pub starting_weight: u64,
    pub denom: String,
    pub direct_part: Decimal,
    pub halftime: u64,
    pub minimal_acceptance: u64,
    pub proxy_code_id: u64,
    pub distribution_code_id: u64,
    pub initial_members: Vec<String>,
}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub struct InstantiationData {
    pub members: Vec<ProposeMemberData>,
}

#[cw_serde]
pub struct ProposeMemberData {
    pub owner_addr: Addr,
    pub proxy_addr: Addr,
}