mod common;

use common::setup;

#[test]
fn test_prune_blockchain() {
    let client = setup();

    let result = client.prune_blockchain();

    assert!(result.is_ok());
}

//TODO finish up these tests
//#[test]
//fn test_invalidateblock() {
//    //We should have a common function that pulls the latest block and tests it.

//}
//
//#[test]
//fn test_reconsiderblock() {
//    //Likewise as above, grab the block from common after invalidate has been called.

//}
