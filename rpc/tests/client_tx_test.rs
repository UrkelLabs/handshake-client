mod common;

use common::{get_tx_hex, get_txid, setup};

#[test]
//TODO
// fn test_gettxout() {
//     let mut client = common::setup();

//     let txout = client.gettxout();

//     assert!(txout.is_ok());

// }
//
#[test]
fn test_get_tx_out_set_info() {
    let client = setup();

    let outsetinfo = client.get_tx_out_set_info();

    assert!(outsetinfo.is_ok());
}

#[test]
fn test_get_raw_transaction() {
    let client = setup();

    let txid = get_txid();

    let tx = client.get_raw_transaction(&txid);

    assert!(tx.is_ok())
}

#[test]
fn test_get_raw_transaction_verbose() {
    let client = setup();

    let txid = get_txid();

    let tx = client.get_raw_transaction_verbose(&txid);

    assert!(tx.is_ok());
}

#[test]
fn test_decode_raw_transaction() {
    let client = setup();

    let txhex = get_tx_hex();

    let tx = client.decode_raw_transaction(&txhex);

    assert!(tx.is_ok());
}

#[test]
fn test_decode_script() {
    let client = setup();

    let script = "76c014af92ad98c7f77559f96430dfef2a6805b87b24f888ac";

    let decodedscript = client.decode_script(script);

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
