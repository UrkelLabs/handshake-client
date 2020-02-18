use handshake_wallet_rpc::HandshakeWalletRpcClient;

pub fn setup() -> HandshakeWalletRpcClient {
    HandshakeWalletRpcClient::new("http://localhost:12039")
}
