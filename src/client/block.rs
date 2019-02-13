use crate::responses;

use jsonrpc::error::Error;

use crate::client::HSClient;

use serde_json;
use serde_json::json;

impl HSClient {
    pub fn getblockchaininfo(&self) -> Result<responses::GetBlockchainInfo, Error> {
        self.call("getblockchaininfo", &[])
    }

    pub fn getbestblockhash(&self) -> Result<responses::GetBestBlockHash, Error> {
        self.call("getbestblockhash", &[])
    }

    pub fn getblockcount(&self) -> Result<responses::GetBlockCount, Error> {
        self.call("getblockcount", &[])
    }

    pub fn getblock(&self, blockhash: &str, verbose: bool, details: bool) -> Result<responses::GetBlock, Error> {
        self.call("getblock", &[json!(blockhash), json!(verbose), json!(details)])
    }

    pub fn getblockbyheight(&self, blockheight: &u32, verbose: bool, details: bool) -> Result<responses::GetBlock, Error> {
        self.call("getblockbyheight", &[json!(blockheight), json!(verbose), json!(details)])
    }

    //TODO returning strange data -> Likely bug in HSD's RPC interface. Removing until fixed
    // pub fn getblockhash(&self, blockheight: u32) -> Result<responses::GetBlockHash, Error> {
    //     self.call("getblockhash", &[json!(blockheight)])
    // }

    pub fn getblockheader(&self, blockhash: &str, verbose: bool) -> Result<responses::GetBlockHeader, Error> {
        self.call("getblockheader", &[json!(blockhash), json!(verbose)])
    }

    pub fn getchaintips(&self) -> Result<responses::GetChainTips, Error> {
        self.call("getchaintips", &[])
    }

    pub fn getdifficulty(&self) -> Result<responses::GetDifficulty, Error> {
        self.call("getdifficulty", &[])
    }
}
