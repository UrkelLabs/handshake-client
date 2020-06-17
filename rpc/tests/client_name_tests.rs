mod common;

use common::setup;

#[async_std::test]
async fn test_grind_name() {
    let client = setup();

    let name = client.grind_name(None).await;

    assert!(name.is_ok());
}

//@todo tests for the rest of names here.
