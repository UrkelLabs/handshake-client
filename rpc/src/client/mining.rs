use crate::client::HandshakeRpcClient;
use crate::responses;
use crate::Result;
use serde_json::json;

impl HandshakeRpcClient {
    // TODO The return type here is accurate, but hasn't been given a struct
    /// Get network hash per second
    pub async fn get_network_hashps(
        &self,
        blocks: &u32,
        height: &u32,
    ) -> Result<responses::GetNetworkHashps> {
        let params = vec![json!(blocks), json!(height)];
        self.call("getnetworkhashps", &params).await
    }

    pub async fn get_mining_info(&self) -> Result<responses::GetMiningInfo> {
        self.call("getmininginfo", &[]).await
    }

    pub async fn get_work(&self) -> Result<responses::GetWork> {
        self.call("getwork", &[]).await
    }

    //TODO implement long polling for this.
    // pub fn get_work_lp(&self) -> Result<respones::GetWork, Error> {
    //     self.call("getworklp", &[])
    // }

    //TODO this needs to be totally revamped. See here for all params: https://bitcoincore.org/en/doc/0.17.0/rpc/mining/getblocktemplate/
    //Make them optional as well.
    //For now, going to remove them entirely.
    pub async fn get_block_template(
        &self,
        // json_request_object: &serde_json::Value,
    ) -> Result<responses::BlockTemplate> {
        let params = vec![json!({})];
        self.call("getblocktemplate", &params).await
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

    pub async fn set_generate(&self, mining: &u32, proclimit: &u32) -> Result<bool> {
        let params = vec![json!(mining), json!(proclimit)];
        self.call("setgenerate", &params).await
    }

    pub async fn get_generate(&self) -> Result<bool> {
        self.call("getgenerate", &[]).await
    }

    //TODO check params -> Might have "maxretries"
    pub async fn generate(&self, num_blocks: &u32) -> Result<Vec<String>> {
        let params = vec![json!(num_blocks)];
        self.call("generate", &params).await
    }

    pub async fn generate_to_address(
        &self,
        num_blocks: &u32,
        address: &str,
    ) -> Result<Vec<String>> {
        let params = vec![json!(num_blocks), json!(address)];
        self.call("generatetoaddress", &params).await
    }
}
