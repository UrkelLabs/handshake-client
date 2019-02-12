mod common;

#[test]
fn test_getblockchaininfo() {
    let mut client = common::setup();

    let info = client.getblockchaininfo();

    assert!(info.is_ok());
}

#[test]
fn test_getbestblockhash() {
    let mut client = common::setup();

    let besthash = client.getbestblockhash();

    assert!(besthash.is_ok());
}

#[test]
fn test_getblockcount() {
    let mut client = common::setup();

    let blockcount = client.getblockcount();

    assert!(blockcount.is_ok());
}

#[test]
fn test_getblock_defaults() {
    let mut client = common::setup();

    //TODO Future proof this test, so we aren't relying on hardcoded hashes. Grab one from the live
    //chain
    let hash = "88491d658a9865681ca2c86f92f0bf242c0008dc9ca90c40e5f816cb37c1d8e2".to_string();
        //TODO test all possible variations of parameters.
    let block = client.getblock(hash, true, false);

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
fn test_getblockbyheight() {
    let mut client = common::setup();

    let height = 100;

    let block = client.getblockbyheight(height, true, false);

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
    let mut client = common::setup();

    let hash = "88491d658a9865681ca2c86f92f0bf242c0008dc9ca90c40e5f816cb37c1d8e2".to_string();

    let blockheader = client.getblockheader(hash, true);

    assert!(blockheader.is_ok());
}

#[test]
fn test_chaintips() {
    let mut client = common::setup();

    let tips = client.getchaintips();

    assert!(tips.is_ok());

}

#[test]
fn test_getdifficulty() {
    let mut client = common::setup();

    let difficulty = client.getdifficulty();

    assert!(difficulty.is_ok());

}
