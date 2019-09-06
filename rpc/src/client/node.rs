use crate::responses;
use crate::client::HandshakeRpcClient;
use serde_json::json;
use crate::Result;

impl HandshakeRpcClient {
    /// Show information about this node.
    pub async fn get_info(&self) -> Result<responses::GetInfo> {
        self.call("getinfo", &[]).await
    }

    pub async fn get_memory_info(&self) -> Result<responses::GetMemoryInfo> {
        self.call("getmemoryinfo", &[]).await
    }

    /// Set the log level on the node.
    pub async fn set_log_level(&self, level: &str) -> Result<()> {
        self.call("setloglevel", &[json!(level)]).await
    }

    /// validate an address
    pub async fn validate_address(&self, address: &str) -> Result<responses::ValidateAddress> {
        self.call("validateaddress", &[json!(address)]).await
    }

    pub async fn stop(&self) -> Result<responses::Stop> {
        self.call("stop", &[]).await
    }

    pub async fn create_multisig(
        &self,
        nrequired: &u32,
        keys: &Vec<String>,
    ) -> Result<responses::CreateMultiSig> {
        self.call("createmultisig", &[json!(nrequired), json!(keys)]).await
    }

    pub async fn sign_message_with_priv_key(
        &self,
        privkey: &str,
        message: &str,
    ) -> Result<responses::SignMessageWithPrivKey> {
        self.call("signmessagewithprivkey", &[json!(privkey), json!(message)]).await
    }

    pub async fn verify_message(
        &self,
        address: &str,
        signature: &str,
        message: &str,
    ) -> Result<responses::VerifyMessage> {
        self.call(
            "verifymessage",
            &[json!(address), json!(signature), json!(message)],
        ).await
    }

    pub async fn set_mock_time(&self, time: &u64) -> Result<()> {
        self.call("setmocktime", &[json!(time)]).await
    }
}
