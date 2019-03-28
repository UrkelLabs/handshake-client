use crate::responses;

use crate::client::RPCClient;
use crate::error::Error;

use serde_json;
use serde_json::json;

impl RPCClient {
    /// Show information about this node.
    pub fn get_info(&self) -> Result<responses::GetInfo, Error> {
        self.call("getinfo", &[])
    }

    pub fn get_memory_info(&self) -> Result<responses::GetMemoryInfo, Error> {
        self.call("getmemoryinfo", &[])
    }

    /// Set the log level on the node.
    pub fn set_log_level(&self, level: &str) -> Result<(), Error> {
        self.call("setloglevel", &[json!(level)])
    }

    /// validate an address
    pub fn validate_address(&self, address: &str) -> Result<responses::ValidateAddress, Error> {
        self.call("validateaddress", &[json!(address)])
    }

    pub fn stop(&self) -> Result<responses::Stop, Error> {
        self.call("stop", &[])
    }

    pub fn create_multisig(
        &self,
        nrequired: &u32,
        keys: &Vec<String>,
    ) -> Result<responses::CreateMultiSig, Error> {
        self.call("createmultisig", &[json!(nrequired), json!(keys)])
    }

    pub fn sign_message_with_priv_key(
        &self,
        privkey: &str,
        message: &str,
    ) -> Result<responses::SignMessageWithPrivKey, Error> {
        self.call("signmessagewithprivkey", &[json!(privkey), json!(message)])
    }

    pub fn verify_message(
        &self,
        address: &str,
        signature: &str,
        message: &str,
    ) -> Result<responses::VerifyMessage, Error> {
        self.call(
            "verifymessage",
            &[json!(address), json!(signature), json!(message)],
        )
    }

    pub fn set_mock_time(&self, time: &u64) -> Result<(), Error> {
        self.call("setmocktime", &[json!(time)])
    }
}
