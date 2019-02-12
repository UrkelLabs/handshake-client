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
fn test_validateaddress() {
    let mut client = common::setup();

    let validateaddress = client.validateaddress(" 
ts1qq79hzunlkj50fvm7rxg3xetx4kml4e0am43htk".to_string());

    dbg!(&validateaddress);

    assert!(validateaddress.is_ok());

}
