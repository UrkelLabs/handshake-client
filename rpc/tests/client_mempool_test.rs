mod common;

use common::setup;
use common::get_txid;

#[async_std::test]
async fn test_get_mempool_info() {
    let client = setup();

    let info = client.get_mempool_info().await;

    assert!(info.is_ok());
}

//@todo we need a way of creating a new tx, and then checking it in the mempool. Disabling for now.
// #[runtime::test]
// async fn test_get_mempool_ancestors() {
//     let client = setup();

//     let ancestors = client.get_mempool_ancestors(&tx_hash).await;

//     assert!(ancestors.is_ok());

// }

// #[runtime::test]
// async fn test_get_mempool_ancestors_verbose() {
//     let client = setup();

//     let ancestors = client.get_mempool_ancestors_verbose(&tx_hash).await;

//     assert!(ancestors.is_ok());

// }

// #[runtime::test]
// async fn test_get_mempool_descendants() {
//     let client = setup();

//     let ancestors = client.get_mempool_descendants(&tx_hash).await;

//     assert!(ancestors.is_ok());
// }

// #[runtime::test]
// async fn test_get_mempool_descendants_verbose() {
//     let client = setup();

//     let ancestors = client.get_mempool_ancestors_verbose(&tx_hash).await;

//     assert!(ancestors.is_ok());
// }

// #[runtime::test]
// async fn test_get_mempool_entry() {
//     let client = setup();

//     let ancestors = client.get_mempool_ancestors(&tx_hash).await;

//     assert!(ancestors.is_ok());

// }

#[async_std::test]
async fn test_get_raw_mempool() {
    let client = common::setup();

    let mempool = client.get_raw_mempool().await;

    assert!(mempool.is_ok());
}

#[async_std::test]
async fn test_get_raw_mempool_verbose() {
    let client = common::setup();

    let mempool = client.get_raw_mempool_verbose().await;

    assert!(mempool.is_ok());
}

//@todo prioritize transaction - see above.
// #[runtime::test]
// async fn test_prioritize_transaction() {
//     let client = common::setup();

//     client.prioritize_transaction(&tx_hash).await.unwrap();
// }

#[async_std::test]
async fn test_estimate_fee() {
    let client = common::setup();

    let fee = client.estimate_fee(10).await;

    assert!(fee.is_ok());
}

#[async_std::test]
async fn test_estimate_priority() {
    let client = common::setup();

    let priority = client.estimate_priority(10).await;

    assert!(priority.is_ok());
}

#[async_std::test]
async fn test_estimate_smart_fee() {
    let client = common::setup();

    let smartfee = client.estimate_smart_fee(10).await;

    assert!(smartfee.is_ok());
}

#[async_std::test]
async fn test_estimate_smart_priority() {
    let client = common::setup();

    let smartpriority = client.estimate_smart_priority(10).await;

    assert!(smartpriority.is_ok());
}
