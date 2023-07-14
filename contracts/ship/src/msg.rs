use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub weight: u64,
    pub denom: String,
    pub direct_part: Decimal,
    pub distribution_contract: String,
    pub membership_contract: String,
    pub halftime: u64,
}

#[cw_serde]
pub enum ExecMsg {
    Donate {},
    Withdraw {
        receiver: Option<String>,
        amount: Option<Uint128>,
    },
    Close {},
    ProposerMember {
        addr: String,
    },
    UpdateWeight {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

// Message for Distribution
#[cw_serde]
pub enum DistributionExecMsg {
    Distribute {},
    Withdraw { weight: u64, diff: i64 },
}

// Message for Membership
#[cw_serde]
pub enum MembershipExecMsg {
    ProposeMember { candidate: String },
}
