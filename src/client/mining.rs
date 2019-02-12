use crate::requests;
use crate::responses;

use jsonrpc::error::Error;

use crate::client::HSClient;

impl HSClient {
    // TODO The return type here is accurate, but hasn't been given a struct
    /// Get network hash per second
    pub fn getnetworkhashps(&mut self, blocks: u32, height: u32) -> Result<responses::GetNetworkHashps, Error> {
        self.call(
            "getnetworkhashps",
            requests::GetNetworkHashps { blocks, height },
        )
    }

    pub fn getmininginfo(&mut self) -> Result<responses::GetMiningInfo, Error> {
        self.call("getmininginfo", requests::GetMiningInfo {})
    }

    pub fn getwork(&mut self) -> Result<responses::GetWork, Error> {
        self.call("getwork", requests::GetWork {})
    }
}
