use crate::responses;

use jsonrpc::error::Error;

use crate::client::RPCClient;

use serde_json;
use serde_json::json;

impl RPCClient {

    //TODO check that the endpoint only parses result{} and not the error or id.
    pub fn get_name_info(&self, name: &str) -> Result<responses::NameInfo, Error> {
        self.call("getnameinfo", &[json!(name)])
    }

    //TODO finish output with enums
    pub fn get_names(&self) -> Result<Vec<responses::Name, Error> {
        self.call("getnames", &[])
    }

    pub fn get_name_by_hash(&self, name_hash: &str) -> Result<String, Error> {
        self.call("getnamebyhash", &[json!(name_hash)])
    }

    //TODO test return value.
    pub fn get_name_resource(&self, name: &str) -> Result<responses::NameResource, Error> {
        self.call("getnameresource", &[json!(name)])
    }

    pub fn get_name_proof(&self, name: &str) -> Result<responses::NameProof, Error> {
        self.call("getnameproof", &[json!(name)])
    }

    pub fn create_claim(&self, name: &str) -> Result<responses::CreateClaim, Error> {
        self.call("createclaim", &[json!(name)])
    }

    //TODO check return type.
    pub fn send_claim(&self, name: &str) -> Result<(), Error> {
        self.call("sendclaim", &[json!(name)])
    }

    //TODO check the return type on this.
    pub fn send_raw_claim(&self, claim: &str) -> Result<(), Error> {
        self.call("sendrawclaim", &[json!(claim)])
    }

    pub fn grind_name(&self, length: &u32) -> Result<String, Error> {
        self.call("grindname", &[json!(length)])
    }

}
