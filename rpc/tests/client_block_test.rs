mod common;

use common::setup;

#[test]
fn test_get_blockchain_info() {
    let client = setup();

    let info = client.get_blockchain_info();

    assert!(info.is_ok());
}

#[test]
fn test_get_best_blockhash() {
    let client = common::setup();

    let besthash = client.get_best_blockhash();

    assert!(besthash.is_ok());
}

#[test]
fn test_get_block_count() {
    let client = common::setup();

    let blockcount = client.get_block_count();

    assert!(blockcount.is_ok());
}

#[test]
fn test_get_block_defaults() {
    let client = common::setup();

    //TODO Future proof this test, so we aren't relying on hardcoded hashes. Grab one from the live
    //chain
    let hash = "88491d658a9865681ca2c86f92f0bf242c0008dc9ca90c40e5f816cb37c1d8e2";
    //TODO test all possible variations of parameters.
    let block = client.get_block(hash, true, false);

    assert!(block.is_ok());
}

//TODO implement full Transaction deserialization if details = true.
//#[test]
//fn test_getblock_verbose_details() {
//    let mut client = common::setup();

//    //TODO Future proof this test, so we aren't relying on hardcoded hashes. Grab one from the live
//    //chain
//    let hash = "88491d658a9865681ca2c86f92f0bf242c0008dc9ca90c40e5f816cb37c1d8e2".to_string();
//        //TODO test all possible variations of parameters.
//    let block = client.getblock(hash, true, true);

//    assert!(block.is_ok());
//}

#[test]
fn test_get_block_by_height() {
    let client = common::setup();

    let height = 100;

    let block = client.get_block_by_height(height, true, false);

    assert!(block.is_ok());
}

// #[test]
// fn test_getblockhash() {
//     let mut client = common::setup();

//     let height = 100;

//     let blockhash = client.getblockhash(height);

//     dbg!(&blockhash);

//     assert!(blockhash.is_ok());
// }

#[test]
fn test_getblockheader() {
    let client = common::setup();

    let hash = "88491d658a9865681ca2c86f92f0bf242c0008dc9ca90c40e5f816cb37c1d8e2";

    let blockheader = client.get_block_header(hash, true);

    assert!(blockheader.is_ok());
}

#[test]
fn test_chaintips() {
    let client = common::setup();

    let tips = client.get_chain_tips();

    assert!(tips.is_ok());
}

#[test]
fn test_getdifficulty() {
    let client = common::setup();

    let difficulty = client.get_difficulty();

    assert!(difficulty.is_ok());
}
