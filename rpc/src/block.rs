use crate::client::HandshakeRpcClient;
use crate::Result;
use extended_primitives::Hash;
use handshake_client_types::{GetBlock, GetBlockDetailed, GetBlockHeader, GetBlockchainInfo};
use serde_json::json;

impl HandshakeRpcClient {
    pub async fn get_blockchain_info(&self) -> Result<GetBlockchainInfo> {
        self.call("getblockchaininfo", &[]).await
    }

    pub async fn get_best_block_hash(&self) -> Result<Hash> {
        self.call("getbestblockhash", &[]).await
    }

    pub async fn get_block_count(&self) -> Result<u32> {
        self.call("getblockcount", &[]).await
    }

    /// get_block returns the block in Hex format.
    //@todo this should probably be a different type. Hash maybe.
    pub async fn get_block<T: ToString>(&self, blockhash: T) -> Result<String> {
        let params = vec![json!(blockhash.to_string()), json!(false), json!(false)];

        self.call("getblock", &params).await
    }

    //@todo this should probably be a different type. Hash?
    pub async fn get_blocks<T: ToString>(&self, blockhashes: &[T]) -> Result<Vec<String>> {
        let mut params_set = Vec::new();
        for hash in blockhashes {
            params_set.push(vec![json!(hash.to_string()), json!(false), json!(false)]);
        }

        self.batch("getblock", &params_set).await
    }

    pub async fn get_block_verbose<T: ToString>(&self, blockhash: T) -> Result<GetBlock> {
        let params = vec![json!(blockhash.to_string()), json!(true), json!(false)];

        self.call("getblock", &params).await
    }

    //Batch
    pub async fn get_blocks_verbose<T: ToString>(
        &self,
        blockhashes: &[T],
    ) -> Result<Vec<GetBlock>> {
        let mut params_set = Vec::new();
        for hash in blockhashes {
            params_set.push(vec![json!(hash.to_string()), json!(true), json!(false)]);
        }

        self.batch("getblock", &params_set).await
    }

    pub async fn get_block_detailed<T: ToString>(&self, blockhash: T) -> Result<GetBlockDetailed> {
        let params = vec![json!(blockhash.to_string()), json!(true), json!(true)];

        self.call("getblock", &params).await
    }

    //Batch
    pub async fn get_blocks_detailed<T: ToString>(
        &self,
        blockhashes: &[T],
    ) -> Result<Vec<GetBlockDetailed>> {
        let mut params_set = Vec::new();
        for hash in blockhashes {
            params_set.push(vec![json!(hash.to_string()), json!(true), json!(true)]);
        }

        self.batch("getblock", &params_set).await
    }

    //Returns a hex of the block
    pub async fn get_block_by_height(&self, blockheight: u32) -> Result<String> {
        let params = vec![json!(blockheight), json!(false), json!(false)];

        self.call("getblockbyheight", &params).await
    }

    pub async fn get_block_by_height_verbose(&self, blockheight: u32) -> Result<GetBlock> {
        let params = vec![json!(blockheight), json!(true), json!(false)];

        self.call("getblockbyheight", &params).await
    }

    pub async fn get_block_by_height_detailed(&self, blockheight: u32) -> Result<GetBlockDetailed> {
        let params = vec![json!(blockheight), json!(true), json!(true)];

        self.call("getblockbyheight", &params).await
    }

    pub async fn get_block_hash(&self, height: u32) -> Result<Hash> {
        let params = vec![json!(height)];
        self.call("getblockhash", &params).await
    }

    //Returns a hex of the block header
    pub async fn get_block_header(&self, blockhash: &str) -> Result<String> {
        let params = vec![json!(blockhash), json!(false)];
        self.call("getblockheader", &params).await
    }
    pub async fn get_block_header_verbose(&self, blockhash: &str) -> Result<GetBlockHeader> {
        let params = vec![json!(blockhash), json!(true)];
        self.call("getblockheader", &params).await
    }
}
