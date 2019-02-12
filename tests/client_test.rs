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


// --- Mining Tests --- //
#[test]
fn test_getnetworkhashps() {
    let mut client = common::setup();

    let networkhashps = client.getnetworkhashps(120, 1);

    dbg!(&networkhashps);

    assert!(networkhashps.is_ok());
}

#[test]
fn test_getmininginfo() {
    let mut client = common::setup();

    let info = client.getmininginfo();

    assert!(info.is_ok());
}

#[test]
fn test_getwork() {
    let mut client = common::setup();

    let work = client.getwork();

    assert!(work.is_ok());
}