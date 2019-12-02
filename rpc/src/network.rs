use crate::client::HandshakeRpcClient;
use crate::Result;
use handshake_client_types::{AddedNodeInfo, BannedNode, NetTotals, NetworkInfo, PeerInfo};
use serde_json::json;

impl HandshakeRpcClient {
    pub async fn get_connection_count(&self) -> Result<u32> {
        self.call("getconnectioncount", &[]).await
    }

    pub async fn ping(&self) -> Result<()> {
        self.call("ping", &[]).await
    }

    pub async fn get_peer_info(&self) -> Result<Vec<PeerInfo>> {
        self.call("getpeerinfo", &[]).await
    }

    pub async fn add_node(&self, addr: &str, cmd: &str) -> Result<()> {
        let params = vec![json!(addr), json!(cmd)];
        self.call("addnode", &params).await
    }

    pub async fn disconnect_node(&self, addr: &str) -> Result<()> {
        let params = vec![json!(addr)];
        self.call("disconnectnode", &params).await
    }

    pub async fn get_added_node_info(&self, addr: &str) -> Result<AddedNodeInfo> {
        let params = vec![json!(addr)];
        self.call("getaddednodeinfo", &params).await
    }

    pub async fn get_net_totals(&self) -> Result<NetTotals> {
        self.call("getnettotals", &[]).await
    }

    pub async fn get_network_info(&self) -> Result<NetworkInfo> {
        self.call("getnetworkinfo", &[]).await
    }

    pub async fn set_ban(&self, addr: &str, cmd: &str) -> Result<()> {
        let params = vec![json!(addr), json!(cmd)];
        self.call("setban", &params).await
    }

    pub async fn list_banned(&self) -> Result<Vec<BannedNode>> {
        self.call("listbanned", &[]).await
    }

    pub async fn clear_banned(&self) -> Result<()> {
        self.call("clearbanned", &[]).await
    }
}
