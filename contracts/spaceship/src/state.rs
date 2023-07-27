use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

// q1) Declare extension as type of struct Metadata below
pub type Extension = Metadata;

#[cw_serde]
pub struct State {
    pub count: i32,
    pub owner: Addr,
    pub extension: Metadata,
}

#[cw_serde]
pub struct Metadata {
    pub unit_denom: String,
    pub price: Uint128,
    pub name: Option<String>,
    // q2) freights as vector of Freight?
    pub freights: Vec<Freight>,
    pub health: Uint128,
    pub fuel: Uint128,
}

#[cw_serde]
pub struct Freight {
    pub denom: String,
    pub amount: Uint128,
    pub unit_weight: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
