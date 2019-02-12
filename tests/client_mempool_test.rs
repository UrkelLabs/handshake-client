mod common;

#[test]
fn test_getmempoolinfo() {
    let mut client = common::setup();

    let info = client.getmempoolinfo();

    assert!(info.is_ok());

}

// #[test]
// fn test_getmempoolancestors() {

// }
//
//TODO Finish tests that require a txhash in the mempool.
//
#[test]
fn test_getrawmempool() {
    let mut client = common::setup();

    let mempool = client.getrawmempool();

    assert!(mempool.is_ok());

}

#[test]
fn test_getrawmempool_verbose() {
    let mut client = common::setup();

    let mempool = client.getrawmempoolverbose();

    assert!(mempool.is_ok());

}

#[test]
fn test_estimatefee() {
    let mut client = common::setup();

    let fee = client.estimatefee(10);

    assert!(fee.is_ok());

}

#[test]
fn test_estimatepriority() {
    let mut client = common::setup();

    let priority = client.estimatepriority(10);

    assert!(priority.is_ok());
}

#[test]
fn test_estimatesmartfee() {
    let mut client = common::setup();

    let smartfee = client.estimatesmartfee(10);

    assert!(smartfee.is_ok());
}

#[test]
fn test_estimatesmartpriority() {
    let mut client = common::setup();

    let smartpriority = client.estimatesmartpriority(10);

    assert!(smartpriority.is_ok());

}
