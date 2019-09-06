use crate::client::HandshakeRpcClient;
use crate::responses;
use crate::Result;
use serde_json::json;

impl HandshakeRpcClient {
    pub async fn get_blockchain_info(&self) -> Result<responses::GetBlockchainInfo> {
        self.call("getblockchaininfo", &[]).await
    }

    pub async fn get_best_blockhash(&self) -> Result<responses::GetBestBlockHash> {
        self.call("getbestblockhash", &[]).await
    }

    pub async fn get_block_count(&self) -> Result<responses::GetBlockCount> {
        self.call("getblockcount", &[]).await
    }

    //TODO break this into multiple functions.
    //verbose will break this.
    //details will also break this.
    pub async fn get_block(
        &self,
        blockhash: &str,
        verbose: bool,
        details: bool,
    ) -> Result<responses::GetBlock> {
        let params = vec![json!(blockhash), json!(verbose), json!(details)];

        self.call("getblock", &params).await
    }

    pub async fn get_block_by_height(
        &self,
        blockheight: u32,
        verbose: bool,
        details: bool,
    ) -> Result<responses::GetBlock> {
        let params = vec![json!(blockheight), json!(verbose), json!(details)];

        self.call("getblockbyheight", &params).await
    }

    //TODO returning strange data -> Likely bug in HSD's RPC interface. Removing until fixed
    // pub fn getblockhash(&self, blockheight: u32) -> Result<responses::GetBlockHash, Error> {
    //     self.call("getblockhash", &[json!(blockheight)])
    // }

    pub async fn get_block_header(
        &self,
        blockhash: &str,
        verbose: bool,
    ) -> Result<responses::GetBlockHeader> {
        let params = vec![json!(blockhash), json!(verbose)];
        self.call("getblockheader", &params).await
    }

    pub async fn get_chain_tips(&self) -> Result<responses::GetChainTips> {
        self.call("getchaintips", &[]).await
    }

    pub async fn get_difficulty(&self) -> Result<responses::GetDifficulty> {
        self.call("getdifficulty", &[]).await
    }
}
