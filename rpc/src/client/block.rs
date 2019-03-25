use crate::responses;

use jsonrpc::error::Error;

use crate::client::RPCClient;

use serde_json;
use serde_json::json;

impl RPCClient {
    pub fn get_blockchain_info(&self) -> Result<responses::GetBlockchainInfo, Error> {
        self.call("getblockchaininfo", &[])
    }

    pub fn get_best_blockhash(&self) -> Result<responses::GetBestBlockHash, Error> {
        self.call("getbestblockhash", &[])
    }

    pub fn get_block_count(&self) -> Result<responses::GetBlockCount, Error> {
        self.call("getblockcount", &[])
    }

    pub fn get_block(
        &self,
        blockhash: &str,
        verbose: bool,
        details: bool,
    ) -> Result<responses::GetBlock, Error> {
        self.call(
            "getblock",
            &[json!(blockhash), json!(verbose), json!(details)],
        )
    }

    pub fn get_block_by_height(
        &self,
        blockheight: &u32,
        verbose: bool,
        details: bool,
    ) -> Result<responses::GetBlock, Error> {
        self.call(
            "getblockbyheight",
            &[json!(blockheight), json!(verbose), json!(details)],
        )
    }

    //TODO returning strange data -> Likely bug in HSD's RPC interface. Removing until fixed
    // pub fn getblockhash(&self, blockheight: u32) -> Result<responses::GetBlockHash, Error> {
    //     self.call("getblockhash", &[json!(blockheight)])
    // }

    pub fn get_block_header(
        &self,
        blockhash: &str,
        verbose: bool,
    ) -> Result<responses::GetBlockHeader, Error> {
        self.call("getblockheader", &[json!(blockhash), json!(verbose)])
    }

    pub fn get_chain_tips(&self) -> Result<responses::GetChainTips, Error> {
        self.call("getchaintips", &[])
    }

    pub fn get_difficulty(&self) -> Result<responses::GetDifficulty, Error> {
        self.call("getdifficulty", &[])
    }
}
