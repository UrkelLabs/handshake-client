use jsonrpc::error::Error;

use crate::client::HSClient;

use serde_json;
use serde_json::json;

impl HSClient {
    pub fn pruneblockchain(&self) -> Result<(), Error> {
        self.call("pruneblockchain", &[])
    }

    pub fn invalidateblock(&self, blockhash: String) -> Result<(), Error> {
        self.call("invalidateblock", &[json!(blockhash)])
    }

    pub fn reconsiderblock(&self, blockhash: String) -> Result<(), Error> {
        self.call("reconsiderblock", &[json!(blockhash)])
    }

}
