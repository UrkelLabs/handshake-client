mod common;
use common::setup;

#[async_std::test]
async fn test_get_info() {
    let client = setup();

    let info = client.get_info().await;

    assert!(info.is_ok());
}

#[async_std::test]
async fn test_get_memory_info() {
    let client = common::setup();

    let memoryinfo = client.get_memory_info().await;

    assert!(memoryinfo.is_ok());
}

#[async_std::test]
async fn test_set_log_level() {
    let client = common::setup();

    let result = client.set_log_level("WARNING").await;

    assert!(result.is_ok());
}


#[async_std::test]
async fn test_validate_address_ok() {
    let client = common::setup();

    let validateaddress = client.validate_address("ts1qq79hzunlkj50fvm7rxg3xetx4kml4e0am43htk").await;

    assert!(validateaddress.is_ok());

    let address = validateaddress.unwrap();

    assert!(address.is_valid);
}

#[async_std::test]
async fn test_validate_address_fail() {
    let client = common::setup();

    let validateaddress = client.validate_address("notanaddress").await;

    assert!(validateaddress.is_ok());

    let address = validateaddress.unwrap();

    assert!(!address.is_valid);
}

////This turns off the code... Let's see if there is a way to restart.
//// #[test]
//// fn test_stop() {
////     let mut client = common::setup();

////     let stop = client.stop();

////     assert!(stop.is_ok());

//// }

#[async_std::test]
async fn test_create_multisig() {
    let client = common::setup();
    let pub1 = "02e3d6bb36b0261628101ee67abd89d678522dc1199912512f814e70803652f395";
    let pub2 = "03d7ded41bb871936bf4d411371b25d706c572f28ef8d2613b45392e9f9c4348a5";
    let pub3 = "034bc2280e68d3bdd0ef0664e0ad2949a467344d8e59e435fe2d9be81e39f70f76";

    let params = vec![pub1, pub2, pub3];

    let multisig = client.create_multisig(1, &params).await;

    assert!(multisig.is_ok());
}

////Hard to test _ok() without exposing priv keys
#[async_std::test]
async fn test_sign_message_with_priv_key_fail() {
    let client = common::setup();

    let privkey = "ENced8VD7YWkzPC8FTJ3gTTq4pQhF2PF79QS51mgZq7BgCfiEP5A";

    let message = "hello";

    let signedmessage = client.sign_message_with_priv_key(privkey, message).await;

    assert!(signedmessage.is_err());
}

////TODO need valid _ok() test
#[async_std::test]
async fn test_verify_message_fail() {
    let client = common::setup();

    //Address is bad.
    let address = "ts1q7qumafugfglg268djelwr7ps4l2uh2vsdpfnuc";

    let signature =
        "arjD5y4glPea270IiExx04E+tTvryHKhWZcA2oy8svVHr9q/AvGA647UF2ICaIGJHazbRyyj3draiNnBns9aWQ==";

    let message = "hello";

    let verified = client.verify_message(address, signature, message).await;

    assert!(verified.is_err());
}

////TODO this will break current tests -> Need to find a way to not have this break things.
//// #[test]
//// fn test_setmocktime() {
////     let mut client = common::setup();

////     let time = 1503058155;

////     let result = client.setmocktime(time);

////     assert!(result.is_ok());

//// }
