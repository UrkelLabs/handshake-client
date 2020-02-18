mod common;

use common::setup;

#[async_std::test]
async fn test_get_balance() {
    let client = setup();

    // let info = client.get_blockchain_info().await;
    //
    client.select_wallet("pool").await;

    let balance = client.get_balance(None).await;

    dbg!(balance);

    // assert!(info.is_ok());
}
