mod common;

use common::setup;

#[async_std::test]
async fn test_get_connection_count() {
    let client = common::setup();

    let connections = client.get_connection_count().await;

    assert!(connections.is_ok());
}

#[async_std::test]
async fn test_ping() {
    let client = common::setup();

    client.ping().await.unwrap();
}

#[async_std::test]
async fn test_get_peer_info() {
    let client = common::setup();

    let peers = client.get_peer_info().await;

    dbg!(&peers);

    assert!(peers.is_ok());
}

// #[runtime::test]
// async fn test_add_node() {
//     let client = common::setup();
//     @todo
// }

//#[runtime::test]
//async fn test_disconnect_node() {
//    let client = common::setup();
//    //@todo
//}

//#[runtime::test]
//async fn test_get_added_node_info() {
//    let client = common::setup();
//    //@todo
//}

#[async_std::test]
async fn test_get_net_totals() {
    let client = common::setup();

    let totals = client.get_net_totals().await;

    assert!(totals.is_ok());
}

#[async_std::test]
async fn test_get_network_info() {
    let client = common::setup();

    let info = client.get_network_info().await;

    dbg!(&info);

    assert!(info.is_ok());
}

// #[runtime::test]
// async fn test_set_ban() {
//     let client = common::setup();
//     @todo
// }

#[async_std::test]
async fn test_list_banned() {
    let client = common::setup();

    let names = client.list_banned().await;

    assert!(names.is_ok());
}

#[async_std::test]
async fn test_clear_banned() {
    let client = common::setup();

    client.clear_banned().await.unwrap();
}
