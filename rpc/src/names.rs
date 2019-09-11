use crate::client::HandshakeRpcClient;
use crate::responses;
use crate::Result;
use serde_json::json;

impl HandshakeRpcClient {
    //TODO check that the endpoint only parses result{} and not the error or id.
    pub async fn get_name_info(&self, name: &str) -> Result<responses::NameInfo> {
        let params = vec![json!(name)];
        self.call("getnameinfo", &params).await
    }

    //TODO finish output with enums
    pub async fn get_names(&self) -> Result<Vec<responses::Name>> {
        self.call("getnames", &[]).await
    }

    pub async fn get_name_by_hash(&self, name_hash: &str) -> Result<String> {
        let params = vec![json!(name_hash)];
        self.call("getnamebyhash", &params).await
    }

    //TODO test return value.
    pub async fn get_name_resource(&self, name: &str) -> Result<responses::NameResource> {
        let params = vec![json!(name)];
        self.call("getnameresource", &params).await
    }

    pub async fn get_name_proof(&self, name: &str) -> Result<responses::NameProof> {
        let params = vec![json!(name)];
        self.call("getnameproof", &params).await
    }

    pub async fn create_claim(&self, name: &str) -> Result<responses::CreateClaim> {
        let params = vec![json!(name)];
        self.call("createclaim", &params).await
    }

    //TODO check return type.
    pub async fn send_claim(&self, name: &str) -> Result<()> {
        let params = vec![json!(name)];
        self.call("sendclaim", &params).await
    }

    //TODO check the return type on this.
    pub async fn send_raw_claim(&self, claim: &str) -> Result<()> {
        let params = vec![json!(claim)];
        self.call("sendrawclaim", &params).await
    }

    pub async fn grind_name(&self, length: u32) -> Result<String> {
        let params = vec![json!(length)];
        self.call("grindname", &params).await
    }
}
