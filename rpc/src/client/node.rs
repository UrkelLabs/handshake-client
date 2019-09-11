use crate::client::HandshakeRpcClient;
use crate::responses;
use crate::Result;
use serde_json::json;
use handshake_client_types::GetInfo;

impl HandshakeRpcClient {
    /// Show information about this node.
    pub async fn get_info(&self) -> Result<GetInfo> {
        self.call("getinfo", &[]).await
    }

    pub async fn get_memory_info(&self) -> Result<responses::GetMemoryInfo> {
        self.call("getmemoryinfo", &[]).await
    }

    /// Set the log level on the node.
    pub async fn set_log_level(&self, level: &str) -> Result<()> {
        let params = vec![json!(level)];
        self.call("setloglevel", &params).await
    }

    /// validate an address
    pub async fn validate_address(&self, address: &str) -> Result<responses::ValidateAddress> {
        let params = vec![json!(address)];
        self.call("validateaddress", &params).await
    }

    pub async fn stop(&self) -> Result<responses::Stop> {
        self.call("stop", &[]).await
    }

    pub async fn create_multisig(
        &self,
        nrequired: &u32,
        keys: &Vec<String>,
    ) -> Result<responses::CreateMultiSig> {
        let params = vec![json!(nrequired), json!(keys)];
        self.call("createmultisig", &params).await
    }

    pub async fn sign_message_with_priv_key(
        &self,
        privkey: &str,
        message: &str,
    ) -> Result<responses::SignMessageWithPrivKey> {
        let params = vec![json!(privkey), json!(message)];
        self.call("signmessagewithprivkey", &params).await
    }

    pub async fn verify_message(
        &self,
        address: &str,
        signature: &str,
        message: &str,
    ) -> Result<responses::VerifyMessage> {
        let params = vec![json!(address), json!(signature), json!(message)];
        self.call("verifymessage", &params).await
    }

    pub async fn set_mock_time(&self, time: &u64) -> Result<()> {
        let params = vec![json!(time)];
        self.call("setmocktime", &params).await
    }
}
