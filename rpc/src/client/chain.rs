use crate::client::HandshakeRpcClient;
use crate::Result;
use serde_json::json;
use handshake_client_types::{GetBlockchainInfo, GetBlock, ChainTip, GetBlockHeader};

impl HandshakeRpcClient {
    // @todo move this to chain and the one below.
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

    pub async fn invalidate_block(&self, blockhash: &str) -> Result<()> {
        let params = vec![json!(blockhash)];
        self.call("invalidateblock", &params).await
    }

    pub async fn reconsider_block(&self, blockhash: &str) -> Result<()> {
        let params = vec![json!(blockhash)];
        self.call("reconsiderblock", &params).await
    }
}
