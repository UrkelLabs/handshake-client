use crate::client::HandshakeRpcClient;
use crate::Result;
use handshake_client_types::ChainTip;
use serde_json::json;

impl HandshakeRpcClient {
    pub async fn get_chain_tips(&self) -> Result<Vec<ChainTip>> {
        self.call("getchaintips", &[]).await
    }

    //@todo move to Handshake difficulty type.
    pub async fn get_difficulty(&self) -> Result<f64> {
        self.call("getdifficulty", &[]).await
    }

    pub async fn prune_blockchain(&self) -> Result<()> {
        self.call("pruneblockchain", &[]).await
    }

    pub async fn invalidate_block(&self, hash: &str) -> Result<()> {
        let params = vec![json!(hash)];
        self.call("invalidateblock", &params).await
    }

    pub async fn reconsider_block(&self, hash: &str) -> Result<()> {
        let params = vec![json!(hash)];
        self.call("reconsiderblock", &params).await
    }

    pub async fn verify_chain(&self, level: u32, blocks: u32) -> Result<()> {
        let params = vec![json!(level), json!(blocks)];
        self.call("verifychain", &params).await
    }
}
