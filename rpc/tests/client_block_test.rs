mod common;

use common::setup;

#[async_std::test]
async fn test_get_blockchain_info() {
    let client = setup();

    let info = client.get_blockchain_info().await;

    assert!(info.is_ok());
}

#[async_std::test]
async fn test_get_best_blockhash() {
    let client = common::setup();

    let besthash = client.get_best_block_hash().await;

    assert!(besthash.is_ok());
}

#[async_std::test]
async fn test_get_block_count() {
    let client = common::setup();

    let blockcount = client.get_block_count().await;

    assert!(blockcount.is_ok());
}

#[async_std::test]
async fn test_get_block() {
    let client = common::setup();

    let hash = client.get_best_block_hash().await.unwrap().to_string();

    let block = client.get_block(&hash).await;

    assert!(block.is_ok());
}

#[async_std::test]
async fn test_get_block_verbose() {
    let client = common::setup();

    let hash = client.get_best_block_hash().await.unwrap().to_string();

    let block = client.get_block_verbose(&hash).await;

    assert!(block.is_ok());
}

//@todo failing - details not working.
#[async_std::test]
async fn test_get_block_verbose_and_details() {
    let client = common::setup();

    let hash = client.get_best_block_hash().await.unwrap().to_string();

    let block = client.get_block_detailed(&hash).await;

    dbg!(&block.unwrap().tx[0].vin);

    // assert!(block.is_ok());
}

#[async_std::test]
async fn test_get_block_by_height() {
    let client = common::setup();

    let height = 100;

    let block = client.get_block_by_height(height).await;

    dbg!(&block);

    assert!(block.is_ok());
}

#[async_std::test]
async fn test_get_block_by_height_verbose() {
    let client = common::setup();

    let height = 7760;

    let block = client.get_block_by_height_verbose(height).await;

    dbg!(&block);

    assert!(block.is_ok());
}

#[async_std::test]
async fn test_get_block_by_height_verbose_and_details() {
    let client = common::setup();

    let height = 7760;

    let block = client.get_block_by_height_detailed(height).await;

    dbg!(&block);

    assert!(block.is_ok());
}

#[async_std::test]
async fn test_get_block_hash() {
    let client = common::setup();

    let height = 100;

    let blockhash = client.get_block_hash(height).await;

    assert!(blockhash.is_ok());
}

#[async_std::test]
async fn test_get_block_header() {
    let client = common::setup();

    //@todo if we use this a lot, wrap this into common.
    let hash = client.get_best_block_hash().await.unwrap().to_string();

    let blockheader = client.get_block_header(&hash).await;

    assert!(blockheader.is_ok());
}

#[async_std::test]
async fn test_get_block_header_verbose() {
    let client = common::setup();

    //@todo if we use this a lot, wrap this into common.
    let hash = client.get_best_block_hash().await.unwrap().to_string();

    let blockheader = client.get_block_header_verbose(&hash).await;

    assert!(blockheader.is_ok());
}
