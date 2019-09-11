mod common;

use common::setup;

#[runtime::test]
async fn test_get_network_hashps() {
    let client = setup();

    let networkhashps = client.get_network_hashps(120, 1).await;

    assert!(networkhashps.is_ok());
}

#[runtime::test]
async fn test_get_mining_info() {
    let client = common::setup();

    let info = client.get_mining_info().await;

    assert!(info.is_ok());
}

#[runtime::test]
async fn test_get_work() {
    let client = common::setup();

    let work = client.get_work().await;

    assert!(work.is_ok());
}

//@todo submit work is currently failing with out of bounds read.
// #[runtime::test]
// async fn test_submit_work() {
//     let client = common::setup();

//     let hash = "abcd";
//     let work = client.submit_work(&hash).await;

//     assert!(work.is_ok());
// }

// #[test]
// fn test_getblocktemplate() {
//     let client = common::setup();

//     let template = client.get_block_template();

//     assert!(template.is_ok());
// }

// #[runtime::test]
// async fn test_submit_block() {
//     let client = common::setup();

//     let block = "abcd";

//     client.submit_block(&block).await.unwrap();
// }

// #[runtime::test]
// async fn test_verify_block() {
//     let client = common::setup();

//     let block = "abcd";

//     client.verify_block(&block).await.unwrap();
// }

#[runtime::test]
async fn test_set_and_get_generate() {
    let client = common::setup();

    //@todo might be excessive... this test takes awhile.
    client.set_generate(false, None).await.unwrap();
    assert_eq!(false, client.get_generate().await.unwrap());
    // client.set_generate(true, None).await.unwrap();
    // assert_eq!(true, client.get_generate().await.unwrap());
    // client.set_generate(false, Some(1)).await.unwrap();
    // assert_eq!(false, client.get_generate().await.unwrap());
    // client.set_generate(true, Some(1)).await.unwrap();
    // assert_eq!(true, client.get_generate().await.unwrap());
}

// #[runtime::test]
// async fn test_generate() {
//     let client = common::setup();

//     let blocks = client.generate(2, None).await;
//     assert!(blocks.is_ok());

//     let blocks = client.generate(2, Some(2)).await;
//     assert!(blocks.is_ok());
// }

//#[runtime::test]
//async fn test_generate_to_address() {
//    let client = common::setup();
//    //@todo get a new address generated here to use

//    let blocks = client.generate(2, &addr, 2).await;

//    assert!(blocks.is_ok());
//}

