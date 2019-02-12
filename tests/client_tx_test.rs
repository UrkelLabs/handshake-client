mod common;

#[test]
//TODO
// fn test_gettxout() {
//     let mut client = common::setup();

//     let txout = client.gettxout();

//     assert!(txout.is_ok());

// }
//
#[test]
fn test_gettxoutsetinfo() {
    let mut client = common::setup();

    let outsetinfo = client.gettxoutsetinfo();

    assert!(outsetinfo.is_ok());

}

#[test]
fn test_getrawtransaction() {
    let mut client = common::setup();

    let txid = common::gettxid();

    let tx = client.getrawtransaction(txid);

    assert!(tx.is_ok())
}

#[test]
fn test_getrawtransaction_verbose() {
    let mut client = common::setup();

    let txid = common::gettxid();

    let tx = client.getrawtransactionverbose(txid);

    assert!(tx.is_ok());

}

#[test]
fn test_decoderawtransaction() {
    let mut client = common::setup();

    let txhex = common::gettxhex();

    let tx = client.decoderawtransaction(txhex);

    assert!(tx.is_ok());

}

#[test]
fn test_decodescript() {
    let mut client = common::setup();

    let script = "76c014af92ad98c7f77559f96430dfef2a6805b87b24f888ac".to_string();

    let decodedscript = client.decodescript(script);

    assert!(decodedscript.is_ok());

}

// #[test]
// fn test_sendrawtransaction() {
//     let mut client = common::setup();
//     TODO

//     let rawtx = "";

//     let sent = client.sendrawtransaction(rawtx);

//     assert!(sent.is_ok());
// }
//
// #[test]
// fn test_gettxoutproof() {

//     let mut client = common::setup();

// }
//
// #[test]
// fn test_verifytxoutproof() {
// TODO

// }
