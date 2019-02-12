use crate::responses;

use jsonrpc::error::Error;

use crate::client::HSClient;

use serde_json;
use serde_json::json;

impl HSClient {
    pub fn getblockchaininfo(&mut self) -> Result<responses::GetBlockchainInfo, Error> {
        self.call("getblockchaininfo", &[])
    }

    pub fn getbestblockhash(&mut self) -> Result<responses::GetBestBlockHash, Error> {
        self.call("getbestblockhash", &[])
    }

    pub fn getblockcount(&mut self) -> Result<responses::GetBlockCount, Error> {
        self.call("getblockcount", &[])
    }

    pub fn getblock(&mut self, blockhash: String, verbose: bool, details: bool) -> Result<responses::GetBlock, Error> {
        self.call("getblock", &[json!(blockhash), json!(verbose), json!(details)])
    }

    pub fn getblockbyheight(&mut self, blockheight: u32, verbose: bool, details: bool) -> Result<responses::GetBlock, Error> {
        self.call("getblockbyheight", &[json!(blockheight), json!(verbose), json!(details)])
    }

    //TODO returning strange data -> Likely bug in HSD's RPC interface. Removing until fixed
    // pub fn getblockhash(&mut self, blockheight: u32) -> Result<responses::GetBlockHash, Error> {
    //     self.call("getblockhash", &[json!(blockheight)])
    // }

    pub fn getblockheader(&mut self, blockhash: String, verbose: bool) -> Result<responses::GetBlockHeader, Error> {
        self.call("getblockheader", &[json!(blockhash), json!(verbose)])
    }

    pub fn getchaintips(&mut self) -> Result<responses::GetChainTips, Error> {
        self.call("getchaintips", &[])
    }

    pub fn getdifficulty(&mut self) -> Result<responses::GetDifficulty, Error> {
        self.call("getdifficulty", &[])
    }
}
