use crate::responses;
use crate::requests;

use jsonrpc::error::Error;

use crate::client::HSClient;

impl HSClient {
    /// Show information about this node.
    pub fn getinfo(&mut self) -> Result<responses::GetInfo, Error> {
        self.call("getinfo", requests::GetInfo {})
    }

    pub fn getmemoryinfo(&mut self) -> Result<responses::GetMemoryInfo, Error> {
        self.call("getmemoryinfo", requests::GetMemoryInfo {})
    }

    /// Set the log level on the node.
    pub fn setloglevel(&mut self, level: String) -> Result<(), Error> {
        self.call("setloglevel", requests::SetLogLevel { level })
    }

    /// validate an address
    pub fn validateaddress(&mut self, address: String) -> Result<responses::ValidateAddress, Error> {
        self.call("validateaddress", requests::ValidateAddress { address })
    }

}
