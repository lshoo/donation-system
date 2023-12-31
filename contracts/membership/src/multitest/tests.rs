use std::collections::HashMap;

use cosmwasm_std::{Addr, Decimal};
use cw_multi_test::App;
use proxy::multitest::{ProxyCodeId, ProxyContract};

use super::{owner, MembershipId};

#[test]
fn add_membership_should_works() {
    let mut app = App::default();

    let denom = "star";

    let members = ["abcef", "cdefg"];
    let candidate = "candidate";

    let proxy_id = ProxyCodeId::store_code(&mut app);
    let membership_id = MembershipId::store_code(&mut app);

    let (membership, data) = membership_id
        .instantiate(
            &mut app,
            owner(),
            10,
            denom,
            Decimal::percent(15),
            3600 * 24 * 30,
            2,
            proxy_id,
            &members,
            "Membership",
        )
        .unwrap();

    let mut proxies: HashMap<String, ProxyContract> = HashMap::new();

    for member in data.members {
        proxies.insert(member.owner_addr, Addr::unchecked(member.proxy_addr).into());
    }

    assert_eq!(proxies.len(), 2);
    assert!(
        membership
            .is_member(&app, proxies[members[0]].addr().as_str())
            .unwrap()
            .is_member
    );
    assert!(
        membership
            .is_member(&app, proxies[members[1]].addr().as_str())
            .unwrap()
            .is_member
    );

    let data = proxies[members[0]]
        .propose_member(&mut app, members[0], candidate)
        .unwrap();

    assert!(data.is_none());

    let _data = proxies[members[1]]
        .propose_member(&mut app, members[1], candidate)
        .unwrap();

    // let data = data.unwrap();

    // assert_eq!(data.owner_addr, candidate);

    // assert!(
    //     membership
    //         .is_member(&app, data.proxy_addr.as_str())
    //         .unwrap()
    //         .is_member
    // )
}
