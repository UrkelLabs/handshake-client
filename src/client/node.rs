use crate::responses;

use jsonrpc::error::Error;

use crate::client::HSClient;

use serde_json;
use serde_json::json;


impl HSClient {
    /// Show information about this node.
    pub fn getinfo(&self) -> Result<responses::GetInfo, Error> {
        self.call("getinfo", &[])
    }

    pub fn getmemoryinfo(&self) -> Result<responses::GetMemoryInfo, Error> {
        self.call("getmemoryinfo", &[])
    }

    /// Set the log level on the node.
    pub fn setloglevel(&self, level: &str) -> Result<(), Error> {
        self.call("setloglevel", &[json!(level)])
    }

    /// validate an address
    pub fn validateaddress(&self, address: &str) -> Result<responses::ValidateAddress, Error> {
        self.call("validateaddress", &[json!(address)] )
    }

    pub fn stop(&self) -> Result<responses::Stop, Error> {
        self.call("stop", &[])
    }

    pub fn createmultisig(&self, nrequired: &u32, keys: &Vec<String>) -> Result<responses::CreateMultiSig, Error> {
        self.call("createmultisig", &[json!(nrequired), json!(keys)])
    }

    pub fn signmessagewithprivkey(&self, privkey: &str, message: &str) -> Result<responses::SignMessageWithPrivKey, Error> {
        self.call("signmessagewithprivkey", &[json!(privkey), json!(message)])
    }

    pub fn verifymessage(&self, address: &str, signature: &str, message: &str) -> Result<responses::VerifyMessage, Error> {
        self.call("verifymessage", &[json!(address), json!(signature), json!(message)])
    }

    pub fn setmocktime(&self, time: &u64) -> Result<(), Error> {
        self.call("setmocktime", &[json!(time)])
    }

}
