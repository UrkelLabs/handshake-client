use handshake_rpc::HandshakeRpcClient;

pub fn setup() -> HandshakeRpcClient {
    HandshakeRpcClient::new("http://127.0.0.1:12037")
}

//@todo make this global, so we only do once.
pub async fn get_txid(client: &HandshakeRpcClient) -> String {
    let tip = client.get_block_count().await.unwrap();

    let block = client.get_block_by_height_verbose(tip).await.unwrap();

    block.tx[0].clone()
}

pub fn get_tx_hex() -> String {
    "00000000010000000000000000000000000000000000000000000000000000000000000000ffffffff6a8a34ba0100ca9a3b00000000001409e0b3bc9c04a4589d1cb8ba5fdbd521055fac0300001f120000030c6d696e65642062792068736408647f83f2c65dbf88080000000000000000".to_string()
}
