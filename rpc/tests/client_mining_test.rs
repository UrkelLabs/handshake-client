mod common;

use common::setup;

#[test]
fn test_get_network_hashps() {
    let client = setup();

    let networkhashps = client.get_network_hashps(&120, &1);

    dbg!(&networkhashps);

    assert!(networkhashps.is_ok());
}

#[test]
fn test_getmininginfo() {
    let client = common::setup();

    let info = client.get_mining_info();

    assert!(info.is_ok());
}

#[test]
fn test_getwork() {
    let client = common::setup();

    let work = client.get_work();

    assert!(work.is_ok());
}
