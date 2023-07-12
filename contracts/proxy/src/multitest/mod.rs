#[cfg(test)]
mod tests;

use common::msg::ProposeMemberData;
use cosmwasm_std::{from_binary, Addr, Coin, Decimal};
use cw_multi_test::{App, ContractWrapper, Executor};

use anyhow::Result as AnyResult;
use cw_utils::parse_execute_response_data;

use crate::{msg::*, *};

#[derive(Clone, Debug, Copy)]
pub struct ProxyCodeId(u64);

impl ProxyCodeId {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
        let code_id = app.store_code(Box::new(contract));
        Self(code_id)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn instantiate(
        self,
        app: &mut App,
        sender: Addr,
        owner: &str,
        weight: u64,
        denom: &str,
        direct_part: Decimal,
        distribution_contract: &str,
        membership_contract: &str,
        halftime: u64,
        label: &str,
    ) -> AnyResult<ProxyContract> {
        ProxyContract::instantiate(
            app,
            self,
            sender,
            owner,
            weight,
            denom,
            direct_part,
            distribution_contract,
            membership_contract,
            halftime,
            label,
        )
    }
}

impl From<ProxyCodeId> for u64 {
    fn from(code_id: ProxyCodeId) -> Self {
        code_id.0
    }
}

#[derive(Debug, Clone)]
pub struct ProxyContract(Addr);

// implement the contract real function, e.g. instantiate, functions in exec, query modules
impl ProxyContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: ProxyCodeId,
        sender: Addr,
        owner: &str,
        weight: u64,
        denom: &str,
        direct_part: Decimal,
        distribution_contract: &str,
        membership_contract: &str,
        halftime: u64,
        label: &str,
    ) -> AnyResult<Self> {
        let msg = InstantiateMsg {
            owner: owner.to_owned(),
            weight,
            denom: denom.to_owned(),
            direct_part,
            distribution_contract: distribution_contract.to_owned(),
            membership_contract: membership_contract.to_owned(),
            halftime,
        };

        app.instantiate_contract(code_id.0, sender, &msg, &[], label, None)
            .map(Self)
    }

    #[track_caller]
    pub fn donate(&self, app: &mut App, sender: Addr, funds: &[Coin]) -> AnyResult<()> {
        let msg = ExecMsg::Donate {};
        app.execute_contract(sender, self.addr(), &msg, funds)?;

        Ok(())
    }

    #[track_caller]
    pub fn close(&self, app: &mut App, sender: Addr) -> AnyResult<()> {
        let msg = ExecMsg::Close {};

        app.execute_contract(sender, self.addr(), &msg, &[])?;

        Ok(())
    }

    #[track_caller]
    pub fn update_weight(&self, app: &mut App, sender: Addr) -> AnyResult<()> {
        let msg = ExecMsg::UpdateWeight {};

        app.execute_contract(sender, self.0.clone(), &msg, &[])?;
        Ok(())
    }

    #[track_caller]
    pub fn propose_member(
        &self,
        app: &mut App,
        sender: &str,
        addr: &str,
    ) -> AnyResult<Option<ProposeMemberData>> {
        let msg = ExecMsg::ProposerMember {
            addr: addr.to_owned(),
        };

        let resp = app.execute_contract(Addr::unchecked(sender), self.addr(), &msg, &[])?;

        println!("the propse member result is: {:?}", resp);

        resp.data
            .map(|data| parse_execute_response_data(&data))
            .transpose()?
            .and_then(|data| data.data)
            .map(|data| from_binary(&data))
            .transpose()
            .map_err(Into::into)
    }
}

impl From<Addr> for ProxyContract {
    fn from(value: Addr) -> Self {
        Self(value)
    }
}

impl From<ProxyContract> for Addr {
    fn from(value: ProxyContract) -> Self {
        value.0
    }
}

pub fn alice() -> Addr {
    Addr::unchecked("sei18rszd3tmgpjvjwq2qajtmn5jqvtscd2yuygl4z")
}

pub fn bob() -> Addr {
    Addr::unchecked("sei1aan9kqywf4rf274cal0hj6eyly6wu0uv7edxy2")
}

pub fn owner() -> Addr {
    Addr::unchecked("sei1zj6fjsc2gkce878ukzg6g9wy8cl8p554dlggxd")
}

pub fn parent() -> Addr {
    Addr::unchecked("inj1g9v8suckezwx93zypckd4xg03r26h6ejlmsptz")
}
