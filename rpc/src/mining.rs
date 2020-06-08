use crate::client::HandshakeRpcClient;
use crate::Result;
use handshake_client_types::{GetMiningInfo, GetWork};
use handshake_primitives::block_template::json::BlockTemplateJSON;
use serde_json::json;

impl HandshakeRpcClient {
    /// Get network hash per second
    pub async fn get_network_hashps(&self, blocks: u32, height: Option<u32>) -> Result<f64> {
        let params = vec![json!(blocks), json!(height)];
        self.call("getnetworkhashps", &params).await
    }

    pub async fn get_mining_info(&self) -> Result<GetMiningInfo> {
        self.call("getmininginfo", &[]).await
    }

    pub async fn get_work(&self) -> Result<GetWork> {
        self.call("getwork", &[]).await
    }

    //TODO implement long polling for this.
    // pub fn get_work_lp(&self) -> Result<respones::GetWork, Error> {
    //     self.call("getworklp", &[])
    // }

    //@todo not sure this return type is correct.
    pub async fn submit_work(&self, work: &str) -> Result<bool> {
        let params = vec![json!(work)];
        self.call("submitwork", &params).await
    }

    //TODO this needs to be totally revamped. See here for all params: https://bitcoincore.org/en/doc/0.17.0/rpc/mining/getblocktemplate/
    //Make them optional as well.
    //For now, going to remove them entirely.
    pub async fn get_block_template(
        &self,
        // json_request_object: &serde_json::Value,
    ) -> Result<BlockTemplateJSON> {
        let params = vec![json!({})];
        self.call("getblocktemplate", &params).await
    }

    pub async fn submit_block(&self, block_data: &str) -> Result<()> {
        let params = vec![json!(block_data)];
        self.call("submitblock", &params).await
    }

    pub async fn verify_block(&self, block_data: &str) -> Result<()> {
        let params = vec![json!(block_data)];
        self.call("verifyblock", &params).await
    }

    //@todo prolimit appears to not be used at all in hsd from this call. Open up an
    //issue/investigate further.
    pub async fn set_generate(&self, mining: bool, proclimit: Option<u32>) -> Result<bool> {
        let params = vec![json!(mining), json!(proclimit)];
        self.call("setgenerate", &params).await
    }

    pub async fn get_generate(&self) -> Result<bool> {
        self.call("getgenerate", &[]).await
    }

    pub async fn generate(&self, num_blocks: u32, max_tries: Option<u32>) -> Result<Vec<String>> {
        let params = vec![json!(num_blocks), json!(max_tries)];
        self.call("generate", &params).await
    }

    pub async fn generate_to_address(
        &self,
        num_blocks: u32,
        address: &str,
        max_tries: Option<u32>,
    ) -> Result<Vec<String>> {
        let params = vec![json!(num_blocks), json!(address), json!(max_tries)];
        self.call("generatetoaddress", &params).await
    }
}
