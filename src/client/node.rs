use crate::responses;

use jsonrpc::error::Error;

use crate::client::HSClient;

use serde_json;
use serde_json::json;


impl HSClient {
    /// Show information about this node.
    pub fn getinfo(&mut self) -> Result<responses::GetInfo, Error> {
        self.call("getinfo", &[])
    }

    pub fn getmemoryinfo(&mut self) -> Result<responses::GetMemoryInfo, Error> {
        self.call("getmemoryinfo", &[])
    }

    /// Set the log level on the node.
    pub fn setloglevel(&mut self, level: String) -> Result<(), Error> {
        self.call("setloglevel", &[json!(level)])
    }

    /// validate an address
    pub fn validateaddress(&mut self, address: String) -> Result<responses::ValidateAddress, Error> {
        self.call("validateaddress", &[json!(address)] )
    }

    pub fn stop(&mut self) -> Result<responses::Stop, Error> {
        self.call("stop", &[])
    }

    pub fn createmultisig(&mut self, nrequired: u32, keys: Vec<String>) -> Result<responses::CreateMultiSig, Error> {
        self.call("createmultisig", &[json!(nrequired), json!(keys)])
    }

    pub fn signmessagewithprivkey(&mut self, privkey: String, message: String) -> Result<responses::SignMessageWithPrivKey, Error> {
        self.call("signmessagewithprivkey", &[json!(privkey), json!(message)])
    }

    pub fn verifymessage(&mut self, address: String, signature: String, message: String) -> Result<responses::VerifyMessage, Error> {
        self.call("verifymessage", &[json!(address), json!(signature), json!(message)])
    }

    pub fn setmocktime(&mut self, time: u64) -> Result<(), Error> {
        self.call("setmocktime", &[json!(time)])
    }

}
