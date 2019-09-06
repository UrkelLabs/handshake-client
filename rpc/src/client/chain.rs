use crate::client::HandshakeRpcClient;
use crate::Result;
use serde_json::json;

impl HandshakeRpcClient {
    pub async fn prune_blockchain(&self) -> Result<()> {
        self.call("pruneblockchain", &[]).await
    }

    pub async fn invalidate_block(&self, blockhash: &str) -> Result<()> {
        let params = vec![json!(blockhash)];
        self.call("invalidateblock", &params).await
    }

    pub async fn reconsider_block(&self, blockhash: &str) -> Result<()> {
        let params = vec![json!(blockhash)];
        self.call("reconsiderblock", &params).await
    }
}
