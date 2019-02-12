use crate::responses;

use jsonrpc::error::Error;

use crate::client::HSClient;

use serde_json::json;

impl HSClient {
    // TODO The return type here is accurate, but hasn't been given a struct
    /// Get network hash per second
    pub fn getnetworkhashps(&mut self, blocks: u32, height: u32) -> Result<responses::GetNetworkHashps, Error> {
        self.call(
            "getnetworkhashps",
            &[json!(blocks), json!(height)]
        )
    }

    pub fn getmininginfo(&mut self) -> Result<responses::GetMiningInfo, Error> {
        self.call("getmininginfo", &[])
    }

    pub fn getwork(&mut self) -> Result<responses::GetWork, Error> {
        self.call("getwork", &[])
    }
}
