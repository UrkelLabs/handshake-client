mod common;

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
