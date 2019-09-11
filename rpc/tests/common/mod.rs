use handshake_rpc::HandshakeRpcClient;

pub fn setup() -> HandshakeRpcClient {
    HandshakeRpcClient::new("http://localhost:13037")
}

pub fn get_txid() -> String {
    "42c2a72d64644486bb2cd72100c0ad746300f59d3653ebba097b82ac0aa5f61e".to_string()
}

pub fn get_tx_hex() -> String {
    "00000000010000000000000000000000000000000000000000000000000000000000000000ffffffff6a8a34ba0100ca9a3b00000000001409e0b3bc9c04a4589d1cb8ba5fdbd521055fac0300001f120000030c6d696e65642062792068736408647f83f2c65dbf88080000000000000000".to_string()
}
