use crate::client::HandshakeRpcClient;
use crate::Result;
use handshake_client_types::{CreateMultiSig, GetInfo, GetMemoryInfo, ValidateAddress};
use serde_json::json;

impl HandshakeRpcClient {
    /// Show information about this node.
    pub async fn get_info(&self) -> Result<GetInfo> {
        self.call("getinfo", &[]).await
    }

    pub async fn get_memory_info(&self) -> Result<GetMemoryInfo> {
        self.call("getmemoryinfo", &[]).await
    }

    /// Set the log level on the node.
    pub async fn set_log_level(&self, level: &str) -> Result<()> {
        let params = vec![json!(level)];
        self.call("setloglevel", &params).await
    }

    /// validate an address
    pub async fn validate_address(&self, address: &str) -> Result<ValidateAddress> {
        let params = vec![json!(address)];
        self.call("validateaddress", &params).await
    }

    pub async fn stop(&self) -> Result<String> {
        self.call("stop", &[]).await
    }

    pub async fn create_multisig(&self, nrequired: u32, keys: &[&str]) -> Result<CreateMultiSig> {
        let params = vec![json!(nrequired), json!(keys)];
        self.call("createmultisig", &params).await
    }

    pub async fn sign_message_with_priv_key(&self, privkey: &str, message: &str) -> Result<String> {
        let params = vec![json!(privkey), json!(message)];
        self.call("signmessagewithprivkey", &params).await
    }

    pub async fn verify_message(
        &self,
        address: &str,
        signature: &str,
        message: &str,
    ) -> Result<bool> {
        let params = vec![json!(address), json!(signature), json!(message)];
        self.call("verifymessage", &params).await
    }

    pub async fn set_mock_time(&self, time: u64) -> Result<()> {
        let params = vec![json!(time)];
        self.call("setmocktime", &params).await
    }
}
