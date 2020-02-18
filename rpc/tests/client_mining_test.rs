mod common;

use common::setup;
// use handshake_primitives::BlockTemplate;
// use handshake_primitives::block_template::json::BlockTemplateJSON;
use handshake_primitives::block_template::builder::BlockTemplateBuilder;

#[async_std::test]
async fn test_get_network_hashps() {
    let client = setup();

    let networkhashps = client.get_network_hashps(120, 1).await;

    assert!(networkhashps.is_ok());
}

#[async_std::test]
async fn test_get_mining_info() {
    let client = common::setup();

    let info = client.get_mining_info().await;

    assert!(info.is_ok());
}

#[async_std::test]
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

#[async_std::test]
async fn test_getblocktemplate() {
    let client = common::setup();

    let template = client.get_block_template().await;

    dbg!(&template);

    let mut block_template = BlockTemplateBuilder::new()
        .with_json(template.unwrap())
        //@todo would be great to have this function accept T: Into<Address>
        .with_address(
            "rs1q80ecwmq0395fp7xwr4wyjkv02jzs5w5y7ynruj"
                .parse()
                .unwrap(),
        )
        .with_create_coinbase()
        .with_create_merkle_root()
        .with_create_witness_root()
        .build();

    // dbg!(&block_template);
    dbg!(block_template.coinbase);

    // assert!(template.is_ok());
}

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

#[async_std::test]
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
