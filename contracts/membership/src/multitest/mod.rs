#[cfg(test)]
mod tests;

use common::msg::ProposeMemberData;
use cosmwasm_std::{from_binary, to_binary, Addr, Decimal, WasmMsg};
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};

use anyhow::Result as AnyResult;
use cw_utils::{parse_execute_response_data, parse_instantiate_response_data};
use std::convert::Into;

use crate::{msg::*, *};

#[derive(Clone, Debug, Copy)]
pub struct MembershipId(u64);

impl MembershipId {
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
        starting_weight: u64,
        denom: &str,
        direct_part: Decimal,
        halftime: u64,
        minimal_acceptances: u64,
        proxy_code_id: proxy::multitest::ProxyCodeId,
        // distribution_code_id: distribution::multitest::CodeId,
        initial_members: &[&str],
        label: &str,
    ) -> AnyResult<(MembershipContract, InstantiationData)> {
        MembershipContract::instantiate(
            app,
            self,
            sender,
            starting_weight,
            denom,
            direct_part,
            halftime,
            minimal_acceptances,
            proxy_code_id,
            initial_members,
            label,
        )
    }
}

impl From<MembershipId> for u64 {
    fn from(code_id: MembershipId) -> Self {
        code_id.0
    }
}

#[derive(Debug, Clone)]
pub struct MembershipContract(Addr);

// implement the contract real function, e.g. instantiate, functions in exec, query modules
impl MembershipContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[allow(clippy::too_many_arguments)]
    #[track_caller]
    pub fn instantiate(
        app: &mut App,
        code_id: MembershipId,
        sender: Addr,
        starting_weight: u64,
        denom: &str,
        direct_part: Decimal,
        halftime: u64,
        minimal_acceptances: u64,
        proxy_code_id: proxy::multitest::ProxyCodeId,
        // distribution_code_id: distribution::multitest::CodeId,
        initial_members: &[&str],
        label: &str,
    ) -> AnyResult<(Self, InstantiationData)> {
        let msg = InstantiateMsg {
            starting_weight,
            denom: denom.to_owned(),
            direct_part,
            halftime,
            minimal_acceptances,
            proxy_code_id: proxy_code_id.into(),
            distribution_code_id: 0,
            initial_members: initial_members.iter().map(|s| s.to_string()).collect(),
        };

        let msg = WasmMsg::Instantiate {
            admin: None,
            code_id: code_id.0,
            msg: to_binary(&msg)?,
            funds: vec![],
            label: label.into(),
        };

        let res = app.execute(sender, msg.into())?;
        let data = parse_instantiate_response_data(res.data.unwrap_or_default().as_slice())?;

        let contract = Self(Addr::unchecked(data.contract_address));
        let data = from_binary(&data.data.unwrap_or_default())?;

        Ok((contract, data))
    }

    #[track_caller]
    pub fn propose_member(
        &self,
        app: &mut App,
        sender: Addr,
        candidate: Addr,
    ) -> AnyResult<Option<ProposeMemberData>> {
        let msg = ExecMsg::ProposeMember {
            candidate: candidate.to_string(),
        };

        let resp = app.execute_contract(sender, self.addr(), &msg, &[])?;

        resp.data
            .map(|data| parse_execute_response_data(&data))
            .transpose()?
            .and_then(|data| data.data)
            .map(|data| from_binary(&data))
            .transpose()
            .map_err(Into::into)
    }

    pub fn is_member(&self, app: &App, addr: &str) -> AnyResult<IsMemberResp> {
        let query = QueryMsg::IsMember {
            addr: addr.to_owned(),
        };

        app.wrap()
            .query_wasm_smart(self.addr(), &query)
            .map_err(Into::into)
    }
}

impl From<Addr> for MembershipContract {
    fn from(value: Addr) -> Self {
        Self(value)
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
