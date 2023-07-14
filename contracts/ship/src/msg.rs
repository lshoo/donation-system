use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Decimal, Empty, Uint128};
use cw721_base::{ExecuteMsg as Cw721ExecuteMsg, MintMsg};
use cw_utils::Expiration;

use crate::{
    state::{Extension, Metadata},
    ContractError,
};

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
pub enum ExecuteMsg {
    TransferNft {
        recipient: String,
        token_id: String,
    },
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    Revoke {
        spender: String,
        token_id: String,
    },
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    RevokeAll {
        operator: String,
    },
    // q3) message "Mint" which extends MintMsg of cw721_base with Extension type declared at state.rs
    Mint(MintMsg<Extension>),
    Burn {
        token_id: String,
    },
    SetMinter {
        minter: String,
    },
    LoadFreight {
        token_id: String,
        denom: String,
        amount: Uint128,
        unit_weight: Uint128,
    },
    FuelUp {
        token_id: String,
        amount: Uint128,
    },
    BurnFuel {
        token_id: String,
        amount: Uint128,
    },
    UnloadFreight {
        token_id: String,
        denom: String,
        amount: Uint128,
    },
    DecreaseHealth {
        token_id: String,
        value: Uint128,
    },
}

impl TryFrom<ExecuteMsg> for Cw721ExecuteMsg<Metadata, Empty> {
    type Error = ContractError;

    fn try_from(msg: ExecuteMsg) -> Result<Self, Self::Error> {
        use ExecuteMsg::*;

        match msg {
            // q4) Convert ExecuteMsg::TransferNft to Cw721ExecuteMsg::TransferNft
            TransferNft {
                recipient,
                token_id,
            } => Ok(Cw721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            }),
            Mint(mint_msg) => Ok(Cw721ExecuteMsg::Mint(mint_msg)),
            SendNft {
                contract,
                token_id,
                msg,
            } => Ok(Cw721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            }),
            Approve {
                spender,
                token_id,
                expires,
            } => Ok(Cw721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            }),
            Revoke { spender, token_id } => Ok(Cw721ExecuteMsg::Revoke { spender, token_id }),
            Burn { token_id } => Ok(Cw721ExecuteMsg::Burn { token_id }),
            ApproveAll { operator, expires } => {
                Ok(Cw721ExecuteMsg::ApproveAll { operator, expires })
            }
            RevokeAll { operator } => Ok(Cw721ExecuteMsg::RevokeAll { operator }),
            _ => Err(ContractError::UnimplementedErr {}),
        }
    }
}

// #[cw_serde]
// #[derive(QueryResponses)]
// pub enum QueryMsg {}
