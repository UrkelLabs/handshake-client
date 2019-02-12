mod common;

#[test]
fn test_getinfo() {

    let mut client = common::setup();

    let info = client.getinfo();

    assert!(info.is_ok());

}

#[test]
fn test_setloglevel() {

    let mut client = common::setup();

    let result = client.setloglevel("WARNING".to_string());

    assert!(result.is_ok());

}

#[test]
fn test_getmemoryinfo() {

    let mut client = common::setup();

    let memoryinfo = client.getmemoryinfo();

    assert!(memoryinfo.is_ok());

}

#[test]
fn test_validateaddress_ok() {
    let mut client = common::setup();

    let validateaddress = client.validateaddress("ts1qq79hzunlkj50fvm7rxg3xetx4kml4e0am43htk".to_string());

    assert!(validateaddress.is_ok());

    let address = validateaddress.unwrap();

    assert!(address.is_valid);
}

#[test]
fn test_validateaddress_fail() {
    let mut client = common::setup();

    let validateaddress = client.validateaddress("notanaddress".to_string());

    assert!(validateaddress.is_ok());

    let address = validateaddress.unwrap();

    assert!(!address.is_valid);

}

//This turns off the code... Let's see if there is a way to restart.
// #[test]
// fn test_stop() {
//     let mut client = common::setup();

//     let stop = client.stop();

//     assert!(stop.is_ok());

// }

#[test]
fn test_createmultisig() {
    let mut client = common::setup();
    let pub1 = "02e3d6bb36b0261628101ee67abd89d678522dc1199912512f814e70803652f395".to_string();
    let pub2 = "03d7ded41bb871936bf4d411371b25d706c572f28ef8d2613b45392e9f9c4348a5".to_string();
    let pub3 = "034bc2280e68d3bdd0ef0664e0ad2949a467344d8e59e435fe2d9be81e39f70f76".to_string();


    let multisig = client.createmultisig(1, vec![pub1, pub2, pub3]);


    assert!(multisig.is_ok());

}

//Hard to test _ok() without exposing priv keys
#[test]
fn test_signmessagewithprivkey_fail() {
    let mut client = common::setup();

    let privkey = "ENced8VD7YWkzPC8FTJ3gTTq4pQhF2PF79QS51mgZq7BgCfiEP5A".to_string();

    let message = "hello".to_string();

    let signedmessage = client.signmessagewithprivkey(privkey, message);

    assert!(signedmessage.is_err());

}

//TODO need valid _ok() test
#[test]
fn test_verifymessage_fail() {
    let mut client = common::setup();

    //Address is bad.
    let address = "ts1q7qumafugfglg268djelwr7ps4l2uh2vsdpfnuc".to_string();

    let signature = "arjD5y4glPea270IiExx04E+tTvryHKhWZcA2oy8svVHr9q/AvGA647UF2ICaIGJHazbRyyj3draiNnBns9aWQ==".to_string();

    let message = "hello".to_string();

    let verified = client.verifymessage(address, signature, message);

    assert!(verified.is_err());

}

//TODO this will break current tests -> Need to find a way to not have this break things.
// #[test]
// fn test_setmocktime() {
//     let mut client = common::setup();

//     let time = 1503058155;

//     let result = client.setmocktime(time);

//     assert!(result.is_ok());

// }

