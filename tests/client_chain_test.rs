mod common;

#[test]
fn test_pruneblockchain() {
     
    let mut client = common::setup();

    let result = client.pruneblockchain();

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
