mod common;

use common::setup;

#[async_std::test]
async fn test_get_chain_tips() {
    let client = setup();

    let result = client.get_chain_tips().await;

    assert!(result.is_ok());
}

#[async_std::test]
async fn test_get_difficulty() {
    let client = setup();

    let result = client.get_difficulty().await;

    assert!(result.is_ok());
}

//@todo as of right now prune is irreversable - let's not re-enable until we either have
//an easy destructable test chain, or we have a way to reverse quickly.
//#[runtime::test]
//async fn test_prune_blockchain() {
//    let client = setup();

//    //@todo not a great way to test this since an error technically is still valid.
//    client.prune_blockchain().await;
//}

//@todo see above, reconsider will fail if we are pruned. Also these are potentially dangerous
//tests since they are temporarily irreversable - e.g. if we invalidate a block, and at the same
//time "getblock" is being tested on that. Re-enable these when we have a more atomic testing
//structure.
//#[runtime::test]
//async fn test_invalidate_and_reconsider_block() {
//    let client = setup();

//@todo this is now taken care of in common
//    let hash = client.get_best_block_hash().await.unwrap().to_string();

//    //We invalidate and then immediately reconsider the latest block.
//    //This might cause race conditions. Let's run this a number of times to test.
//    client.invalidate_block(&hash).await.unwrap();

//    client.reconsider_block(&hash).await.unwrap();

//}

//@todo can't be run w/ pruning... See above todos
// #[runtime::test]
// async fn test_verify_chain() {
//     let client = setup();

//     let result = client.verify_chain(0, 1).await;

//     assert!(result.is_ok());
// }
