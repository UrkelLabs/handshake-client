use crate::client::HandshakeRpcClient;
use crate::Result;
use handshake_client_types::{CreateClaim, Name, NameInfo, NameProof, NameResource};
use serde_json::json;

impl HandshakeRpcClient {
    //TODO check that the endpoint only parses result{} and not the error or id.
    pub async fn get_name_info(&self, name: &str) -> Result<NameInfo> {
        self.call("getnameinfo", &[json!(name)]).await
    }

    ////TODO finish output with enums
    pub async fn get_names(&self) -> Result<Vec<Name>> {
        self.call("getnames", &[]).await
    }

    pub async fn get_name_by_hash(&self, name_hash: &str) -> Result<String> {
        self.call("getnamebyhash", &[json!(name_hash)]).await
    }

    ////TODO test return value.
    pub async fn get_name_resource(&self, name: &str) -> Result<NameResource> {
        self.call("getnameresource", &[json!(name)]).await
    }

    pub async fn get_name_proof(&self, name: &str) -> Result<NameProof> {
        self.call("getnameproof", &[json!(name)]).await
    }

    pub async fn create_claim(&self, name: &str) -> Result<CreateClaim> {
        self.call("createclaim", &[json!(name)]).await
    }

    pub async fn send_claim(&self, name: &str) -> Result<()> {
        self.call("sendclaim", &[json!(name)]).await
    }

    pub async fn send_raw_claim(&self, claim: &str) -> Result<()> {
        self.call("sendrawclaim", &[json!(claim)]).await
    }

    //@todo DNSSEC request.

    pub async fn send_raw_airdrop(&self, airdrop: &str) -> Result<()> {
        self.call("sendrawairdrop", &[json!(airdrop)]).await
    }

    pub async fn grind_name(&self, length: Option<u32>) -> Result<String> {
        self.call("grindname", &[json!(length)]).await
    }
}
