mod common;

use common::setup;

#[test]
fn test_get_mempool_info() {
    let client = setup();

    let info = client.get_mempool_info();

    assert!(info.is_ok());
}

// #[test]
// fn test_getmempoolancestors() {

// }
//
//TODO Finish tests that require a txhash in the mempool.
//
#[test]
fn test_get_raw_mempool() {
    let client = common::setup();

    let mempool = client.get_raw_mempool();

    assert!(mempool.is_ok());
}

#[test]
fn test_getrawmempool_verbose() {
    let client = common::setup();

    let mempool = client.get_raw_mempool_verbose();

    assert!(mempool.is_ok());
}

#[test]
fn test_estimate_fee() {
    let client = common::setup();

    let fee = client.estimate_fee(&10);

    assert!(fee.is_ok());
}

#[test]
fn test_estimate_priority() {
    let client = common::setup();

    let priority = client.estimate_priority(&10);

    assert!(priority.is_ok());
}

#[test]
fn test_estimate_smart_fee() {
    let client = common::setup();

    let smartfee = client.estimate_smart_fee(&10);

    assert!(smartfee.is_ok());
}

#[test]
fn test_estimate_smart_priority() {
    let client = common::setup();

    let smartpriority = client.estimate_smart_priority(&10);

    assert!(smartpriority.is_ok());
}
