use crate::responses;

use crate::client::RPCClient;
use crate::error::Error;

use serde_json::json;

impl RPCClient {
    // TODO The return type here is accurate, but hasn't been given a struct
    /// Get network hash per second
    pub fn get_network_hashps(
        &self,
        blocks: &u32,
        height: &u32,
    ) -> Result<responses::GetNetworkHashps, Error> {
        self.call("getnetworkhashps", &[json!(blocks), json!(height)])
    }

    pub fn get_mining_info(&self) -> Result<responses::GetMiningInfo, Error> {
        self.call("getmininginfo", &[])
    }

    pub fn get_work(&self) -> Result<responses::GetWork, Error> {
        self.call("getwork", &[])
    }

    //TODO implement long polling for this.
    // pub fn get_work_lp(&self) -> Result<respones::GetWork, Error> {
    //     self.call("getworklp", &[])
    // }

    pub fn get_block_template(
        &self,
        json_request_object: &serde_json::Value,
    ) -> Result<responses::BlockTemplate, Error> {
        self.call("getblocktemplate", &[json!(json_request_object)])
    }

    //TODO - both are currently erroring with out of bounds errors
    //TODO return value
    //pub fn submit_block(&self, block_data: &str) -> Result<responses::SubmitBlock, Error> {
    //    self.call("submitblock", &[json!(block_data)])
    //}

    ////TODO return value
    //pub fn verify_block(&self, block_data: &str) -> Result<responses::VerifyBlock, Error> {
    //    self.call("verifyblock", &[json!(block_data)])
    //}

    pub fn set_generate(&self, mining: &u32, proclimit: &u32) -> Result<bool, Error> {
        self.call("setgenerate", &[json!(mining), json!(proclimit)])
    }

    pub fn get_generate(&self) -> Result<bool, Error> {
        self.call("getgenerate", &[])
    }

    //TODO check params -> Might have "maxretries"
    pub fn generate(&self, num_blocks: &u32) -> Result<Vec<String>, Error> {
        self.call("generate", &[json!(num_blocks)])
    }

    pub fn generate_to_address(
        &self,
        num_blocks: &u32,
        address: &str,
    ) -> Result<Vec<String>, Error> {
        self.call("generatetoaddress", &[json!(num_blocks), json!(address)])
    }
}
