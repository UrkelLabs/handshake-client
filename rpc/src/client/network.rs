use crate::responses;
use crate::client::HandshakeRpcClient;
use serde_json::json;
use crate::Result;

impl HandshakeRpcClient {
    pub async fn get_connection_count(&self) -> Result<responses::ConnectionCount> {
        self.call("getconnectioncount", &[]).await
    }

    pub async fn ping(&self) -> Result<()> {
        self.call("ping", &[]).await
    }

    // not sure if will work but we'll see
    pub async fn get_peer_info(&self) -> Result<Vec<responses::PeerInfo>> {
        self.call("getpeerinfo", &[]).await
    }

    //Consider breaking this into multiple functions since we only have 3 acceptable cmds
    //"add", "onetry", "remove"
    pub async fn add_node(&self, addr: &str, cmd: &str) -> Result<()> {

        let params = vec![json!(addr), json!(cmd)];
        self.call("addnode", &params).await
    }

    pub async fn disconnect_node(&self, addr: &str) -> Result<()> {

        let params = vec![json!(addr)];
        self.call("disconnectnode", &params).await
    }

    pub async fn get_added_node_info(&self, addr: &str) -> Result<responses::AddedNodeInfo> {

        let params = vec![json!(addr)];
        self.call("getaddednodeinfo", &params).await
    }

    pub async fn get_net_totals(&self) -> Result<responses::NetTotals> {
        self.call("getnettotals", &[]).await
    }

    pub async fn get_network_info(&self) -> Result<responses::NetworkInfo> {
        self.call("getnetworkinfo", &[]).await
    }

    //AS with add_node this command has a set amount of cmds so consider breaking this up
    pub async fn set_ban(&self, addr: &str, cmd: &str) -> Result<()> {

        let params = vec![json!(addr), json!(cmd)];
        self.call("setban", &params).await
    }

    pub async fn list_banned(&self) -> Result<Vec<responses::BannedNode>> {
        self.call("listbanned", &[]).await
    }

    pub async fn clear_banned(&self) -> Result<()> {
        self.call("clearbanned", &[]).await
    }
}
